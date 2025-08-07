use substreams_ethereum::pb::eth::v2 as eth;

// Import the generated protobuf types
pub mod pb {
    pub mod farcaster_stats {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/farcaster_stats.v1.rs"));
        }
    }
}

use pb::farcaster_stats::v1::*;

#[substreams::handlers::map]
pub fn map_farcaster_events(block: eth::Block) -> Result<FarcasterEvents, substreams::errors::Error> {
    let mut events = FarcasterEvents::default();
    let mut total_events = 0;

    // Farcaster contract addresses on Optimism
    const ID_REGISTRY: &str = "0x00000000fc6c5f01fc30151999387bb99a9f489b";
    const STORAGE_REGISTRY: &str = "0x00000000fcce7f938e7ae6d3c335bd6a1a7c593d";
    const KEY_REGISTRY: &str = "0x00000000fc1237824fb747abde0ff18990e59b7e";

    // Count events from each contract
    for log in block.logs() {
        let address = format!("0x{}", hex::encode(&log.address()));
        
        match address.as_str() {
            ID_REGISTRY => {
                // Count user registrations
                let registration = UserRegistration {
                    farcaster_id: format!("0x{}", hex::encode(&log.topics()[0])),
                    ethereum_address: address,
                    block_number: block.number,
                    transaction_hash: format!("0x{}", hex::encode(&log.receipt.transaction.hash)),
                };
                events.user_registrations.push(registration);
                total_events += 1;
            }
            STORAGE_REGISTRY => {
                // Count storage events
                let storage = StorageEvent {
                    farcaster_id: format!("0x{}", hex::encode(&log.topics()[0])),
                    storage_units: log.data().len() as u32,
                    event_type: "storage".to_string(),
                    block_number: block.number,
                    transaction_hash: format!("0x{}", hex::encode(&log.receipt.transaction.hash)),
                };
                events.storage_events.push(storage);
                total_events += 1;
            }
            KEY_REGISTRY => {
                // Count key changes
                let key_change = KeyChange {
                    farcaster_id: format!("0x{}", hex::encode(&log.topics()[0])),
                    key_type: "signing".to_string(),
                    block_number: block.number,
                    transaction_hash: format!("0x{}", hex::encode(&log.receipt.transaction.hash)),
                };
                events.key_changes.push(key_change);
                total_events += 1;
            }
            _ => {}
        }
    }

    events.total_events = total_events;
    Ok(events)
} 
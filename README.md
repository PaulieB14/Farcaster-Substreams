# Farcaster Stats Substream

A Substreams module that tracks Farcaster protocol events on Optimism, including user registrations, storage rentals, and key changes.

## Overview

This substream monitors the three main Farcaster contracts on Optimism:
- **ID Registry** (`0x00000000fc6c5f01fc30151999387bb99a9f489b`) - User registrations
- **Storage Registry** (`0x00000000fcce7f938e7ae6d3c335bd6a1a7c593d`) - Storage rentals
- **Key Registry** (`0x00000000fc1237824fb747abde0ff18990e59b7e`) - Key management

## Features

- **Real-time event tracking** from Farcaster contracts
- **Protobuf-based data model** for efficient serialization
- **WASM compilation** for high-performance execution
- **Optimism network support** for mainnet data

## Project Structure

```
farcaster-stats/
├── src/
│   ├── lib.rs              # Main substream logic
│   └── pb/                 # Generated protobuf code
├── proto/
│   └── farcaster_stats.proto  # Protobuf definitions
├── substreams.yaml         # Substreams manifest
├── Cargo.toml             # Rust dependencies
└── README.md              # This file
```

## Data Model

The substream outputs `FarcasterEvents` containing:

- **UserRegistration**: New user registrations with FID and address
- **StorageEvent**: Storage rental events with units and type
- **KeyChange**: Key addition/removal events
- **total_events**: Count of all events in the block

## Installation

1. **Install Substreams CLI**:
   ```bash
   curl -sSfL https://raw.githubusercontent.com/streamingfast/substreams/master/install.sh | bash
   ```

2. **Install Rust and WASM target**:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup target add wasm32-unknown-unknown
   ```

3. **Clone and build**:
   ```bash
   git clone <repository-url>
   cd farcaster-stats
   cargo build --target wasm32-unknown-unknown --release
   ```

## Usage

### Generate Protobuf Code

```bash
substreams protogen substreams.yaml --exclude-paths="sf/ethereum,sf/substreams,google"
```

### Build the Substream

```bash
cargo build --target wasm32-unknown-unknown --release
```

### Run Locally

```bash
substreams run map_farcaster_events substreams.yaml
```

### Run on Optimism

```bash
substreams run map_farcaster_events substreams.yaml \
  --endpoint https://optimism-mainnet.substreams.streamingfast.io \
  --start-block 1000000
```

## Testing

### Local Testing

1. **Test with sample data**:
   ```bash
   # Create a test block with Farcaster events
   substreams run map_farcaster_events substreams.yaml --start-block 1000000
   ```

2. **Validate output format**:
   ```bash
   # Check that protobuf output is valid
   substreams run map_farcaster_events substreams.yaml --output debug
   ```

### Integration Testing

Test with real Optimism data:

```bash
# Run on recent blocks
substreams run map_farcaster_events substreams.yaml \
  --endpoint https://optimism-mainnet.substreams.streamingfast.io \
  --start-block $(($(date +%s) - 3600)) \
  --stop-block +10
```

## Deployment

### Deploy to Substreams

1. **Package the substream**:
   ```bash
   substreams pack substreams.yaml
   ```

2. **Deploy to Substreams**:
   ```bash
   substreams deploy farcaster_stats substreams.yaml
   ```

### Deploy to StreamingFast

1. **Create a package**:
   ```bash
   substreams pack substreams.yaml --output farcaster_stats.spkg
   ```

2. **Upload to StreamingFast**:
   ```bash
   # Use StreamingFast dashboard or API
   curl -X POST https://api.streamingfast.io/substreams \
     -H "Authorization: Bearer YOUR_API_KEY" \
     -F "package=@farcaster_stats.spkg"
   ```

## Configuration

### Environment Variables

- `SUBSTREAMS_ENDPOINT`: Substreams endpoint URL
- `SUBSTREAMS_API_KEY`: API key for authenticated endpoints
- `START_BLOCK`: Starting block number for processing

### Manifest Configuration

The `substreams.yaml` file configures:
- **Module inputs**: Ethereum blocks from Optimism
- **Module outputs**: Farcaster events in protobuf format
- **Binary**: WASM-compiled Rust code

## Monitoring

### Metrics to Track

- **Event counts**: Total events processed per block
- **Registration rate**: New user registrations over time
- **Storage activity**: Storage rental frequency
- **Key changes**: Key management activity

### Logs

Monitor substream execution:
```bash
substreams run map_farcaster_events substreams.yaml --log-level debug
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## License

MIT License - see LICENSE file for details

## Support

- **Documentation**: [Substreams Docs](https://substreams.streamingfast.io/)
- **Community**: [Discord](https://discord.gg/streamingfast)
- **Issues**: GitHub Issues

## Roadmap

- [ ] Add more sophisticated event parsing
- [ ] Support for additional Farcaster contracts
- [ ] Historical data analysis
- [ ] Real-time dashboard integration
- [ ] Multi-chain support (Ethereum mainnet) 
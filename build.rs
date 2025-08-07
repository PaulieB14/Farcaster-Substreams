fn main() {
    tonic_build::configure()
        .build_server(false)
        .compile(
            &["farcaster_stats.proto"],
            &["."],
        )
        .unwrap();
} 
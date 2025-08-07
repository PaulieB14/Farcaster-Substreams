#!/bin/bash

echo "🚀 Deploying Farcaster Stats Substream..."

# Build the project
echo "📦 Building WASM binary..."
cargo build --target wasm32-unknown-unknown --release

# Generate protobuf code
echo "🔧 Generating protobuf bindings..."
substreams protogen substreams.yaml --exclude-paths="sf/ethereum,sf/substreams,google"

# Package the substream
echo "📦 Packaging substream..."
substreams pack substreams.yaml -o farcaster_stats.spkg

# Check if package was created
if [ -f "farcaster_stats.spkg" ]; then
    echo "✅ Package created successfully!"
    echo "📊 Package size: $(du -h farcaster_stats.spkg | cut -f1)"
    echo ""
    echo "🎉 Deployment package ready!"
    echo ""
    echo "Next steps:"
    echo "1. Upload farcaster_stats.spkg to your Substreams provider"
    echo "2. Deploy using: substreams deploy farcaster_stats substreams.yaml"
    echo "3. Or use the StreamingFast dashboard to upload the .spkg file"
    echo ""
    echo "To test locally:"
    echo "substreams run substreams.yaml map_farcaster_events -s <block_number> -t +10"
else
    echo "❌ Package creation failed!"
    exit 1
fi 
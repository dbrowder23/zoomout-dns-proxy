#!/bin/bash

set -e

echo "Installing ZoomOut DNS Proxy..."

# Check if Rust is installed
if ! command -v cargo &> /dev/null
then
    echo "Rust not found. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env
else
    echo "Rust is already installed."
fi

# Confirm we're inside the zoomout-dns-proxy repo
if [ ! -f Cargo.toml ]; then
    echo "Error: You must run this script inside the zoomout-dns-proxy repository root."
    exit 1
fi

# Build the project
echo "Building ZoomOut..."
cargo build --release

# Install the binary
echo "Installing zoomout binary to /usr/local/bin..."
sudo cp ./target/release/zoomout /usr/local/bin/zoomout

# Create the blacklist if it doesn't exist
if [ ! -f blacklist.txt ]; then
    echo "Creating initial blacklist.txt..."
    cat << EOF > blacklist.txt
dGVsZW1ldHJ5Lnplb20udXM=
bWV0cmljcy56b29tLnVz
ZGF0YS56b29tLnVz
bG9ncy56b29tLnVz
cmVwb3J0cy56b29tLnVz
c3RhdHMuem9vbS51cw==
EOF
else
    echo "blacklist.txt already exists, skipping creation."
fi

echo ""
echo "ZoomOut DNS Proxy installed successfully!"
echo ""
echo "Next Steps:"
echo " - Set your DNS server to 127.0.0.1"
echo "   Example on MacOS:"
echo "     sudo networksetup -setdnsservers Wi-Fi 127.0.0.1"
echo ""
echo " - Start the DNS proxy by running:"
echo "     zoomout"
echo ""
echo "Enjoy your enhanced privacy!"

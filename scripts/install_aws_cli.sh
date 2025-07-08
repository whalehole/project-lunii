#!/bin/bash
set -e

# uninstall preinstalled aws cli
sudo yum remove awscli || true

sudo apt-get update
sudo apt-get install -y curl unzip

# install aws cli based on cpu architecture
ARCH=$(uname -m)

if [[ "$ARCH" == "x86_64" || "$ARCH" == "amd64" ]]; then
    echo "This is x86_64/amd64"
    curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip"
    unzip awscliv2.zip
    sudo ./aws/install
elif [[ "$ARCH" == "aarch64" || "$ARCH" == "arm64" ]]; then
    echo "This is ARM (aarch64/arm64)"
    curl "https://awscli.amazonaws.com/awscli-exe-linux-aarch64.zip" -o "awscliv2.zip"
    unzip awscliv2.zip
    sudo ./aws/install
else
    echo "Unknown architecture: $ARCH"
    exit 1
fi

# clean up
rm -rf aws awscliv2.zip

echo "aws cli installation complete:"
aws --version
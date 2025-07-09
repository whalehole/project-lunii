#!/bin/bash
set -e

# use package manager if present
if command -v apt-get >/dev/null; then
    PM="apt-get"
    sudo apt-get update
elif command -v yum >/dev/null; then
    PM="yum"
    sudo yum update -y
else
    echo "No supported package manager found (apt-get, yum)."
    exit 1
fi

# uninstall preinstalled yum version of aws cli
sudo "$PM" remove -y awscli || true

# install curl and unzip if not installed
if ! command -v curl >/dev/null; then
    sudo "$PM" install -y curl
fi
if ! command -v unzip >/dev/null; then
    sudo "$PM" install -y unzip
fi

# update aws cli if already installed
if command -v aws >/dev/null 2>&1; then
    echo "aws cli already installed. update to latest"
    sudo ./aws/install --bin-dir /usr/local/bin --install-dir /usr/local/aws-cli --update
else
    echo "aws cli not found. installing aws cli"

    # install aws cli based on cpu architecture
    ARCH=$(uname -m)

    if [[ "$ARCH" == "x86_64" || "$ARCH" == "amd64" ]]; then
        echo "This is x86_64/amd64"
        curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip"
        unzip awscliv2.zip
        sudo ./aws/install -i /usr/local/aws-cli -b /usr/local/bin
    elif [[ "$ARCH" == "aarch64" || "$ARCH" == "arm64" ]]; then
        echo "This is ARM (aarch64/arm64)"
        curl "https://awscli.amazonaws.com/awscli-exe-linux-aarch64.zip" -o "awscliv2.zip"
        unzip awscliv2.zip
        sudo ./aws/install -i /usr/local/aws-cli -b /usr/local/bin
    else
        echo "Unknown architecture: $ARCH"
        exit 1
    fi

    # clean up
    rm -rf aws awscliv2.zip

    echo "aws cli installation complete:"
    aws --version
fi
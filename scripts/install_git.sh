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

# install git
sudo "$PM" update
sudo "$PM" install -y git
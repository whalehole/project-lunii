#!/bin/bash
set -e

# use package manager if present
if command -v apt-get >/dev/null; then
    PM="apt-get"
    sudo apt-get update -y
    # uninstall all packages that may conflict with docker
    for pkg in docker.io docker-doc docker-compose docker-compose-v2 podman-docker containerd runc; do sudo "$PM" remove -y $pkg; done
    
    # install curl if not installed
    if ! command -v curl >/dev/null; then
        sudo "$PM" install -y curl
    fi

    # install tools needed to add repositories and verify signatures
    sudo "$PM" install -y ca-certificates gnupg # ca-certificates for HTTPS, gnupg for handling GPG keys (encryption/signing)
    # creates a directory /etc/apt/keyrings with correct permissions to store trusted GPG keys
    sudo install -m 0755 -d /etc/apt/keyrings

    # download docker’s GPG public key (used to verify packages from Docker)
    curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg
    sudo chmod a+r /etc/apt/keyrings/docker.gpg

    # add the repository to Apt sources:
    echo \
    "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu \
    $(. /etc/os-release && echo "$VERSION_CODENAME") stable" | \
    sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

    # install docker
    sudo apt-get install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
    sudo "$PM" update

    # start docker service
    sudo systemctl enable docker
    sudo systemctl start docker

    # verify docker engine installation
    sudo docker run hello-world
elif command -v yum >/dev/null; then
    # the user of the host server
    SSH_USER="$1"

    if [ -z "$SSH_USER" ]; then
        echo "$0 missing argument: SSH_USER"
        exit 1
    fi

    PM="yum"
    sudo yum update -y
    # uninstall all packages that may conflict with docker
    for pkg in docker.io docker-doc docker-compose docker-compose-v2 podman-docker containerd runc; do sudo "$PM" remove -y $pkg; done

    # install docker
    sudo yum -y install docker

    # start docker
    sudo systemctl start docker
    sudo systemctl enable docker

    # access docker commands for user in remote server
    sudo usermod -a -G docker "$SSH_USER"

    # verify docker engine installation
    sudo docker run hello-world
else
    echo "No supported package manager found (apt-get, yum)."
    exit 1
fi
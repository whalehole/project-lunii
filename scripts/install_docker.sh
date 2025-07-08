#!/bin/bash
set -e

# uninstall all packages that may conflict with docker
for pkg in docker.io docker-doc docker-compose docker-compose-v2 podman-docker containerd runc; do sudo apt-get remove $pkg; done

# update available packages
sudo apt-get update
# install tools needed to add repositories and verify signatures
sudo apt-get install -y ca-certificates curl gnupg # ca-certificates for HTTPS, curl for downloading files, gnupg for handling GPG keys (encryption/signing)
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
sudo apt-get update

# run docker service
sudo systemctl enable docker
sudo systemctl start docker

# verify docker engine installation
sudo docker run hello-world
#!/bin/bash
set -e

# remove existing AllowAgentForwarding setting
sudo sed -i '/^AllowAgentForwarding /d' /etc/ssh/sshd_config

# add the correct line
echo 'AllowAgentForwarding yes' | sudo tee -a /etc/ssh/sshd_config

# restart ssh service
if sudo systemctl restart sshd 2>/dev/null; then
    echo "sshd restarted"
elif sudo systemctl restart ssh 2>/dev/null; then
    echo "ssh service restarted"
else
    echo "could not restart SSH daemon automatically"
    exit 1
fi

echo "AllowAgentForwarding is now set to yes"

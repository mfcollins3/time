#!/usr/bin/env bash

set -e

echo "Installing ALSA developer libraries..."

# Update package lists
sudo apt-get update

# Install ALSA development libraries and headers
sudo apt-get install -y \
    libasound2-dev \
    alsa-utils \
    alsa-tools

echo "ALSA developer libraries installed successfully!"
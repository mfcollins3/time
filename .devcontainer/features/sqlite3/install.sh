#!/bin/bash
set -e

echo "Installing SQLite 3..."

# Update package list and install sqlite3
apt-get update
apt-get install -y sqlite3

# Verify installation
if command -v sqlite3 &> /dev/null; then
    echo "SQLite 3 installed successfully: $(sqlite3 --version)"
else
    echo "ERROR: SQLite 3 installation failed"
    exit 1
fi

# Clean up
apt-get clean
rm -rf /var/lib/apt/lists/*

echo "SQLite 3 feature installation complete"

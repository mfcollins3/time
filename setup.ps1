#!/usr/bin/env pwsh

# Exit on error
$ErrorActionPreference = "Stop"

# Install and configure Git LFS; download LFS objects
Write-Host "Installing Git LFS..."
git lfs install --skip-repo
if ($LASTEXITCODE -ne 0) {
    throw "Failed to install Git LFS"
}

Write-Host "Pulling Git LFS objects..."
git lfs pull
if ($LASTEXITCODE -ne 0) {
    throw "Failed to pull Git LFS objects"
}

# Install NPM dependencies
Write-Host "Installing NPM dependencies..."
npm ci
if ($LASTEXITCODE -ne 0) {
    throw "Failed to install NPM dependencies"
}

# Install Go dependencies
Write-Host "Downloading Go dependencies..."
go mod download
if ($LASTEXITCODE -ne 0) {
    throw "Failed to download Go dependencies"
}

Write-Host "Setup completed successfully!"

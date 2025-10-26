#!/usr/bin/env bash

set -e

# Install and configure Git LFS; download LFS objects
git lfs install --skip-repo
git lfs pull

# Install NPM dependencies
npm ci

# Install Go dependencies
go mod download

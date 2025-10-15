#!/usr/bin/env bash

set -e

# Pull LFS objects
git lfs pull

# Install NPM dependencies
npm ci

# Install Go dependencies
go mod download

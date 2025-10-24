#!/usr/bin/env bash

set -e

dlv debug --headless --listen=:2345 ./cmd/time

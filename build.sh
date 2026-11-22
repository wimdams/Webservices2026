#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/rust"

cargo build \
    --release \
    --target thumbv7em-none-eabihf \
    -p lcd
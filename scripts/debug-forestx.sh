#!/bin/bash

# Set "chatgpt.cliExecutable": "/Users/<USERNAME>/code/forestx/scripts/debug-forestx.sh" in VSCode settings to always get the 
# latest forestx-rs binary when debugging Forestx Extension.


set -euo pipefail

FORESTX_RS_DIR=$(realpath "$(dirname "$0")/../forestx-rs")
(cd "$FORESTX_RS_DIR" && cargo run --quiet --bin forestx -- "$@")
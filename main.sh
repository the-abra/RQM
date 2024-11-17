#!/bin/bash

# Sources
source .shsrc/logging.lib
source system.cfg

cd $workpath

[[ $1 == "setup" ]] && log.info "Setup for pht." && source /usr/share/RQM/.setup/pht-setup.sh && exit 0

cd core/
source $HOME/.cargo/env
cargo run
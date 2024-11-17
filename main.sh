#!/bin/bash

# Sources
source .shsrc/logging.lib
source system.cfg

cd $workpath

[[ $1 == "setup" ]] && log.info "Setup for pht." && bash /usr/share/RQM/.setup/pht-setup.sh

cd core/
cargo run
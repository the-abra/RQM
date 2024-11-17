#!/bin/bash

# Sources
source .shsrc/logging.lib
source system.cfg

cd $workpath

[[ $1 == "first" ]] && log.info "First installation for pht." && bash /usr/share/RQM/.setup/pht-setup.sh

cd core/
cargo run
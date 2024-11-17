#!/bin/bash

[[ $1 == "first" ]] && log.info "First installation for pht." && bash /usr/share/RQM/.setup/pht-setup.sh && phtmode=true

# Source Config
source /usr/share/RQM/system.cfg

# Working Path Set
cd $workpath

# Sources
source .shrc/*

cd core/
cargo run
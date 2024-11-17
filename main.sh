#!/bin/bash

# Sources
source .shrc/*
source /usr/share/RQM/system.cfg

cd $workpath

[[ $1 == "first" ]] && log.info "First installation for pht." && bash /usr/share/RQM/.setup/pht-setup.sh && phtmode=true

cd core/
cargo run
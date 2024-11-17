#!/bin/bash

# Sources
source .shsrc/*
source system.cfg

cd $workpath

[[ $1 == "first" ]] && log.info "First installation for pht." && bash /usr/share/RQM/.setup/pht-setup.sh && phtmode=true

cd core/
cargo run
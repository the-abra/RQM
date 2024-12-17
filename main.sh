#!/bin/bash

# Sources
source .shsrc/logging.lib
#source .shsrc/inipars.lib
#CONF_FILE="system.cfg"

hwd=$PWD

#cd $(inipars.get 'INFO' 'workpath')

[[ $1 == "setup" ]] && log.info "Setup for pht." && source /usr/share/RQM/.setup/pht-setup.sh && exit 0

cd core/
source $HOME/.cargo/env
cargo run -- $@   

cd $hwd
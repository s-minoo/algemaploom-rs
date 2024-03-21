#!/usr/bin/env bash

function divider() {
    local char=$1
    if [[ -z "${1+x}" ]]; then
       char="=" 
    fi
    echo "" 
    printf "${char}%.0s"  $(seq 1 63) 
    echo "" 
}


cargo build  2> /dev/null

PREV_STATUS=$? 


if [[ "$PREV_STATUS" -eq 1 ]]; then

    echo "Cargo build failed!"
    exit 1
fi
cat ./sample.shexml | ../target/debug/shexml_parser

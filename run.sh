#!/usr/bin/env bash
#


cargo b -r
./target/release/translator ./resources/multiple_tm.ttl

dot -Tpng output.dot > output.png

dot -Tpng output.dot.pretty > pretty.png

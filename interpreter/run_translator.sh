#!/usr/bin/env bash



cd ../
cargo b -r 
cd interpreter 
cargo b -r 
../target/release/translator ./resources/tests/multiple_tm_join.ttl
dot -Tpng ./output.dot.pretty > pretty.png
dot -Tpng ./output.dot > output.png

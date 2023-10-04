#!/usr/bin/env bash



cd ../
cargo b -r 
cd interpreter 
cargo b -r 
../target/release/translator $1
mkdir output/ > /dev/null  2>&1
dot -T svg ./output.dot.pretty > ./output/pretty.svg
dot -T svg ./output.dot > ./output/output.svg

mv output.dot output/ 
mv output.dot.pretty output/ 

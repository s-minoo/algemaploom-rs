#!/usr/bin/env bash



cd ../
cargo b -r 
cd interpreter 
cargo b -r 
../target/release/translator $1
mkdir output/ > /dev/null  2>&1
dot -Tpng ./output.dot.pretty > ./output/pretty.png
dot -Tpng ./output.dot > ./output/output.png

mv output.dot output/ 
mv output.dot.pretty output/ 

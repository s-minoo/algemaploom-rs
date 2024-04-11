#!/usr/bin/env bash

CSV_TESTFOLDER=./test_resources/csv-testcases/

cargo b -r
./target/release/translator folder $CSV_TESTFOLDER

for rml_folder in  ${CSV_TESTFOLDER}/*; do
    
    mapping_file=${rml_folder}/mapping_pretty.dot
    output_file=${rml_folder}/pretty.png
    dot -Tpng $mapping_file  > $output_file

done

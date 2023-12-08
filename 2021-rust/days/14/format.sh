#!/usr/bin/env bash

# This "script" can be used to clean up the puzzle inputs for easier handling

# spaces to commas (useful if sets of information are stored on a line):
# sed -z 's/ /,/g' data.txt > formatted.txt

sed -z 's/ -> /,/g' data.txt > formatted.txt
sed -z 's/ -> /,/g' small-data.txt > small-formatted.txt

# # single newline to comma, double newline to single newline
# # (useful if sets of data are stored on consecutive lines):
# sed -z 's|\n\n|¯\_(ツ)_/¯|g' data.txt > formatted.txt
# sed -i -z -e 's|\n|,|g' formatted.txt
# sed -i -z -e 's|¯\_(ツ)_/¯|\n|g' formatted.txt

#!/usr/bin/env bash

# This "script" can be used to clean up the puzzle inputs for easier handling

# lines starting with a space get shortened
sed -z -e 's/\n /\n/g' data.txt > formatted.txt
# double spaces become commas
sed -i -z -e 's/  /,/g' formatted.txt
# single spaces become commas
sed -i -z -e 's/ /,/g' formatted.txt
# single spaces become commas
sed -i -z -e 's/ /,/g' formatted.txt

# # single newline to comma, double newline to single newline
# # (useful if sets of data are stored on consecutive lines):
sed -i -z -e 's|\n\n|¯\_(ツ)_/¯|g' formatted.txt
sed -i -z -e 's|\n|,|g' formatted.txt
sed -i -z -e 's|¯\_(ツ)_/¯|\n|g' formatted.txt
sed -i -z -e 's|,$||g' formatted.txt

sed -z -e 's/\n /\n/g' small-data.txt > small-formatted.txt
# double spaces become commas
sed -i -z -e 's/  /,/g' small-formatted.txt
# single spaces become commas
sed -i -z -e 's/ /,/g' small-formatted.txt
# single spaces become commas
sed -i -z -e 's/ /,/g' small-formatted.txt

# # single newline to comma, double newline to single newline
# # (useful if sets of data are stored on consecutive lines):
sed -i -z -e 's|\n\n|¯\_(ツ)_/¯|g' small-formatted.txt
sed -i -z -e 's|\n|,|g' small-formatted.txt
sed -i -z -e 's|¯\_(ツ)_/¯|\n|g' small-formatted.txt
sed -i -z -e 's|,$||g' small-formatted.txt


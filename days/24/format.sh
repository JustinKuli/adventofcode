#!/usr/bin/env bash

sed -z 's/e/e,/g' data.txt > formatted.txt
sed -i -z -e 's/w/w,/g' formatted.txt

# single newline to comma, double newline to single, spaces to commas:
# sed -z 's|\n\n|¯\_(ツ)_/¯|g' data.txt > formatted.txt
# sed -i -z -e 's|\n|,|g' formatted.txt
# sed -i -z -e 's|¯\_(ツ)_/¯|\n|g' formatted.txt
# sed -i -z -e 's/ /,/g' formatted.txt

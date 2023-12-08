#!/usr/bin/env bash

sed 's/-/ /g' data.txt > formatted.txt
sed -i '' -e 's/: / /g' formatted.txt
sed -i '' -e 's/ /,/g' formatted.txt

#!/usr/bin/env bash

sed -z 's/\n\n/MAGIC/g' data.txt > formatted.txt
sed -i -z -e 's/\n/,/g' formatted.txt
sed -i -z -e 's/MAGIC/\n/g' formatted.txt
echo '' >> formatted.txt

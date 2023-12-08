#!/usr/bin/env bash

sed -z 's/\n\n/MAGIC/g' data.txt > formatted.txt
sed -i -z -e 's/\n/,/g' formatted.txt
sed -i -z -e 's/MAGIC/\n/g' formatted.txt
sed -i -e 's/ /,/g' formatted.txt

# The part one answer is easy with grep:
grep 'byr' formatted.txt | grep 'iyr' | grep 'eyr' | grep 'hgt' | grep 'hcl' | grep 'ecl' | grep 'pid' | wc -l

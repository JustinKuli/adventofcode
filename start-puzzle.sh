#!/usr/bin/env bash
# Note: run using `. start-puzzle.sh 01` so that the `cd` applies to your shell
set -euxo pipefail

day=$1
wd="./days/${day}"
saniday=$(echo ${day} | sed -z 's/^0//g') # remove leading 0 if present

cargo new --vcs "none" --name "aoc_2021_${day}" "${wd}"

curl "https://adventofcode.com/2021/day/${saniday}/input" \
  -H 'pragma: no-cache' \
  -H 'cache-control: no-cache' \
  -H 'user-agent: idk' \
  -H "$(cat cookie-header.txt)" \
  -H 'accept: text/html' > "${wd}/data.txt"

cp ./template/main.rs "${wd}/src/main.rs"
cp ./template/format.sh "${wd}/format.sh"
cat ./template/initial-deps.txt >> "${wd}/Cargo.toml"

cd "${wd}"
cargo build
code .
set +euxo pipefail

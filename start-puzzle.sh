#!/usr/bin/env bash
# Note: run using `. start-puzzle.sh 01` so that the `cd` applies to your shell
set -euxo pipefail

day=$1
wd="./days/${day}"

cargo new --vcs "none" --name "aoc_2021_${day}" "${wd}"
touch "${wd}/data.txt"
cp ./template/main.rs "${wd}/src/main.rs"
cp ./template/format.sh "${wd}/format.sh"
cat ./template/initial-deps.txt >> "${wd}/Cargo.toml"

cd "${wd}"
cargo build
code .
set +euxo pipefail

#!/bin/bash

for DAY in $(seq -f '%02g' 1 25); do
  export DAY
  lib="src/day${DAY}.rs"
  test -e "$lib" || envsubst < templates/lib.rs.txt > "$lib"

  for PART in 1 2; do
    export PART
    bin="src/bin/day${DAY}p${PART}.rs"
    test -e "$bin" || envsubst < templates/bin.rs.txt > "$bin"
  done
done

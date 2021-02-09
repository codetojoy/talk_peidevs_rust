#!/bin/bash 

for i in {1..40}
do
   echo ""
done

cargo clean -q
cargo run -q

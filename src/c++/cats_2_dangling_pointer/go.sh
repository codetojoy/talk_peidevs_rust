#!/bin/bash

set -e

for i in {1..40}
do
   echo ""
done

# run this in Docker
g++ -o example example.cpp
./example

echo "Ready."

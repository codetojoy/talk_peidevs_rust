#!/bin/bash

set -e

# run this in Docker
g++ example.cpp
./a.out

echo "Ready."

#!/bin/bash 

set -e 

if [ -z "$1" ]
  then
    echo "please specify 'which' arg"
    exit -1
fi

for i in {1..40}
do
   echo ""
done

WHICH=$1

cargo -q run $WHICH

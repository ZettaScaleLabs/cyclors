#!/bin/bash

if [ ! -s stderr.txt ]; then
    echo 'Command succeeded with no stderr output.'
    exit 0
else
    echo 'Command failed with errors:'
    cat stderr.txt
    exit 1
fi

#! /bin/bash

LIBRARY_PATH=lib:$LIBRARY_PATH LD_LIBRARY_PATH=lib:$LD_LIBRARY_PATH cargo run --release > /dev/null

#!/bin/bash

gcc -O2 -shared -o env.so -fPIC main.c -DUSE_DOUBLE
sudo cp env.so /usr/lib/csound/plugins64-6.0
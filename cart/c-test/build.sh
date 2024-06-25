#!/bin/bash

PROJECT="c-test"

set -e

rm -rf build
mkdir build
cd build

sdasz80 -p -l -o -s -g -ff crt0.rel ../crt0.s

for i in ../../kernel/*.c; do
    sdcc -mz80 --opt-code-size --fomit-frame-pointer --allow-unsafe-read --no-std-crt0 -c $i
done

sdcc -mz80 --opt-code-size --fomit-frame-pointer --allow-unsafe-read --no-std-crt0 --code-loc 0x0200 --data-loc 0x4000 -I ../../kernel/ -o $PROJECT.ihx *.rel ../main.c
objcopy -I ihex -O binary $PROJECT.ihx $PROJECT.bin

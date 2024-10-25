#!/bin/sh

cargo build
if [ $? -eq 0 ]; then
  target/debug/rust-http-add.exe
  exit $?
else 
  echo "ERROR: Build Failed" >&2
  exit 1
fi

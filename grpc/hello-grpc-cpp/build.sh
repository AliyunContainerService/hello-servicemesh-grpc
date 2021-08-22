#!/bin/bash
cd "$(
  cd "$(dirname "$0")" >/dev/null 2>&1
  pwd -P
)/" || exit
#
echo "cmake"
rm -rf build common/*.cc common/*.h
mkdir build
pushd build
cmake ..
echo "===="
echo "make"
make -j
popd
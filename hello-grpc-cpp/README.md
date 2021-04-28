## grpc c++ demo

https://grpc.io/docs/languages/cpp/quickstart/

### Setup
```bash
cmake --version
brew install autoconf automake libtool pkg-config

cd /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs
sudo ln -s MacOSX.sdk MacOSX11.1.sdk

export MY_INSTALL_DIR=$HOME/.local
mkdir -p $MY_INSTALL_DIR
export PATH="$MY_INSTALL_DIR/bin:$PATH"
```

### Build and install gRPC, Protocol Buffers, and Abseil 
```bash
cd grpc
git submodule update --init
mkdir -p cmake/build
pushd cmake/build
cmake -DgRPC_INSTALL=ON \
      -DgRPC_BUILD_TESTS=OFF \
      -DCMAKE_INSTALL_PREFIX=$MY_INSTALL_DIR \
      ../..
make -j
make install
popd
mkdir -p third_party/abseil-cpp/cmake/build
pushd third_party/abseil-cpp/cmake/build
cmake -DCMAKE_INSTALL_PREFIX=$MY_INSTALL_DIR \
      -DCMAKE_POSITION_INDEPENDENT_CODE=TRUE \
      ../..      
make -j
make install
popd
```
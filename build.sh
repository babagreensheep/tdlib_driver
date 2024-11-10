#!/bin//bash

# Check if OUT_DIR param provided
if [ -z $1 ]
then
    echo "ERROR: missing OUT_DIR"
    exit 1;
fi

# Debug
echo "build script running in $(pwd)"
echo "build script output to OUT_DIR: " $1

# # Check if source file has been cloned
if [ ! -d $OUT_DIR/.git ]
then
    echo "cloning tdlib"

    # Clone
    # git clone --single-branch --branch v1.8.0 -- git@github.com:tdlib/td.git $OUT_DIR
    git clone --single-branch --branch master -- git@github.com:tdlib/td.git $OUT_DIR
else
  exit 0;
fi

if [ ! -d $OUT_DIR/build ]
then
    echo "building tdlib"

    # Create docker image
    docker build --tag "local/td:1.8.0" ./build

    # Run build
    docker run --rm --user $UID -v $OUT_DIR:/project/source/td local/td:1.8.0

    # Remove docker image
    docker image rm local/td:1.8.0
else
  exit 0;
fi

if [ ! -d $OUT_DIR/bin ]
then
    echo "ERROR: missing binaries"
    exit 1;
else
    echo "build directory:"
    ls -la $OUT_DIR/bin
fi

echo "build script complete; exiting"
exit 0;

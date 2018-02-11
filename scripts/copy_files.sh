#!/bin/bash
set -e # Exit with nonzero exit code if anything fails

# Pull requests and commits to other branches shouldn't try to deploy, just build to verify
if [ "$TRAVIS_PULL_REQUEST" != "false" -o "$TRAVIS_BRANCH" != "master" ]; then
    echo "Skipping deploy; just doing a build."
    exit 0
fi

mkdirp -p out/wasm
cp -rv static/* out/
cp -v ./target/wasm32-unknown-unknown/release/synacor_vm.wasm out/wasm/synacor_vm.wasm

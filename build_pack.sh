#!/usr/bin/env zsh

name="$(basename "$(pwd)")"
arch="armv7-unknown-linux-musleabihf"
#arch="armv7-unknown-linux-gnueabihf"

echo "Compiling..."
cross build --target "$arch" --release || exit 1
echo "Done"

echo "Copying to directory..."
cp "./target/$arch/release/$name" "release/$name" || exit 1
echo "Done"

version="$(grep 'version' Cargo.toml | head -n 1 | cut -d\" -f2)"

cd release || exit 1
zip "../$name-release-$version.zip" \
./install.sh \
./"$name"

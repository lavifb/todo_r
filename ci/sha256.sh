#!/bin/sh

# Generate sha256 for a specific version of todor.
# modified from ripgrep https://github.com/BurntSushi/ripgrep

set -e

if [ $# != 1 ]; then
  echo "Usage: $(basename $0) version" >&2
  exit 1
fi
version="$1"

# Linux and Darwin builds.
for arch in x86_64; do
  for target in apple-darwin unknown-linux-gnu; do
    url="https://github.com/lavifb/todo_r/releases/download/v$version/todor-v$version-$arch-$target.tar.gz"
    sha=$(curl -sfSL "$url" | shasum -a 256)
    echo "$version-$arch-$target $sha"
  done
done

# Source.
for ext in zip tar.gz; do
  url="https://github.com/lavifb/todo_r/archive/v$version.$ext"
  sha=$(curl -sfSL "$url" | shasum -a 256)
  echo "source.$ext $sha"
done
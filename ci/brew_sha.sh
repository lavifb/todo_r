#!/bin/sh

set -e

TODOR_VERSION=$(git describe --abbrev=0 --tags)
MAC_URL="https://github.com/lavifb/todo_r/releases/download/${TODOR_VERSION}/todor-${TODOR_VERSION}-x86_64-apple-darwin.tar.gz"
MAC_SHA=$(curl -sfSL "$MAC_URL" | shasum -a 256)
echo "Mac SHA ${MAC_SHA}"

LIN_URL="https://github.com/lavifb/todo_r/releases/download/${TODOR_VERSION}/todor-${TODOR_VERSION}-x86_64-unknown-linux-gnu.tar.gz"
LIN_SHA=$(curl -sfSL "$LIN_URL" | shasum -a 256)
echo "Linux SHA ${LIN_SHA}"

cat >HomebrewFormula/todor.rb <<EOF
class Todor < Formula
  version '${TODOR_VERSION:1}'
  desc "Find all your TODO notes with one command!"
  homepage "https://github.com/lavifb/todo_r"

  if OS.mac?
      url "${MAC_URL}"
      sha256 "${MAC_SHA}"
  elsif OS.linux?
      url "${LIN_URL}"
      sha256 "${LIN_SHA}"
  end

  conflicts_with "todor"

  def install
    bin.install "todor"

    bash_completion.install "complete/todor.bash-completion"
    fish_completion.install "complete/todor.fish"
    zsh_completion.install "complete/_todor"
  end
end
EOF
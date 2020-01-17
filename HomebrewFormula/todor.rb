class Todor < Formula
  version '0.7.3'
  desc "Find all your TODO notes with one command!"
  homepage "https://github.com/lavifb/todo_r"

  if OS.mac?
      url "https://github.com/lavifb/todo_r/releases/download/v0.7.3/todor-v0.7.3-x86_64-apple-darwin.tar.gz"
      sha256 "8f64d6c85af4650420148dc015fc34ec2d8930cacd5ccdf6014686d0d59771c8"
  elsif OS.linux?
      url "https://github.com/lavifb/todo_r/releases/download/v0.7.3/todor-v0.7.3-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "6041d4c42f31e8c538c95f4d5e6094a0393e9d4d78bda940b0736ff247706ef0"
  end

  conflicts_with "todor"

  def install
    bin.install "todor"

    bash_completion.install "complete/todor.bash-completion"
    fish_completion.install "complete/todor.fish"
    zsh_completion.install "complete/_todor"
  end
end

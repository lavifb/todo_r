class Todor < Formula
  version '0.7.1'
  desc "Find all your TODO notes with one command!"
  homepage "https://github.com/lavifb/todo_r"

  if OS.mac?
      url "https://github.com/lavifb/todo_r/releases/download/v0.7.1/todor-v0.7.1-x86_64-apple-darwin.tar.gz"
      sha256 "70375c189ec899232efafee0449d3c21b05cc61eb81ad570f62a3485781b89ef"
  elsif OS.linux?
      url "https://github.com/lavifb/todo_r/releases/download/v0.7.1/todor-v0.7.1-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "c390be32d6e33cd06baf1b9c0253f904f4ecd6b413dba8ea3e32b73799cf8406"
  end

  conflicts_with "todor"

  def install
    bin.install "todor"

    bash_completion.install "complete/todor.bash-completion"
    fish_completion.install "complete/todor.fish"
    zsh_completion.install "complete/_todor"
  end
end

class TodoR < Formula
  version '0.5.1'
  desc "Find all your TODO notes with one command!"
  homepage "https://github.com/lavifb/todo_r"

  if OS.mac?
      url "https://github.com/lavifb/todo_r/releases/download/v#{version}/todor-v#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "f30862cecb73950a117ca47f9ca6bfe6978bfcc4084228606d6e49627967a1b6"
  elsif OS.linux?
      url "https://github.com/lavifb/todo_r/releases/download/v#{version}/todor-v#{version}-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "b9447ddc722ca97463c100a40932f2a761078e35eb1b6d86eea318bd5900af5f"
  end

  conflicts_with "todor"

  def install
    bin.install "todor"

    # bash_completion.install "complete/todor.bash"
    # fish_completion.install "complete/todor.fish"
    # zsh_completion.install "complete/_todor"
  end
end
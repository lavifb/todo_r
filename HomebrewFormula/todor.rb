class Todor < Formula
  version '0.7.2'
  desc "Find all your TODO notes with one command!"
  homepage "https://github.com/lavifb/todo_r"

  if OS.mac?
      url "https://github.com/lavifb/todo_r/releases/download/v0.7.2/todor-v0.7.2-x86_64-apple-darwin.tar.gz"
      sha256 "e33c3389fc9893c6b1caa83c73b9fb00ecf6a56ed076b34dcb4f2e7f1c3c089e"
  elsif OS.linux?
      url "https://github.com/lavifb/todo_r/releases/download/v0.7.2/todor-v0.7.2-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "fe64b859bc23ab34ac7a1849e32b3787a2fb399906dca672361c9e119ee5c717"
  end

  conflicts_with "todor"

  def install
    bin.install "todor"

    bash_completion.install "complete/todor.bash-completion"
    fish_completion.install "complete/todor.fish"
    zsh_completion.install "complete/_todor"
  end
end

class Todor < Formula
  version '0.6.0'
  desc "Find all your TODO notes with one command!"
  homepage "https://github.com/lavifb/todo_r"

  if OS.mac?
      url "https://github.com/lavifb/todo_r/releases/download/v#{version}/todor-v#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "3a43293c8576f2ac612fef2f28582f2cc93d7b473dab9cb03cb981a8f3fbc87e"
  elsif OS.linux?
      url "https://github.com/lavifb/todo_r/releases/download/v#{version}/todor-v#{version}-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "80bf5e63811432cb29927bc3b9051a4123601e0fb749a0382829d73c55650c55"
  end

  conflicts_with "todor"

  def install
    bin.install "todor"

    bash_completion.install "complete/todor.bash-completion"
    fish_completion.install "complete/todor.fish"
    zsh_completion.install "complete/_todor"
  end
end
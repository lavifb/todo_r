class Todor < Formula
  version '0.7.0'
  desc "Find all your TODO notes with one command!"
  homepage "https://github.com/lavifb/todo_r"

  if OS.mac?
      url "https://github.com/lavifb/todo_r/releases/download/v0.7.0/todor-v0.7.0-x86_64-apple-darwin.tar.gz"
      sha256 "43187106a9377bce87c07813af75346494faa5f4d54e0ea45c9d02da2ff3b46e"
  elsif OS.linux?
      url "https://github.com/lavifb/todo_r/releases/download/v0.7.0/todor-v0.7.0-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "22b08622f9b580ecade36775bcace9a009b10c8dcfefd611bdddf3987ed255a7"
  end

  conflicts_with "todor"

  def install
    bin.install "todor"

    bash_completion.install "complete/todor.bash-completion"
    fish_completion.install "complete/todor.fish"
    zsh_completion.install "complete/_todor"
  end
end

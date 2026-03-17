class Ppmm < Formula
  desc "Python Project Manager - fast CLI tool to create, manage, and deploy Python projects"
  homepage "https://github.com/Sumangal44/ppmm"
  version "1.1.5"
  license "MIT"

  # SHA256 checksums are generated automatically from release artifacts.
  # To update: run `sha256sum ppmm-macos-<arch>` on each binary after downloading
  # from https://github.com/Sumangal44/ppmm/releases and replace the values below.
  on_macos do
    on_arm do
      url "https://github.com/Sumangal44/ppmm/releases/download/v#{version}/ppmm-macos-arm64"
      sha256 "UPDATE_WITH_ACTUAL_SHA256_FOR_ARM64"
    end

    on_intel do
      url "https://github.com/Sumangal44/ppmm/releases/download/v#{version}/ppmm-macos-x64"
      sha256 "UPDATE_WITH_ACTUAL_SHA256_FOR_X64"
    end
  end

  def install
    bin.install stable.url.split("/").last => "ppmm"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/ppmm --version")
  end
end

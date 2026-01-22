class Cdrip < Formula
  desc "CLI to read/convert AIFF files to FLAC"
  homepage "https://github.com/DonovanMontoya/CD-Rip"
  url "https://github.com/DonovanMontoya/CD-Rip/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "REPLACE_WITH_TARBALL_SHA256"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    system "#{bin}/cdrip", "--help"
  end
end

let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> {
    overlays = [ moz_overlay ];
  };
  ruststable = (nixpkgs.latest.rustChannels.stable.rust.override {
    extensions = [ "rust-src" "rust-analysis" ];}
  );
in
  with nixpkgs;
  stdenv.mkDerivation {
    name = "rust";
    buildInputs = [ rustup ruststable ];
  }

# { pkgs ? import <nixpkgs> {} }:
#
# pkgs.mkShell {
#   buildInputs = with pkgs; [
#     cargo
#     # glibc
#     pkgconfig
#     openssl
#   ];
#
#   shellHook = ''
#     exec cargo build
#   '';
# }

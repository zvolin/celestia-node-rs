{ pkgs, ... }:

let
  # Swift from nixpkgs has many issues, so we instead install the official .tar.gz
  # for Debian and make wrappers for it.
  swift = pkgs.stdenv.mkDerivation {
    pname = "swift";
    version = "6.0.3";

    src = pkgs.fetchurl {
      url = "https://download.swift.org/swift-6.0.3-release/debian12/swift-6.0.3-RELEASE/swift-6.0.3-RELEASE-debian12.tar.gz";
      sha256 = "sha256-b3ggPksAN8js+uiQlkM+sO8ZjSCuSy6/rfPAQgnSzeI=";
    };

    nativeBuildInputs = [ pkgs.makeWrapper ];
    buildInputs = [ pkgs.ncurses6 ];

    installPhase = ''
      mkdir -p "$out/opt/swift"
      cp -ra * "$out/opt/swift"

      for x in "$out/opt/swift/usr/bin"/*; do
        if [[ -f "$x" && -x "$x" ]]; then
          patchelf "$x" --add-rpath "${pkgs.ncurses6}/lib" || continue
        fi
      done

      mkdir -p "$out/bin"

      for x in "$out/opt/swift/usr/bin"/*swift*; do
        if [[ -x "$x" ]]; then
          dest="$out/bin/''${x##*/}"
          makeWrapper "$x" "$dest" --prefix PATH : "$out/opt/swift/usr/bin"
        fi
      done
    '';
  };
in
{
  packages = [
    pkgs.protobuf
    pkgs.wasm-pack
    swift
  ];

  cachix.enable = false;
  dotenv.enable = true;

  languages.rust = {
    enable = true;
    channel = "stable";
    # Extra targets other than the native
    targets = [ "wasm32-unknown-unknown" ];
  };

  languages.javascript = {
    enable = true;
    npm.enable = true;
  };

  languages.kotlin.enable = true;
}

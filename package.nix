{
  stdenv,
  rustPlatform,
  libsodium,
  pkg-config,
  lib,
}: let
  inherit (lib.sources) sourceByRegex;
  src = sourceByRegex ./. ["Cargo.*" "(src)(/.*)?"];
in
  rustPlatform.buildRustPackage rec {
    pname = "tasmota-backup";
    version = "0.1.0";

    inherit src;

    cargoLock = {
      lockFile = ./Cargo.lock;

      outputHashes = {
        "tasmota-mqtt-client-0.1.0" = "sha256-Azs9F825oU4ME+KwJIniLHGzVEBHJJws3faJLdBYoAA=";
      };
    };
  }

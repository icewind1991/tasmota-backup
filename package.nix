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

    doCheck = false;

    cargoLock = {
      lockFile = ./Cargo.lock;

      outputHashes = {
        "tasmota-mqtt-client-0.1.0" = "sha256-ZdM2fCH6NXEUEbml9GKXy77hDL3VnUQ2c1WZH6kDLZQ=";
      };
    };
  }

{ stdenv
, rustPlatform
, libsodium
, pkg-config
, lib
,
}:
let
  inherit (lib.sources) sourceByRegex;
  src = sourceByRegex ./. [ "Cargo.*" "(src)(/.*)?" ];
in
rustPlatform.buildRustPackage rec {
  pname = "tasmota-backup";
  version = "0.1.0";

  inherit src;

  doCheck = false;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };
}

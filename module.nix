{
  config,
  lib,
  pkgs,
  ...
}:
with lib; let
  format = pkgs.formats.toml {};
  configFile = format.generate "tasmota-backup.toml" {
    output.target = cfg.outputPath;
    mqtt = {
      inherit (cfg.mqtt) hostname port username;
      "password-file" = cfg.mqtt.passwordFile;
    };
    device."password-file" = cfg.devicePasswordFile;
  };
  cfg = config.services.tasmota-backup;
in {
  options.services.tasmota-backup = {
    enable = mkEnableOption "Log archiver";

    outputPath = mkOption {
      type = types.str;
      description = "Directory to save the backups into";
    };

    mqtt = mkOption {
      type = types.submodule {
        options = {
          hostname = mkOption {
            type = types.str;
            description = "MQTT hostname";
          };
          port = mkOption {
            type = types.port;
            default = 1883;
            description = "MQTT port";
          };
          username = mkOption {
            type = types.str;
            description = "MQTT username";
          };
          passwordFile = mkOption {
            type = types.str;
            description = "File containing the MQTT password";
          };
        };
      };
      description = "MQTT options";
    };

    devicePasswordFile = mkOption {
      type = types.str;
      description = "File containing the device password";
    };

    package = mkOption {
      type = types.package;
      defaultText = literalExpression "pkgs.tasproxy";
      description = "package to use";
    };
  };

  config = mkIf cfg.enable {
    systemd.services."tasmota-backup" = {
      wantedBy = ["multi-user.target"];

      serviceConfig = {
        ExecStart = "${cfg.package}/bin/tasmota-backup ${configFile}";
        Restart = "on-failure";
        DynamicUser = true;
        PrivateTmp = true;
        ProtectSystem = "strict";
        ProtectHome = true;
        NoNewPrivileges = true;
        PrivateDevices = true;
        ProtectClock = true;
        CapabilityBoundingSet = true;
        ProtectKernelLogs = true;
        ProtectControlGroups = true;
        SystemCallArchitectures = "native";
        ProtectKernelModules = true;
        RestrictNamespaces = true;
        MemoryDenyWriteExecute = true;
        ProtectHostname = true;
        LockPersonality = true;
        ProtectKernelTunables = true;
        RestrictAddressFamilies = "AF_INET AF_INET6";
        RestrictRealtime = true;
        ProtectProc = "noaccess";
        SystemCallFilter = ["@system-service" "~@resources" "~@privileged"];
        IPAddressDeny = "multicast";
        PrivateUsers = true;
        ProcSubset = "pid";
        RuntimeDirectory = "tasmota-backup";
        RestrictSUIDSGID = true;
      };
    };
  };
}

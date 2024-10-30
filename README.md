# tasmota-backup

Backup tasmota configuration over MQTT

## Usage

```bash
tasmota-backup /path/to/config.toml
```

config.toml:

```toml
[output]
target = "/path/to/output-directory"

[mqtt]
hostname = "mqtt.example.com"
port = 1883 # optional, defaults to 1883
username = "backup"
password = "mqtt-password"
# or load the password from an external file
# password-file = "/path/to/mqtt-password"

[device]
password = "device-password" # the device password is the MQTT password used by the tasmota device
# or load the password from an external file
# password-file = "/path/to/device-password"
```

A `.dmp` for every discovered device file will be written to the configured output directory.

The output files should be stable as long as the device configuration isn't changed and the backup program will not overwrite existing unchanged files.

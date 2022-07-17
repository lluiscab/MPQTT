# MPQTT

MPQTT is a linux program used to extract data from Voltronic / Axpert / MasterPower / Phocos inverters and sending it to MQTT for use in Home Assistant

## Installation

Download and install the latest [release](https://github.com/lluiscab/MPQTT/releases).

```bash
sudo dpkg -i
```

## Usage

Once installed, edit the configuration file and start the service

```bash
sudo nano /etc/mpqtt/config.yaml
sudo service mpqtt start
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License

[MIT](https://choosealicense.com/licenses/mit/)

## Troubleshooting

### Getting stuck in loop cycle
- via python:
    ```
    import serial
    ser = serial.Serial("/dev/ttyUSB0", 2400)
    ser.read()
    ```
    ctrl+c
- reboot: `sudo shutdown -r now`
- cycle power on inverter display unit
- use something like [wolffserial](https://crates.io/crates/wolffserial) to read for a bit then retry
    ```
    wolffserial watch /dev/ttyUSB0 2400
    ```
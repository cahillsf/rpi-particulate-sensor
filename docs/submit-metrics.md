# Submit Metrics to Datadog

Working out a few kinks here, still requires manual hookup of IO devices (mouse, keyboard, screen) to the Raspberry PI to login as root.


## As `root`

Once you have logged in, you can submit the metrics directly as root user by running:

```bash
ENV=prod DD_SITE="datadoghq.com" DD_API_KEY="$DATADOG_API_KEY" sps30-metrics
```

## Log in via SSH

For now, you need to update the password from the root user via manual connection to the device (e.g. `sudo passwd pi`).  Then you can SSH into the device from a workstation on the same WLAN network you configured via the build machine env vars.  In the [AWS Terraform example](../build/clouds/) these are set in the `wifi_*` inputs.

### On target device (as `root`)
1. Update the password for the `pi` user and grant I2C permissions.

    ```bash
    sudo passwd pi
    ...
    sudo chmod 666 /dev/i2c*
    ```

### On workstation
1. Find the IP of the Raspberry pi using a tool like [nmap](https://nmap.org/)  (e.g. `nmap -sn 192.168.1.0/24`)
2. Configure trust to the target machine on your workstation:

    ```bash
    ssh-keyscan -t rsa <DEVICE_IP> >> ~/.ssh/known_hosts
    ```
3. Login to the device `ssh pi@<TARGET_IP`
4.  Generate the metrics

    ```bash
    export $(grep -v '^#\|^$' /etc/environment | xargs) && \
     ENV=prod DD_SITE="datadoghq.com" DD_API_KEY="$DATADOG_API_KEY" sps30-metrics
    ```


## TODO
- fix EXTRA_USERS password settings and guidance
- autogrant I2C permissions to pi user
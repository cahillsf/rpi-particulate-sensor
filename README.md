# Custom RPI OS Image with Sensirion SPS30 Integration

[Starting point + inspiration](https://www.hackster.io/david-gherghita/air-quality-monitor-using-raspberry-pi-4-sps30-and-azure-03cb42#code)

Submit [Sensirion SPS30](https://sensirion.com/products/catalog/SPS30) air quality metrics to [Datadog](https://www.datadoghq.com/) via a [Raspberry Pi 4](https://www.raspberrypi.com/products/raspberry-pi-4-model-b/) running a custom [Yocto](https://www.yoctoproject.org/) based image.


AWS terraform examples for resources to build the OS image and push it up to S3 object storage.

## Docs

- [Project Overview](docs/overview.md)
- [Building the Custom OS Image](docs/os-build.md)
- [Mounting the Custom OS Image](docs/os-mount.md)
- [Connect the Sensor](doc/connect.md)
- [Submitting the Air Quality Metrics to Datadog](docs/submit-metrics.md)

## Hardware

- [Raspberry Pi 4 Model B 4GB](https://www.raspberrypi.com/products/raspberry-pi-4-model-b/)
- [Sensirion SPS30](https://www.digikey.com/en/products/detail/sensirion-ag/SPS30/9598990)
- [ZHR CABLE 5-PIN 1.5MM](https://www.digikey.com/en/products/detail/sparkfun-electronics/15108/13561759?so=92883604&content=productdetail_US&mkt_tok=MDI4LVNYSy01MDcAAAGaws6BPv4tzMYVsRdYLrUPDweOVTw-lf-GsdXkd74OCsACCU7ZRZBMclX26TlSv1eWgItLrN9MmO1f8txhI4XEqIeVGZZbuCvNdIdFA3n4Uw)
- Breadboard
- Jumper Wires
- Micro SD Card

## Software
- [AWS Account](https://aws.amazon.com/) : hosts the VM to build the custom OS + object storage to store the built image
- [Datadog Account](https://www.datadoghq.com/#): receives the air quality metrics for aggregation, visualization and monitoring
- [Rust programming language](https://www.rust-lang.org/)
- [Yocto Project](https://www.yoctoproject.org/)
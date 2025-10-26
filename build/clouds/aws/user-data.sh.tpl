#!/bin/bash


# yocto build reqs: https://docs.yoctoproject.org/ref-manual/system-requirements.html#ubuntu-and-debian
sudo apt-get update
sudo apt-get install -y build-essential chrpath cpio debianutils diffstat file gawk gcc git iputils-ping libacl1 liblz4-tool locales python3 python3-git python3-jinja2 python3-pexpect python3-pip python3-subunit socat texinfo unzip wget xz-utils zstd


curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip"
unzip awscliv2.zip
sudo ./aws/install

# probably not right
echo 'kernel.unprivileged_userns_clone=1' | sudo tee /etc/sysctl.d/99-yocto.conf
sudo systemctl stop apparmor
sudo sysctl --system


# add user for yocto build
sudo adduser --disabled-password --gecos "" yocto
echo "yocto:${yocto_user_password}" | sudo chpasswd
sudo usermod -aG sudo yocto

# Run the rest of the setup as the yocto user
sudo -u yocto bash << YOCTO_SETUP
# export secrets for runtime use
export PI_USER_PASSWORD_HASH=$(echo "${pi_user_password}" | openssl passwd -6 -stdin)
cd /home/yocto

# clone repos
git clone -b scarthgap --depth 1 https://git.yoctoproject.org/poky
git clone -b scarthgap --depth 1 https://github.com/agherzan/meta-raspberrypi.git
git clone -b scarthgap --depth 1 https://github.com/openembedded/meta-openembedded
git clone -b scarthgap --depth 1 https://github.com/cahillsf/meta-mylayer.git
git clone -b master --depth 1 https://github.com/rust-embedded/meta-rust-bin.git

# init build env
cd poky
source oe-init-build-env rpi-build

# add layers
bitbake-layers add-layer ../../meta-raspberrypi
bitbake-layers add-layer ../../meta-mylayer
bitbake-layers add-layer ../../meta-openembedded/meta-oe
bitbake-layers add-layer ../../meta-openembedded/meta-python
bitbake-layers add-layer ../../meta-openembedded/meta-networking
bitbake-layers add-layer ../../meta-rust-bin

# Create local.conf with our Raspberry Pi configuration
cat > conf/local.conf << EOF
# defaults from https://github.com/yoctoproject/poky/blob/scarthgap/meta-poky/conf/templates/default/local.conf.sample
DISTRO ?= "poky"
EXTRA_IMAGE_FEATURES ?= "debug-tweaks"
USER_CLASSES ?= "buildstats"
PATCHRESOLVE = "noop"
BB_DISKMON_DIRS ??= "\
    STOPTASKS,$${TMPDIR},1G,100K \
    STOPTASKS,$${DL_DIR},1G,100K \
    STOPTASKS,$${SSTATE_DIR},1G,100K \
    STOPTASKS,/tmp,100M,100K \
    HALT,$${TMPDIR},100M,1K \
    HALT,$${DL_DIR},100M,1K \
    HALT,$${SSTATE_DIR},100M,1K \
    HALT,/tmp,10M,1K"
PACKAGECONFIG:append:pn-qemu-system-native = " sdl"
CONF_VERSION = "2"

# options specific to our build
MACHINE = "raspberrypi4-64"
LICENSE_FLAGS_ACCEPTED += "synaptics-killswitch"
IMAGE_ROOTFS_EXTRA_SPACE = "8388608"
ENABLE_I2C = "1"
KERNEL_MODULE_AUTOLOAD:rpi += "i2c-dev i2c-bcm2708"
CORE_IMAGE_EXTRA_INSTALL += "bash nano tar zip curl ca-certificates ntp tzdata packagegroup-core-buildessential i2c-tools git startup-script rust-metrics dropbear rust rust-dev cargo env-vars"
IMAGE_CLASSES += "extrausers"
EXTRA_USERS_PARAMS = "\
  useradd -m -s /bin/bash pi; \
  usermod -p '${PI_USER_PASSWORD_HASH}' pi; \
"

# Note: site.conf is automatically included by Yocto's default configuration
# The variables from site.conf will be available without explicit require
EOF

# Hash the pi user password for secure storage

# Create site.conf with injected secrets
cat > conf/site.conf << EOF
# Secrets injected by Terraform during EC2 instance launch
DATADOG_API_KEY = "${datadog_api_key}"
%{ if github_token != "" }
GITHUB_TOKEN = "${github_token}"
%{ endif }
%{ if wifi_ssid != "" }
WIFI_SSID = "${wifi_ssid}"
%{ endif }
%{ if wifi_password != "" }
WIFI_PASSWORD = "${wifi_password}"
%{ endif }
EOF

# Set proper permissions
chmod 600 conf/site.conf

YOCTO_SETUP

## need to:
## - modify the local.conf
## - inject secrets (WIFI connection + DD API Key)
## - upload the image to S3


# Configuration files created, ready to build

## start the build
#nohup bitbake core-image-sato > build.log 2>&1 &
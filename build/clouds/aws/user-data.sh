#!/bin/bash


# yocto build reqs: https://docs.yoctoproject.org/ref-manual/system-requirements.html#ubuntu-and-debian
sudo apt-get update
sudo apt-get install build-essential chrpath cpio debianutils diffstat file gawk gcc git iputils-ping libacl1 liblz4-tool locales python3 python3-git python3-jinja2 python3-pexpect python3-pip python3-subunit socat texinfo unzip wget xz-utils zstd


curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip"
unzip awscliv2.zip
sudo ./aws/install

# probably not right
echo 'kernel.unprivileged_userns_clone=1' | sudo tee /etc/sysctl.d/99-yocto.conf
sudo systemctl stop apparmor
sudo sysctl --system


# add user for yocto build
sudo adduser yocto
sudo usermod -aG sudo yocto
su - yocto

# clone repos
git clone -b scarthgap --depth 1 https://git.yoctoproject.org/poky
git clone -b scarthgap --depth 1 https://github.com/agherzan/meta-raspberrypi.git
git clone -b scarthgap --depth 1 https://github.com/openembedded/meta-openembedded
git clone -b scarthgap --depth 1 https://github.com/cahillsf/meta-mylayer.git
git clone -b master --depth 1 https://github.com/rust-embedded/meta-rust-bin.git

# init build env
source oe-init-build-env rpi-build

# add layers
bitbake-layers add-layer ../../meta-raspberrypi
bitbake-layers add-layer ../../meta-mylayer
bitbake-layers add-layer ../../meta-openembedded/meta-oe
bitbake-layers add-layer ../../meta-openembedded/meta-python
bitbake-layers add-layer ../../meta-openembedded/meta-networking
bitbake-layers add-layer ../../meta-rust-bin

## need to:
## - modify the local.conf
## - inject secrets (WIFI connection + DD API Key)


## start the build
#nohup bitbake core-image-sato > build.log 2>&1 &
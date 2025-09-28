# Build the OS

I [forked and updated](https://github.com/cahillsf/meta-mylayer/tree/walnascar) the layer from the hackster post to make it compatible with the most recent (at time of writing) [Yocto](https://www.yoctoproject.org/) release `walnascar`.  To test and build the OS it I used two separate envs, a local one for quick development and then a chunkier machine on AWS to run the build.  Details linked below (I'm on a 2020 Mac AMD), did not "overoptimize" here so I'm sure there are improvements that could be made.

## Vagrant + Virtualbox

[config](../build/local/)

Darwin doesn't play well with the build process in my limited experience, so I used [vagrant + virtualbox](https://developer.hashicorp.com/vagrant/docs/providers/virtualbox) to setup a linux env and test the layer. 

Assumes you have both [poky](https://github.com/yoctoproject/poky) and [my-layer](https://github.com/cahillsf/meta-mylayer/tree/walnascar) available in your home dir and mounts them into the VM.


## AWS

[config](../build/clouds/)

Terraform config for an EC2 to run the build of the custom OS image.  The current config allows for using AWS instance connect over public internet to get a bash in the build VM.  

### Transfer to S3

(TODO: automate)

After the image is built, push it up to the S3 bucket:

```bash
IMAGE_FILE=$(ls /home/yocto/poky/rpi-build/tmp/deploy/images/raspberrypi4-64/ | grep 'wic.bz2')
aws s3 cp "/home/yocto/poky/rpi-build/tmp/deploy/images/raspberrypi4-64/$(ls /home/yocto/poky/rpi-build/tmp/deploy/images/raspberrypi4-64/ | grep 'wic.bz2')" s3://<TARGET_BUCKET>/images/
```
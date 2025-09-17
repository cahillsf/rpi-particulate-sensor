# Mount the OS on the SD

These are the steps to perform on your dev machine.  In my case, I'm on a Mac Intel and the process is pretty manual, using a USB C SD Card Reader.

Prereqs:
- [AWS CLI](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html) installed
- [Raspberry PI Imager](https://www.raspberrypi.com/software/) installed

1. Plug in your Micro SD to your machine to make the device available for mounting.

2. Pull down the compressed image:

```bash
aws s3 cp s3://<SOURCE_BUCKET>/images/<IMAGE_FILE> ./
```

3. Unzip and rename to expected fmt:

```bash
mv <IMAGE_FILE>.wic <IMAGE_FILE>.img
```

4. Mount the image on the disk using the [Raspberry PI Imager](https://www.raspberrypi.com/software/).
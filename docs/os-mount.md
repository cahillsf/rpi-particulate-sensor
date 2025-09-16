# Mount the OS on the SD

These are the steps to perform on your dev machine.  In my case, I'm on a Mac Intel and the process is pretty manual.

1. Pull down the compressed image:

```bash
aws s3 cp s3://<SOURCE_BUCKET>/images/<IMAGE_FILE> ./
```

2. Unzip and rename to expected fmt:

```bash
bunzip2 mv <IMAGE_FILE>.wic <IMAGE_FILE>.img
```

3. Mount the image on the disk using the [Raspberry PI Image]( https://www.raspberrypi.com/software/).
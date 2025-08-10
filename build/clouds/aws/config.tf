terraform {
  backend "s3" {
    bucket = "<YOUR_BUCKET>"
    key    = "build/clouds/poky-build"
    region = "us-east-1"
  }
}

provider "aws" {
  region = "us-east-1"
}

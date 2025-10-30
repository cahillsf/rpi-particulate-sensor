data "aws_region" "current" {}

data "aws_ec2_managed_prefix_list" "ec2_ic" {
  name = "com.amazonaws.${data.aws_region.current.name}.ec2-instance-connect"
}

data "aws_vpc" "selected" {
  id = var.vpc_id
}

data "aws_iam_policy_document" "assume_role" {
  statement {
    effect = "Allow"

    principals {
      type        = "Service"
      identifiers = ["ec2.amazonaws.com"]
    }

    actions = ["sts:AssumeRole"]
  }
}


data "aws_iam_policy_document" "image_builder_upload" {
  statement {
    effect = "Allow"

    actions = [
      "s3:PutObject",
      "s3:ListBucket",

    ]
    resources = [
      "arn:aws:s3:::${var.bucket_name}/*",
      "arn:aws:s3:::${var.bucket_name}"
    ]
  }
}
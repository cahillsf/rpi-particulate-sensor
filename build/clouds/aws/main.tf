resource "aws_security_group" "allow_tls" {
  name        = "allow_tls"
  description = "Allow TLS inbound traffic and all outbound traffic"
  vpc_id      = data.aws_vpc.selected.id

  tags = {
    Name = "allow_tls"
  }
}

resource "aws_vpc_security_group_ingress_rule" "allow_tls_ipv4" {
  security_group_id = aws_security_group.allow_tls.id
  cidr_ipv4         = data.aws_vpc.selected.cidr_block
  from_port         = 443
  ip_protocol       = "tcp"
  to_port           = 443
}

resource "aws_vpc_security_group_ingress_rule" "allow_instance_connect" {
  security_group_id = aws_security_group.allow_tls.id
  from_port         = 22
  ip_protocol       = "tcp"
  to_port           = 22
  prefix_list_id    = data.aws_ec2_managed_prefix_list.ec2_ic.id
}


resource "aws_vpc_security_group_egress_rule" "allow_all_traffic_ipv4" {
  security_group_id = aws_security_group.allow_tls.id
  cidr_ipv4         = "0.0.0.0/0"
  ip_protocol       = "-1" # all ports
}


resource "aws_instance" "yocto_builder" {
  # TODO probably just dont use the private AMI with prereqs + create a startup script

  ### small base

  # ami           = "ami-084568db4383264d4" # Ubuntu 22.04 LTS in us-east-1
  # instance_type = "t2.medium"


  # root_block_device {
  #   # volume_size = 100  # 100 GB disk
  #   volume_size = 20 
  #   volume_type = "gp3"
  # }

  ###  builder

  instance_type = "c6a.2xlarge"
  ami           = "<TODO>"

  root_block_device {
    volume_size = 150 # 100 GB disk
    volume_type = "gp3"
  }
  iam_instance_profile = aws_iam_instance_profile.image_builder.name
  user_data = templatefile("user-data.sh.tpl", {
    datadog_api_key      = var.datadog_api_key
    pi_user_password     = var.pi_user_password
    github_token         = var.github_token
    wifi_ssid            = var.wifi_ssid
    wifi_password        = var.wifi_password
    yocto_user_password  = var.yocto_user_password
  })
  # common
  vpc_security_group_ids = [aws_security_group.allow_tls.id]
  subnet_id              = var.subnet_id

  tags = {
    Name = "yocto-builder"
  }
}


resource "aws_ec2_instance_state" "yocto_builder" {
  instance_id = aws_instance.yocto_builder.id
  state       = "running"
}

resource "aws_iam_role" "image_builder" {
  name               = "image-builder"
  assume_role_policy = data.aws_iam_policy_document.assume_role.json
}

resource "aws_iam_role_policy_attachment" "image_builder" {
  role       = aws_iam_role.image_builder.name
  policy_arn = "arn:aws:iam::aws:policy/AmazonS3FullAccess"
}

resource "aws_iam_policy" "image_artifact_upload" {
  name = "image-artifact-upload"
  policy = data.aws_iam_policy_document.image_builder_upload.json
}

resource "aws_iam_instance_profile" "image_builder" {
  name = "image-builder"
  role = aws_iam_role.role.name
}



resource "aws_s3_bucket" "image_artifacts" {
  bucket = var.bucket_name
}

resource "aws_s3_bucket_public_access_block" "image_artifacts" {
  bucket = aws_s3_bucket.image_artifacts.id

  block_public_acls       = true
  block_public_policy     = true
  ignore_public_acls      = true
  restrict_public_buckets = true
}
variable "vpc_id" {
  type        = string
  description = "Target VPC for EC2 launch"
}

variable "subnet_id" {
  type        = string
  description = "Target public subnet for EC2 build machine launch"
}

variable "bucket_name" {
  type        = string
  description = "Name of the S3 bucket to store the built image"
}

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

variable "datadog_api_key" {
  type        = string
  description = "Datadog API key"
  sensitive   = true
}

variable "pi_user_password" {
  type        = string
  description = "Password for the pi user on the built Raspberry Pi image"
  sensitive   = true
}

variable "github_token" {
  type        = string
  description = "GitHub token for accessing private repositories during build"
  sensitive   = true
  default     = ""
}

variable "wifi_ssid" {
  type        = string
  description = "WiFi SSID for the Raspberry Pi"
  default     = ""
}

variable "wifi_password" {
  type        = string
  description = "WiFi password for the Raspberry Pi"
  sensitive   = true
  default     = ""
}

variable "yocto_user_password" {
  type        = string
  description = "Password for the yocto build user on the EC2 instance"
  sensitive   = true
  default     = "yocto123"
}
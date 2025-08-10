output "instance_id" {
  value       = aws_instance.yocto_builder.id
  description = "The EC2 instance ID of the launched image."
}
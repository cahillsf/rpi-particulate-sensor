resource "aws_iam_role" "image_builder" {
  name               = "image-builder"
  assume_role_policy = data.aws_iam_policy_document.assume_role.json
}

resource "aws_iam_policy" "image_artifact_upload" {
  name   = "image-artifact-upload"
  policy = data.aws_iam_policy_document.image_builder_upload.json
}

resource "aws_iam_instance_profile" "image_builder" {
  name = "image-builder"
  role = aws_iam_role.role.name
}

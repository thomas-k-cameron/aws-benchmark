// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ExportImageOutput {
    /// <p>A description of the image being exported.</p>
    #[doc(hidden)]
    pub description: std::option::Option<std::string::String>,
    /// <p>The disk image format for the exported image.</p>
    #[doc(hidden)]
    pub disk_image_format: std::option::Option<crate::types::DiskImageFormat>,
    /// <p>The ID of the export image task.</p>
    #[doc(hidden)]
    pub export_image_task_id: std::option::Option<std::string::String>,
    /// <p>The ID of the image.</p>
    #[doc(hidden)]
    pub image_id: std::option::Option<std::string::String>,
    /// <p>The name of the role that grants VM Import/Export permission to export images to your Amazon S3 bucket.</p>
    #[doc(hidden)]
    pub role_name: std::option::Option<std::string::String>,
    /// <p>The percent complete of the export image task.</p>
    #[doc(hidden)]
    pub progress: std::option::Option<std::string::String>,
    /// <p>Information about the destination Amazon S3 bucket.</p>
    #[doc(hidden)]
    pub s3_export_location: std::option::Option<crate::types::ExportTaskS3Location>,
    /// <p>The status of the export image task. The possible values are <code>active</code>, <code>completed</code>, <code>deleting</code>, and <code>deleted</code>.</p>
    #[doc(hidden)]
    pub status: std::option::Option<std::string::String>,
    /// <p>The status message for the export image task.</p>
    #[doc(hidden)]
    pub status_message: std::option::Option<std::string::String>,
    /// <p>Any tags assigned to the export image task.</p>
    #[doc(hidden)]
    pub tags: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    _request_id: Option<String>,
}
impl ExportImageOutput {
    /// <p>A description of the image being exported.</p>
    pub fn description(&self) -> std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The disk image format for the exported image.</p>
    pub fn disk_image_format(&self) -> std::option::Option<&crate::types::DiskImageFormat> {
        self.disk_image_format.as_ref()
    }
    /// <p>The ID of the export image task.</p>
    pub fn export_image_task_id(&self) -> std::option::Option<&str> {
        self.export_image_task_id.as_deref()
    }
    /// <p>The ID of the image.</p>
    pub fn image_id(&self) -> std::option::Option<&str> {
        self.image_id.as_deref()
    }
    /// <p>The name of the role that grants VM Import/Export permission to export images to your Amazon S3 bucket.</p>
    pub fn role_name(&self) -> std::option::Option<&str> {
        self.role_name.as_deref()
    }
    /// <p>The percent complete of the export image task.</p>
    pub fn progress(&self) -> std::option::Option<&str> {
        self.progress.as_deref()
    }
    /// <p>Information about the destination Amazon S3 bucket.</p>
    pub fn s3_export_location(&self) -> std::option::Option<&crate::types::ExportTaskS3Location> {
        self.s3_export_location.as_ref()
    }
    /// <p>The status of the export image task. The possible values are <code>active</code>, <code>completed</code>, <code>deleting</code>, and <code>deleted</code>.</p>
    pub fn status(&self) -> std::option::Option<&str> {
        self.status.as_deref()
    }
    /// <p>The status message for the export image task.</p>
    pub fn status_message(&self) -> std::option::Option<&str> {
        self.status_message.as_deref()
    }
    /// <p>Any tags assigned to the export image task.</p>
    pub fn tags(&self) -> std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
}
impl aws_http::request_id::RequestId for ExportImageOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ExportImageOutput {
    /// Creates a new builder-style object to manufacture [`ExportImageOutput`](crate::operation::export_image::ExportImageOutput).
    pub fn builder() -> crate::operation::export_image::builders::ExportImageOutputBuilder {
        crate::operation::export_image::builders::ExportImageOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::export_image::ExportImageOutput;
/// A builder for [`ExportImageOutput`](crate::operation::export_image::ExportImageOutput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
pub struct ExportImageOutputBuilder {
    pub(crate) description: std::option::Option<std::string::String>,
    pub(crate) disk_image_format: std::option::Option<crate::types::DiskImageFormat>,
    pub(crate) export_image_task_id: std::option::Option<std::string::String>,
    pub(crate) image_id: std::option::Option<std::string::String>,
    pub(crate) role_name: std::option::Option<std::string::String>,
    pub(crate) progress: std::option::Option<std::string::String>,
    pub(crate) s3_export_location: std::option::Option<crate::types::ExportTaskS3Location>,
    pub(crate) status: std::option::Option<std::string::String>,
    pub(crate) status_message: std::option::Option<std::string::String>,
    pub(crate) tags: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    _request_id: Option<String>,
}
impl ExportImageOutputBuilder {
    /// <p>A description of the image being exported.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.description = Some(input.into());
        self
    }
    /// <p>A description of the image being exported.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The disk image format for the exported image.</p>
    pub fn disk_image_format(mut self, input: crate::types::DiskImageFormat) -> Self {
        self.disk_image_format = Some(input);
        self
    }
    /// <p>The disk image format for the exported image.</p>
    pub fn set_disk_image_format(
        mut self,
        input: std::option::Option<crate::types::DiskImageFormat>,
    ) -> Self {
        self.disk_image_format = input;
        self
    }
    /// <p>The ID of the export image task.</p>
    pub fn export_image_task_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.export_image_task_id = Some(input.into());
        self
    }
    /// <p>The ID of the export image task.</p>
    pub fn set_export_image_task_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.export_image_task_id = input;
        self
    }
    /// <p>The ID of the image.</p>
    pub fn image_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.image_id = Some(input.into());
        self
    }
    /// <p>The ID of the image.</p>
    pub fn set_image_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.image_id = input;
        self
    }
    /// <p>The name of the role that grants VM Import/Export permission to export images to your Amazon S3 bucket.</p>
    pub fn role_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.role_name = Some(input.into());
        self
    }
    /// <p>The name of the role that grants VM Import/Export permission to export images to your Amazon S3 bucket.</p>
    pub fn set_role_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.role_name = input;
        self
    }
    /// <p>The percent complete of the export image task.</p>
    pub fn progress(mut self, input: impl Into<std::string::String>) -> Self {
        self.progress = Some(input.into());
        self
    }
    /// <p>The percent complete of the export image task.</p>
    pub fn set_progress(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.progress = input;
        self
    }
    /// <p>Information about the destination Amazon S3 bucket.</p>
    pub fn s3_export_location(mut self, input: crate::types::ExportTaskS3Location) -> Self {
        self.s3_export_location = Some(input);
        self
    }
    /// <p>Information about the destination Amazon S3 bucket.</p>
    pub fn set_s3_export_location(
        mut self,
        input: std::option::Option<crate::types::ExportTaskS3Location>,
    ) -> Self {
        self.s3_export_location = input;
        self
    }
    /// <p>The status of the export image task. The possible values are <code>active</code>, <code>completed</code>, <code>deleting</code>, and <code>deleted</code>.</p>
    pub fn status(mut self, input: impl Into<std::string::String>) -> Self {
        self.status = Some(input.into());
        self
    }
    /// <p>The status of the export image task. The possible values are <code>active</code>, <code>completed</code>, <code>deleting</code>, and <code>deleted</code>.</p>
    pub fn set_status(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.status = input;
        self
    }
    /// <p>The status message for the export image task.</p>
    pub fn status_message(mut self, input: impl Into<std::string::String>) -> Self {
        self.status_message = Some(input.into());
        self
    }
    /// <p>The status message for the export image task.</p>
    pub fn set_status_message(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.status_message = input;
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Any tags assigned to the export image task.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = Some(v);
        self
    }
    /// <p>Any tags assigned to the export image task.</p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ExportImageOutput`](crate::operation::export_image::ExportImageOutput).
    pub fn build(self) -> crate::operation::export_image::ExportImageOutput {
        crate::operation::export_image::ExportImageOutput {
            description: self.description,
            disk_image_format: self.disk_image_format,
            export_image_task_id: self.export_image_task_id,
            image_id: self.image_id,
            role_name: self.role_name,
            progress: self.progress,
            s3_export_location: self.s3_export_location,
            status: self.status,
            status_message: self.status_message,
            tags: self.tags,
            _request_id: self._request_id,
        }
    }
}
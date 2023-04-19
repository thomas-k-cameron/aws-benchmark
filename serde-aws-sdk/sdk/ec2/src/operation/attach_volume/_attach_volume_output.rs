// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes volume attachment details.</p>
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
pub struct AttachVolumeOutput {
    /// <p>The time stamp when the attachment initiated.</p>
    #[doc(hidden)]
    pub attach_time: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The device name.</p>
    #[doc(hidden)]
    pub device: std::option::Option<std::string::String>,
    /// <p>The ID of the instance.</p>
    #[doc(hidden)]
    pub instance_id: std::option::Option<std::string::String>,
    /// <p>The attachment state of the volume.</p>
    #[doc(hidden)]
    pub state: std::option::Option<crate::types::VolumeAttachmentState>,
    /// <p>The ID of the volume.</p>
    #[doc(hidden)]
    pub volume_id: std::option::Option<std::string::String>,
    /// <p>Indicates whether the EBS volume is deleted on instance termination.</p>
    #[doc(hidden)]
    pub delete_on_termination: std::option::Option<bool>,
    _request_id: Option<String>,
}
impl AttachVolumeOutput {
    /// <p>The time stamp when the attachment initiated.</p>
    pub fn attach_time(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.attach_time.as_ref()
    }
    /// <p>The device name.</p>
    pub fn device(&self) -> std::option::Option<&str> {
        self.device.as_deref()
    }
    /// <p>The ID of the instance.</p>
    pub fn instance_id(&self) -> std::option::Option<&str> {
        self.instance_id.as_deref()
    }
    /// <p>The attachment state of the volume.</p>
    pub fn state(&self) -> std::option::Option<&crate::types::VolumeAttachmentState> {
        self.state.as_ref()
    }
    /// <p>The ID of the volume.</p>
    pub fn volume_id(&self) -> std::option::Option<&str> {
        self.volume_id.as_deref()
    }
    /// <p>Indicates whether the EBS volume is deleted on instance termination.</p>
    pub fn delete_on_termination(&self) -> std::option::Option<bool> {
        self.delete_on_termination
    }
}
impl aws_http::request_id::RequestId for AttachVolumeOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl AttachVolumeOutput {
    /// Creates a new builder-style object to manufacture [`AttachVolumeOutput`](crate::operation::attach_volume::AttachVolumeOutput).
    pub fn builder() -> crate::operation::attach_volume::builders::AttachVolumeOutputBuilder {
        crate::operation::attach_volume::builders::AttachVolumeOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::attach_volume::AttachVolumeOutput;
/// A builder for [`AttachVolumeOutput`](crate::operation::attach_volume::AttachVolumeOutput).
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
pub struct AttachVolumeOutputBuilder {
    pub(crate) attach_time: std::option::Option<aws_smithy_types::DateTime>,
    pub(crate) device: std::option::Option<std::string::String>,
    pub(crate) instance_id: std::option::Option<std::string::String>,
    pub(crate) state: std::option::Option<crate::types::VolumeAttachmentState>,
    pub(crate) volume_id: std::option::Option<std::string::String>,
    pub(crate) delete_on_termination: std::option::Option<bool>,
    _request_id: Option<String>,
}
impl AttachVolumeOutputBuilder {
    /// <p>The time stamp when the attachment initiated.</p>
    pub fn attach_time(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.attach_time = Some(input);
        self
    }
    /// <p>The time stamp when the attachment initiated.</p>
    pub fn set_attach_time(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.attach_time = input;
        self
    }
    /// <p>The device name.</p>
    pub fn device(mut self, input: impl Into<std::string::String>) -> Self {
        self.device = Some(input.into());
        self
    }
    /// <p>The device name.</p>
    pub fn set_device(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.device = input;
        self
    }
    /// <p>The ID of the instance.</p>
    pub fn instance_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.instance_id = Some(input.into());
        self
    }
    /// <p>The ID of the instance.</p>
    pub fn set_instance_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.instance_id = input;
        self
    }
    /// <p>The attachment state of the volume.</p>
    pub fn state(mut self, input: crate::types::VolumeAttachmentState) -> Self {
        self.state = Some(input);
        self
    }
    /// <p>The attachment state of the volume.</p>
    pub fn set_state(
        mut self,
        input: std::option::Option<crate::types::VolumeAttachmentState>,
    ) -> Self {
        self.state = input;
        self
    }
    /// <p>The ID of the volume.</p>
    pub fn volume_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.volume_id = Some(input.into());
        self
    }
    /// <p>The ID of the volume.</p>
    pub fn set_volume_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.volume_id = input;
        self
    }
    /// <p>Indicates whether the EBS volume is deleted on instance termination.</p>
    pub fn delete_on_termination(mut self, input: bool) -> Self {
        self.delete_on_termination = Some(input);
        self
    }
    /// <p>Indicates whether the EBS volume is deleted on instance termination.</p>
    pub fn set_delete_on_termination(mut self, input: std::option::Option<bool>) -> Self {
        self.delete_on_termination = input;
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
    /// Consumes the builder and constructs a [`AttachVolumeOutput`](crate::operation::attach_volume::AttachVolumeOutput).
    pub fn build(self) -> crate::operation::attach_volume::AttachVolumeOutput {
        crate::operation::attach_volume::AttachVolumeOutput {
            attach_time: self.attach_time,
            device: self.device,
            instance_id: self.instance_id,
            state: self.state,
            volume_id: self.volume_id,
            delete_on_termination: self.delete_on_termination,
            _request_id: self._request_id,
        }
    }
}
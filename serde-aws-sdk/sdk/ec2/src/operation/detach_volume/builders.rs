// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::detach_volume::_detach_volume_output::DetachVolumeOutputBuilder;

pub use crate::operation::detach_volume::_detach_volume_input::DetachVolumeInputBuilder;

/// Fluent builder constructing a request to `DetachVolume`.
///
/// <p>Detaches an EBS volume from an instance. Make sure to unmount any file systems on the device within your operating system before detaching the volume. Failure to do so can result in the volume becoming stuck in the <code>busy</code> state while detaching. If this happens, detachment can be delayed indefinitely until you unmount the volume, force detachment, reboot the instance, or all three. If an EBS volume is the root device of an instance, it can't be detached while the instance is running. To detach the root volume, stop the instance first.</p>
/// <p>When a volume with an Amazon Web Services Marketplace product code is detached from an instance, the product code is no longer associated with the instance.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ebs-detaching-volume.html">Detach an Amazon EBS volume</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DetachVolumeFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::detach_volume::builders::DetachVolumeInputBuilder,
}
impl DetachVolumeFluentBuilder {
    /// Creates a new `DetachVolume`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::detach_volume::DetachVolume,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::detach_volume::DetachVolumeError>,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        Ok(crate::client::customize::CustomizableOperation { handle, operation })
    }

    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> std::result::Result<
        crate::operation::detach_volume::DetachVolumeOutput,
        aws_smithy_http::result::SdkError<crate::operation::detach_volume::DetachVolumeError>,
    > {
        let op = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    #[cfg(aws_sdk_unstable)]
    /// This function replaces the parameter with new one.
    /// It is useful when you want to replace the existing data with de-serialized data.
    /// ```compile_fail
    /// let result_future = async {
    ///     let deserialized_parameters: crate::operation::detach_volume::builders::DetachVolumeInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.detach_volume().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::detach_volume::builders::DetachVolumeInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The device name.</p>
    pub fn device(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.device(input.into());
        self
    }
    /// <p>The device name.</p>
    pub fn set_device(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_device(input);
        self
    }
    /// <p>Forces detachment if the previous detachment attempt did not occur cleanly (for example, logging into an instance, unmounting the volume, and detaching normally). This option can lead to data loss or a corrupted file system. Use this option only as a last resort to detach a volume from a failed instance. The instance won't have an opportunity to flush file system caches or file system metadata. If you use this option, you must perform file system check and repair procedures.</p>
    pub fn force(mut self, input: bool) -> Self {
        self.inner = self.inner.force(input);
        self
    }
    /// <p>Forces detachment if the previous detachment attempt did not occur cleanly (for example, logging into an instance, unmounting the volume, and detaching normally). This option can lead to data loss or a corrupted file system. Use this option only as a last resort to detach a volume from a failed instance. The instance won't have an opportunity to flush file system caches or file system metadata. If you use this option, you must perform file system check and repair procedures.</p>
    pub fn set_force(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_force(input);
        self
    }
    /// <p>The ID of the instance. If you are detaching a Multi-Attach enabled volume, you must specify an instance ID.</p>
    pub fn instance_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.instance_id(input.into());
        self
    }
    /// <p>The ID of the instance. If you are detaching a Multi-Attach enabled volume, you must specify an instance ID.</p>
    pub fn set_instance_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_instance_id(input);
        self
    }
    /// <p>The ID of the volume.</p>
    pub fn volume_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.volume_id(input.into());
        self
    }
    /// <p>The ID of the volume.</p>
    pub fn set_volume_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_volume_id(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
}
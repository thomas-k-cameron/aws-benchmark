// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ModifyVolumeAttribute`](crate::operation::modify_volume_attribute::builders::ModifyVolumeAttributeFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`auto_enable_io(AttributeBooleanValue)`](crate::operation::modify_volume_attribute::builders::ModifyVolumeAttributeFluentBuilder::auto_enable_io) / [`set_auto_enable_io(Option<AttributeBooleanValue>)`](crate::operation::modify_volume_attribute::builders::ModifyVolumeAttributeFluentBuilder::set_auto_enable_io): <p>Indicates whether the volume should be auto-enabled for I/O operations.</p>
    ///   - [`volume_id(impl Into<String>)`](crate::operation::modify_volume_attribute::builders::ModifyVolumeAttributeFluentBuilder::volume_id) / [`set_volume_id(Option<String>)`](crate::operation::modify_volume_attribute::builders::ModifyVolumeAttributeFluentBuilder::set_volume_id): <p>The ID of the volume.</p>
    ///   - [`dry_run(bool)`](crate::operation::modify_volume_attribute::builders::ModifyVolumeAttributeFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::modify_volume_attribute::builders::ModifyVolumeAttributeFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`ModifyVolumeAttributeOutput`](crate::operation::modify_volume_attribute::ModifyVolumeAttributeOutput)
    /// - On failure, responds with [`SdkError<ModifyVolumeAttributeError>`](crate::operation::modify_volume_attribute::ModifyVolumeAttributeError)
    pub fn modify_volume_attribute(
        &self,
    ) -> crate::operation::modify_volume_attribute::builders::ModifyVolumeAttributeFluentBuilder
    {
        crate::operation::modify_volume_attribute::builders::ModifyVolumeAttributeFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
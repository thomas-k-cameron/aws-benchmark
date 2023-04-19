// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateLaunchTemplateVersion`](crate::operation::create_launch_template_version::builders::CreateLaunchTemplateVersionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::create_launch_template_version::builders::CreateLaunchTemplateVersionFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::create_launch_template_version::builders::CreateLaunchTemplateVersionFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`client_token(impl Into<String>)`](crate::operation::create_launch_template_version::builders::CreateLaunchTemplateVersionFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::create_launch_template_version::builders::CreateLaunchTemplateVersionFluentBuilder::set_client_token): <p>Unique, case-sensitive identifier you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring idempotency</a>.</p>  <p>Constraint: Maximum 128 ASCII characters.</p>
    ///   - [`launch_template_id(impl Into<String>)`](crate::operation::create_launch_template_version::builders::CreateLaunchTemplateVersionFluentBuilder::launch_template_id) / [`set_launch_template_id(Option<String>)`](crate::operation::create_launch_template_version::builders::CreateLaunchTemplateVersionFluentBuilder::set_launch_template_id): <p>The ID of the launch template.</p>  <p>You must specify either the <code>LaunchTemplateId</code> or the <code>LaunchTemplateName</code>, but not both.</p>
    ///   - [`launch_template_name(impl Into<String>)`](crate::operation::create_launch_template_version::builders::CreateLaunchTemplateVersionFluentBuilder::launch_template_name) / [`set_launch_template_name(Option<String>)`](crate::operation::create_launch_template_version::builders::CreateLaunchTemplateVersionFluentBuilder::set_launch_template_name): <p>The name of the launch template.</p>  <p>You must specify the <code>LaunchTemplateName</code> or the <code>LaunchTemplateId</code>, but not both.</p>
    ///   - [`source_version(impl Into<String>)`](crate::operation::create_launch_template_version::builders::CreateLaunchTemplateVersionFluentBuilder::source_version) / [`set_source_version(Option<String>)`](crate::operation::create_launch_template_version::builders::CreateLaunchTemplateVersionFluentBuilder::set_source_version): <p>The version number of the launch template version on which to base the new version. The new version inherits the same launch parameters as the source version, except for parameters that you specify in <code>LaunchTemplateData</code>. Snapshots applied to the block device mapping are ignored when creating a new version unless they are explicitly included.</p>
    ///   - [`version_description(impl Into<String>)`](crate::operation::create_launch_template_version::builders::CreateLaunchTemplateVersionFluentBuilder::version_description) / [`set_version_description(Option<String>)`](crate::operation::create_launch_template_version::builders::CreateLaunchTemplateVersionFluentBuilder::set_version_description): <p>A description for the version of the launch template.</p>
    ///   - [`launch_template_data(RequestLaunchTemplateData)`](crate::operation::create_launch_template_version::builders::CreateLaunchTemplateVersionFluentBuilder::launch_template_data) / [`set_launch_template_data(Option<RequestLaunchTemplateData>)`](crate::operation::create_launch_template_version::builders::CreateLaunchTemplateVersionFluentBuilder::set_launch_template_data): <p>The information for the launch template.</p>
    ///   - [`resolve_alias(bool)`](crate::operation::create_launch_template_version::builders::CreateLaunchTemplateVersionFluentBuilder::resolve_alias) / [`set_resolve_alias(Option<bool>)`](crate::operation::create_launch_template_version::builders::CreateLaunchTemplateVersionFluentBuilder::set_resolve_alias): <p>If <code>true</code>, and if a Systems Manager parameter is specified for <code>ImageId</code>, the AMI ID is displayed in the response for <code>imageID</code>. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-launch-templates.html#use-an-ssm-parameter-instead-of-an-ami-id">Use a Systems Manager parameter instead of an AMI ID</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>  <p>Default: <code>false</code> </p>
    /// - On success, responds with [`CreateLaunchTemplateVersionOutput`](crate::operation::create_launch_template_version::CreateLaunchTemplateVersionOutput) with field(s):
    ///   - [`launch_template_version(Option<LaunchTemplateVersion>)`](crate::operation::create_launch_template_version::CreateLaunchTemplateVersionOutput::launch_template_version): <p>Information about the launch template version.</p>
    ///   - [`warning(Option<ValidationWarning>)`](crate::operation::create_launch_template_version::CreateLaunchTemplateVersionOutput::warning): <p>If the new version of the launch template contains parameters or parameter combinations that are not valid, an error code and an error message are returned for each issue that's found.</p>
    /// - On failure, responds with [`SdkError<CreateLaunchTemplateVersionError>`](crate::operation::create_launch_template_version::CreateLaunchTemplateVersionError)
    pub fn create_launch_template_version(&self) -> crate::operation::create_launch_template_version::builders::CreateLaunchTemplateVersionFluentBuilder{
        crate::operation::create_launch_template_version::builders::CreateLaunchTemplateVersionFluentBuilder::new(self.handle.clone())
    }
}
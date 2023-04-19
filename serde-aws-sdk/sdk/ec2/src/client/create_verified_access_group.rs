// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateVerifiedAccessGroup`](crate::operation::create_verified_access_group::builders::CreateVerifiedAccessGroupFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`verified_access_instance_id(impl Into<String>)`](crate::operation::create_verified_access_group::builders::CreateVerifiedAccessGroupFluentBuilder::verified_access_instance_id) / [`set_verified_access_instance_id(Option<String>)`](crate::operation::create_verified_access_group::builders::CreateVerifiedAccessGroupFluentBuilder::set_verified_access_instance_id): <p>The ID of the Amazon Web Services Verified Access instance.</p>
    ///   - [`description(impl Into<String>)`](crate::operation::create_verified_access_group::builders::CreateVerifiedAccessGroupFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_verified_access_group::builders::CreateVerifiedAccessGroupFluentBuilder::set_description): <p>A description for the Amazon Web Services Verified Access group.</p>
    ///   - [`policy_document(impl Into<String>)`](crate::operation::create_verified_access_group::builders::CreateVerifiedAccessGroupFluentBuilder::policy_document) / [`set_policy_document(Option<String>)`](crate::operation::create_verified_access_group::builders::CreateVerifiedAccessGroupFluentBuilder::set_policy_document): <p>The Amazon Web Services Verified Access policy document.</p>
    ///   - [`tag_specifications(Vec<TagSpecification>)`](crate::operation::create_verified_access_group::builders::CreateVerifiedAccessGroupFluentBuilder::tag_specifications) / [`set_tag_specifications(Option<Vec<TagSpecification>>)`](crate::operation::create_verified_access_group::builders::CreateVerifiedAccessGroupFluentBuilder::set_tag_specifications): <p>The tags to assign to the Amazon Web Services Verified Access group.</p>
    ///   - [`client_token(impl Into<String>)`](crate::operation::create_verified_access_group::builders::CreateVerifiedAccessGroupFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::create_verified_access_group::builders::CreateVerifiedAccessGroupFluentBuilder::set_client_token): <p>A unique, case-sensitive token that you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    ///   - [`dry_run(bool)`](crate::operation::create_verified_access_group::builders::CreateVerifiedAccessGroupFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::create_verified_access_group::builders::CreateVerifiedAccessGroupFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`CreateVerifiedAccessGroupOutput`](crate::operation::create_verified_access_group::CreateVerifiedAccessGroupOutput) with field(s):
    ///   - [`verified_access_group(Option<VerifiedAccessGroup>)`](crate::operation::create_verified_access_group::CreateVerifiedAccessGroupOutput::verified_access_group): <p>The ID of the Verified Access group.</p>
    /// - On failure, responds with [`SdkError<CreateVerifiedAccessGroupError>`](crate::operation::create_verified_access_group::CreateVerifiedAccessGroupError)
    pub fn create_verified_access_group(&self) -> crate::operation::create_verified_access_group::builders::CreateVerifiedAccessGroupFluentBuilder{
        crate::operation::create_verified_access_group::builders::CreateVerifiedAccessGroupFluentBuilder::new(self.handle.clone())
    }
}
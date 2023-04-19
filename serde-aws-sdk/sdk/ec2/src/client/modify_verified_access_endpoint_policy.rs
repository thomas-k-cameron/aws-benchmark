// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ModifyVerifiedAccessEndpointPolicy`](crate::operation::modify_verified_access_endpoint_policy::builders::ModifyVerifiedAccessEndpointPolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`verified_access_endpoint_id(impl Into<String>)`](crate::operation::modify_verified_access_endpoint_policy::builders::ModifyVerifiedAccessEndpointPolicyFluentBuilder::verified_access_endpoint_id) / [`set_verified_access_endpoint_id(Option<String>)`](crate::operation::modify_verified_access_endpoint_policy::builders::ModifyVerifiedAccessEndpointPolicyFluentBuilder::set_verified_access_endpoint_id): <p>The ID of the Amazon Web Services Verified Access endpoint.</p>
    ///   - [`policy_enabled(bool)`](crate::operation::modify_verified_access_endpoint_policy::builders::ModifyVerifiedAccessEndpointPolicyFluentBuilder::policy_enabled) / [`set_policy_enabled(Option<bool>)`](crate::operation::modify_verified_access_endpoint_policy::builders::ModifyVerifiedAccessEndpointPolicyFluentBuilder::set_policy_enabled): <p>The status of the Verified Access policy.</p>
    ///   - [`policy_document(impl Into<String>)`](crate::operation::modify_verified_access_endpoint_policy::builders::ModifyVerifiedAccessEndpointPolicyFluentBuilder::policy_document) / [`set_policy_document(Option<String>)`](crate::operation::modify_verified_access_endpoint_policy::builders::ModifyVerifiedAccessEndpointPolicyFluentBuilder::set_policy_document): <p>The Amazon Web Services Verified Access policy document.</p>
    ///   - [`client_token(impl Into<String>)`](crate::operation::modify_verified_access_endpoint_policy::builders::ModifyVerifiedAccessEndpointPolicyFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::modify_verified_access_endpoint_policy::builders::ModifyVerifiedAccessEndpointPolicyFluentBuilder::set_client_token): <p>A unique, case-sensitive token that you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    ///   - [`dry_run(bool)`](crate::operation::modify_verified_access_endpoint_policy::builders::ModifyVerifiedAccessEndpointPolicyFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::modify_verified_access_endpoint_policy::builders::ModifyVerifiedAccessEndpointPolicyFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`ModifyVerifiedAccessEndpointPolicyOutput`](crate::operation::modify_verified_access_endpoint_policy::ModifyVerifiedAccessEndpointPolicyOutput) with field(s):
    ///   - [`policy_enabled(Option<bool>)`](crate::operation::modify_verified_access_endpoint_policy::ModifyVerifiedAccessEndpointPolicyOutput::policy_enabled): <p>The status of the Verified Access policy.</p>
    ///   - [`policy_document(Option<String>)`](crate::operation::modify_verified_access_endpoint_policy::ModifyVerifiedAccessEndpointPolicyOutput::policy_document): <p>The Amazon Web Services Verified Access policy document.</p>
    /// - On failure, responds with [`SdkError<ModifyVerifiedAccessEndpointPolicyError>`](crate::operation::modify_verified_access_endpoint_policy::ModifyVerifiedAccessEndpointPolicyError)
    pub fn modify_verified_access_endpoint_policy(&self) -> crate::operation::modify_verified_access_endpoint_policy::builders::ModifyVerifiedAccessEndpointPolicyFluentBuilder{
        crate::operation::modify_verified_access_endpoint_policy::builders::ModifyVerifiedAccessEndpointPolicyFluentBuilder::new(self.handle.clone())
    }
}
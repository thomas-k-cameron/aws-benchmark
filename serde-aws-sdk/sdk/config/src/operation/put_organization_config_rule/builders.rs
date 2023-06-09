// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_organization_config_rule::_put_organization_config_rule_output::PutOrganizationConfigRuleOutputBuilder;

pub use crate::operation::put_organization_config_rule::_put_organization_config_rule_input::PutOrganizationConfigRuleInputBuilder;

/// Fluent builder constructing a request to `PutOrganizationConfigRule`.
///
/// <p>Adds or updates an Config rule for your entire organization to evaluate if your Amazon Web Services resources comply with your desired configurations. For information on how many organization Config rules you can have per account, see <a href="https://docs.aws.amazon.com/config/latest/developerguide/configlimits.html"> <b>Service Limits</b> </a> in the <i>Config Developer Guide</i>.</p>
/// <p> Only a management account and a delegated administrator can create or update an organization Config rule. When calling this API with a delegated administrator, you must ensure Organizations <code>ListDelegatedAdministrator</code> permissions are added. An organization can have up to 3 delegated administrators.</p>
/// <p>This API enables organization service access through the <code>EnableAWSServiceAccess</code> action and creates a service-linked role <code>AWSServiceRoleForConfigMultiAccountSetup</code> in the management or delegated administrator account of your organization. The service-linked role is created only when the role does not exist in the caller account. Config verifies the existence of role with <code>GetRole</code> action.</p>
/// <p>To use this API with delegated administrator, register a delegated administrator by calling Amazon Web Services Organization <code>register-delegated-administrator</code> for <code>config-multiaccountsetup.amazonaws.com</code>. </p>
/// <p>There are two types of rules: Config Custom Rules and Config Managed Rules. You can use <code>PutOrganizationConfigRule</code> to create both Config custom rules and Config managed rules.</p>
/// <p>Custom rules are rules that you can create using either Guard or Lambda functions. Guard (<a href="https://github.com/aws-cloudformation/cloudformation-guard">Guard GitHub Repository</a>) is a policy-as-code language that allows you to write policies that are enforced by Config Custom Policy rules. Lambda uses custom code that you upload to evaluate a custom rule. If you are adding a new Custom Lambda rule, you first need to create an Lambda function in the management account or a delegated administrator that the rule invokes to evaluate your resources. You also need to create an IAM role in the managed account that can be assumed by the Lambda function. When you use <code>PutOrganizationConfigRule</code> to add a Custom Lambda rule to Config, you must specify the Amazon Resource Name (ARN) that Lambda assigns to the function.</p>
/// <p>Managed rules are predefined, customizable rules created by Config. For a list of managed rules, see <a href="https://docs.aws.amazon.com/config/latest/developerguide/managed-rules-by-aws-config.html">List of Config Managed Rules</a>. If you are adding an Config managed rule, you must specify the rule's identifier for the <code>RuleIdentifier</code> key.</p> <note>
/// <p>Prerequisite: Ensure you call <code>EnableAllFeatures</code> API to enable all features in an organization.</p>
/// <p>Make sure to specify one of either <code>OrganizationCustomPolicyRuleMetadata</code> for Custom Policy rules, <code>OrganizationCustomRuleMetadata</code> for Custom Lambda rules, or <code>OrganizationManagedRuleMetadata</code> for managed rules.</p>
/// </note>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct PutOrganizationConfigRuleFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::put_organization_config_rule::builders::PutOrganizationConfigRuleInputBuilder
            }
impl PutOrganizationConfigRuleFluentBuilder {
    /// Creates a new `PutOrganizationConfigRule`.
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
            crate::operation::put_organization_config_rule::PutOrganizationConfigRule,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::put_organization_config_rule::PutOrganizationConfigRuleError,
        >,
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
        crate::operation::put_organization_config_rule::PutOrganizationConfigRuleOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::put_organization_config_rule::PutOrganizationConfigRuleError,
        >,
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
    ///     let deserialized_parameters: crate::operation::put_organization_config_rule::builders::PutOrganizationConfigRuleInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.put_organization_config_rule().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::put_organization_config_rule::builders::PutOrganizationConfigRuleInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The name that you assign to an organization Config rule.</p>
    pub fn organization_config_rule_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.organization_config_rule_name(input.into());
        self
    }
    /// <p>The name that you assign to an organization Config rule.</p>
    pub fn set_organization_config_rule_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_organization_config_rule_name(input);
        self
    }
    /// <p>An <code>OrganizationManagedRuleMetadata</code> object. This object specifies organization managed rule metadata such as resource type and ID of Amazon Web Services resource along with the rule identifier. It also provides the frequency with which you want Config to run evaluations for the rule if the trigger type is periodic.</p>
    pub fn organization_managed_rule_metadata(
        mut self,
        input: crate::types::OrganizationManagedRuleMetadata,
    ) -> Self {
        self.inner = self.inner.organization_managed_rule_metadata(input);
        self
    }
    /// <p>An <code>OrganizationManagedRuleMetadata</code> object. This object specifies organization managed rule metadata such as resource type and ID of Amazon Web Services resource along with the rule identifier. It also provides the frequency with which you want Config to run evaluations for the rule if the trigger type is periodic.</p>
    pub fn set_organization_managed_rule_metadata(
        mut self,
        input: std::option::Option<crate::types::OrganizationManagedRuleMetadata>,
    ) -> Self {
        self.inner = self.inner.set_organization_managed_rule_metadata(input);
        self
    }
    /// <p>An <code>OrganizationCustomRuleMetadata</code> object. This object specifies organization custom rule metadata such as resource type, resource ID of Amazon Web Services resource, Lambda function ARN, and organization trigger types that trigger Config to evaluate your Amazon Web Services resources against a rule. It also provides the frequency with which you want Config to run evaluations for the rule if the trigger type is periodic.</p>
    pub fn organization_custom_rule_metadata(
        mut self,
        input: crate::types::OrganizationCustomRuleMetadata,
    ) -> Self {
        self.inner = self.inner.organization_custom_rule_metadata(input);
        self
    }
    /// <p>An <code>OrganizationCustomRuleMetadata</code> object. This object specifies organization custom rule metadata such as resource type, resource ID of Amazon Web Services resource, Lambda function ARN, and organization trigger types that trigger Config to evaluate your Amazon Web Services resources against a rule. It also provides the frequency with which you want Config to run evaluations for the rule if the trigger type is periodic.</p>
    pub fn set_organization_custom_rule_metadata(
        mut self,
        input: std::option::Option<crate::types::OrganizationCustomRuleMetadata>,
    ) -> Self {
        self.inner = self.inner.set_organization_custom_rule_metadata(input);
        self
    }
    /// Appends an item to `ExcludedAccounts`.
    ///
    /// To override the contents of this collection use [`set_excluded_accounts`](Self::set_excluded_accounts).
    ///
    /// <p>A comma-separated list of accounts that you want to exclude from an organization Config rule.</p>
    pub fn excluded_accounts(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.excluded_accounts(input.into());
        self
    }
    /// <p>A comma-separated list of accounts that you want to exclude from an organization Config rule.</p>
    pub fn set_excluded_accounts(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_excluded_accounts(input);
        self
    }
    /// <p>An <code>OrganizationCustomPolicyRuleMetadata</code> object. This object specifies metadata for your organization's Config Custom Policy rule. The metadata includes the runtime system in use, which accounts have debug logging enabled, and other custom rule metadata, such as resource type, resource ID of Amazon Web Services resource, and organization trigger types that initiate Config to evaluate Amazon Web Services resources against a rule.</p>
    pub fn organization_custom_policy_rule_metadata(
        mut self,
        input: crate::types::OrganizationCustomPolicyRuleMetadata,
    ) -> Self {
        self.inner = self.inner.organization_custom_policy_rule_metadata(input);
        self
    }
    /// <p>An <code>OrganizationCustomPolicyRuleMetadata</code> object. This object specifies metadata for your organization's Config Custom Policy rule. The metadata includes the runtime system in use, which accounts have debug logging enabled, and other custom rule metadata, such as resource type, resource ID of Amazon Web Services resource, and organization trigger types that initiate Config to evaluate Amazon Web Services resources against a rule.</p>
    pub fn set_organization_custom_policy_rule_metadata(
        mut self,
        input: std::option::Option<crate::types::OrganizationCustomPolicyRuleMetadata>,
    ) -> Self {
        self.inner = self
            .inner
            .set_organization_custom_policy_rule_metadata(input);
        self
    }
}

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
pub struct PutOrganizationConfigRuleInput {
    /// <p>The name that you assign to an organization Config rule.</p>
    #[doc(hidden)]
    pub organization_config_rule_name: std::option::Option<std::string::String>,
    /// <p>An <code>OrganizationManagedRuleMetadata</code> object. This object specifies organization managed rule metadata such as resource type and ID of Amazon Web Services resource along with the rule identifier. It also provides the frequency with which you want Config to run evaluations for the rule if the trigger type is periodic.</p>
    #[doc(hidden)]
    pub organization_managed_rule_metadata:
        std::option::Option<crate::types::OrganizationManagedRuleMetadata>,
    /// <p>An <code>OrganizationCustomRuleMetadata</code> object. This object specifies organization custom rule metadata such as resource type, resource ID of Amazon Web Services resource, Lambda function ARN, and organization trigger types that trigger Config to evaluate your Amazon Web Services resources against a rule. It also provides the frequency with which you want Config to run evaluations for the rule if the trigger type is periodic.</p>
    #[doc(hidden)]
    pub organization_custom_rule_metadata:
        std::option::Option<crate::types::OrganizationCustomRuleMetadata>,
    /// <p>A comma-separated list of accounts that you want to exclude from an organization Config rule.</p>
    #[doc(hidden)]
    pub excluded_accounts: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>An <code>OrganizationCustomPolicyRuleMetadata</code> object. This object specifies metadata for your organization's Config Custom Policy rule. The metadata includes the runtime system in use, which accounts have debug logging enabled, and other custom rule metadata, such as resource type, resource ID of Amazon Web Services resource, and organization trigger types that initiate Config to evaluate Amazon Web Services resources against a rule.</p>
    #[doc(hidden)]
    pub organization_custom_policy_rule_metadata:
        std::option::Option<crate::types::OrganizationCustomPolicyRuleMetadata>,
}
impl PutOrganizationConfigRuleInput {
    /// <p>The name that you assign to an organization Config rule.</p>
    pub fn organization_config_rule_name(&self) -> std::option::Option<&str> {
        self.organization_config_rule_name.as_deref()
    }
    /// <p>An <code>OrganizationManagedRuleMetadata</code> object. This object specifies organization managed rule metadata such as resource type and ID of Amazon Web Services resource along with the rule identifier. It also provides the frequency with which you want Config to run evaluations for the rule if the trigger type is periodic.</p>
    pub fn organization_managed_rule_metadata(
        &self,
    ) -> std::option::Option<&crate::types::OrganizationManagedRuleMetadata> {
        self.organization_managed_rule_metadata.as_ref()
    }
    /// <p>An <code>OrganizationCustomRuleMetadata</code> object. This object specifies organization custom rule metadata such as resource type, resource ID of Amazon Web Services resource, Lambda function ARN, and organization trigger types that trigger Config to evaluate your Amazon Web Services resources against a rule. It also provides the frequency with which you want Config to run evaluations for the rule if the trigger type is periodic.</p>
    pub fn organization_custom_rule_metadata(
        &self,
    ) -> std::option::Option<&crate::types::OrganizationCustomRuleMetadata> {
        self.organization_custom_rule_metadata.as_ref()
    }
    /// <p>A comma-separated list of accounts that you want to exclude from an organization Config rule.</p>
    pub fn excluded_accounts(&self) -> std::option::Option<&[std::string::String]> {
        self.excluded_accounts.as_deref()
    }
    /// <p>An <code>OrganizationCustomPolicyRuleMetadata</code> object. This object specifies metadata for your organization's Config Custom Policy rule. The metadata includes the runtime system in use, which accounts have debug logging enabled, and other custom rule metadata, such as resource type, resource ID of Amazon Web Services resource, and organization trigger types that initiate Config to evaluate Amazon Web Services resources against a rule.</p>
    pub fn organization_custom_policy_rule_metadata(
        &self,
    ) -> std::option::Option<&crate::types::OrganizationCustomPolicyRuleMetadata> {
        self.organization_custom_policy_rule_metadata.as_ref()
    }
}
impl PutOrganizationConfigRuleInput {
    /// Creates a new builder-style object to manufacture [`PutOrganizationConfigRuleInput`](crate::operation::put_organization_config_rule::PutOrganizationConfigRuleInput).
    pub fn builder() -> crate::operation::put_organization_config_rule::builders::PutOrganizationConfigRuleInputBuilder{
        crate::operation::put_organization_config_rule::builders::PutOrganizationConfigRuleInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::put_organization_config_rule::PutOrganizationConfigRuleInput;
/// A builder for [`PutOrganizationConfigRuleInput`](crate::operation::put_organization_config_rule::PutOrganizationConfigRuleInput).
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
pub struct PutOrganizationConfigRuleInputBuilder {
    pub(crate) organization_config_rule_name: std::option::Option<std::string::String>,
    pub(crate) organization_managed_rule_metadata:
        std::option::Option<crate::types::OrganizationManagedRuleMetadata>,
    pub(crate) organization_custom_rule_metadata:
        std::option::Option<crate::types::OrganizationCustomRuleMetadata>,
    pub(crate) excluded_accounts: std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) organization_custom_policy_rule_metadata:
        std::option::Option<crate::types::OrganizationCustomPolicyRuleMetadata>,
}
impl PutOrganizationConfigRuleInputBuilder {
    /// <p>The name that you assign to an organization Config rule.</p>
    pub fn organization_config_rule_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.organization_config_rule_name = Some(input.into());
        self
    }
    /// <p>The name that you assign to an organization Config rule.</p>
    pub fn set_organization_config_rule_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.organization_config_rule_name = input;
        self
    }
    /// <p>An <code>OrganizationManagedRuleMetadata</code> object. This object specifies organization managed rule metadata such as resource type and ID of Amazon Web Services resource along with the rule identifier. It also provides the frequency with which you want Config to run evaluations for the rule if the trigger type is periodic.</p>
    pub fn organization_managed_rule_metadata(
        mut self,
        input: crate::types::OrganizationManagedRuleMetadata,
    ) -> Self {
        self.organization_managed_rule_metadata = Some(input);
        self
    }
    /// <p>An <code>OrganizationManagedRuleMetadata</code> object. This object specifies organization managed rule metadata such as resource type and ID of Amazon Web Services resource along with the rule identifier. It also provides the frequency with which you want Config to run evaluations for the rule if the trigger type is periodic.</p>
    pub fn set_organization_managed_rule_metadata(
        mut self,
        input: std::option::Option<crate::types::OrganizationManagedRuleMetadata>,
    ) -> Self {
        self.organization_managed_rule_metadata = input;
        self
    }
    /// <p>An <code>OrganizationCustomRuleMetadata</code> object. This object specifies organization custom rule metadata such as resource type, resource ID of Amazon Web Services resource, Lambda function ARN, and organization trigger types that trigger Config to evaluate your Amazon Web Services resources against a rule. It also provides the frequency with which you want Config to run evaluations for the rule if the trigger type is periodic.</p>
    pub fn organization_custom_rule_metadata(
        mut self,
        input: crate::types::OrganizationCustomRuleMetadata,
    ) -> Self {
        self.organization_custom_rule_metadata = Some(input);
        self
    }
    /// <p>An <code>OrganizationCustomRuleMetadata</code> object. This object specifies organization custom rule metadata such as resource type, resource ID of Amazon Web Services resource, Lambda function ARN, and organization trigger types that trigger Config to evaluate your Amazon Web Services resources against a rule. It also provides the frequency with which you want Config to run evaluations for the rule if the trigger type is periodic.</p>
    pub fn set_organization_custom_rule_metadata(
        mut self,
        input: std::option::Option<crate::types::OrganizationCustomRuleMetadata>,
    ) -> Self {
        self.organization_custom_rule_metadata = input;
        self
    }
    /// Appends an item to `excluded_accounts`.
    ///
    /// To override the contents of this collection use [`set_excluded_accounts`](Self::set_excluded_accounts).
    ///
    /// <p>A comma-separated list of accounts that you want to exclude from an organization Config rule.</p>
    pub fn excluded_accounts(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.excluded_accounts.unwrap_or_default();
        v.push(input.into());
        self.excluded_accounts = Some(v);
        self
    }
    /// <p>A comma-separated list of accounts that you want to exclude from an organization Config rule.</p>
    pub fn set_excluded_accounts(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.excluded_accounts = input;
        self
    }
    /// <p>An <code>OrganizationCustomPolicyRuleMetadata</code> object. This object specifies metadata for your organization's Config Custom Policy rule. The metadata includes the runtime system in use, which accounts have debug logging enabled, and other custom rule metadata, such as resource type, resource ID of Amazon Web Services resource, and organization trigger types that initiate Config to evaluate Amazon Web Services resources against a rule.</p>
    pub fn organization_custom_policy_rule_metadata(
        mut self,
        input: crate::types::OrganizationCustomPolicyRuleMetadata,
    ) -> Self {
        self.organization_custom_policy_rule_metadata = Some(input);
        self
    }
    /// <p>An <code>OrganizationCustomPolicyRuleMetadata</code> object. This object specifies metadata for your organization's Config Custom Policy rule. The metadata includes the runtime system in use, which accounts have debug logging enabled, and other custom rule metadata, such as resource type, resource ID of Amazon Web Services resource, and organization trigger types that initiate Config to evaluate Amazon Web Services resources against a rule.</p>
    pub fn set_organization_custom_policy_rule_metadata(
        mut self,
        input: std::option::Option<crate::types::OrganizationCustomPolicyRuleMetadata>,
    ) -> Self {
        self.organization_custom_policy_rule_metadata = input;
        self
    }
    /// Consumes the builder and constructs a [`PutOrganizationConfigRuleInput`](crate::operation::put_organization_config_rule::PutOrganizationConfigRuleInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::put_organization_config_rule::PutOrganizationConfigRuleInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::put_organization_config_rule::PutOrganizationConfigRuleInput {
                organization_config_rule_name: self.organization_config_rule_name,
                organization_managed_rule_metadata: self.organization_managed_rule_metadata,
                organization_custom_rule_metadata: self.organization_custom_rule_metadata,
                excluded_accounts: self.excluded_accounts,
                organization_custom_policy_rule_metadata: self
                    .organization_custom_policy_rule_metadata,
            },
        )
    }
}
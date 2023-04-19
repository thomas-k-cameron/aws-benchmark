// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetOrganizationCustomRulePolicy`](crate::operation::get_organization_custom_rule_policy::builders::GetOrganizationCustomRulePolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`organization_config_rule_name(impl Into<String>)`](crate::operation::get_organization_custom_rule_policy::builders::GetOrganizationCustomRulePolicyFluentBuilder::organization_config_rule_name) / [`set_organization_config_rule_name(Option<String>)`](crate::operation::get_organization_custom_rule_policy::builders::GetOrganizationCustomRulePolicyFluentBuilder::set_organization_config_rule_name): <p>The name of your organization Config Custom Policy rule. </p>
    /// - On success, responds with [`GetOrganizationCustomRulePolicyOutput`](crate::operation::get_organization_custom_rule_policy::GetOrganizationCustomRulePolicyOutput) with field(s):
    ///   - [`policy_text(Option<String>)`](crate::operation::get_organization_custom_rule_policy::GetOrganizationCustomRulePolicyOutput::policy_text): <p>The policy definition containing the logic for your organization Config Custom Policy rule.</p>
    /// - On failure, responds with [`SdkError<GetOrganizationCustomRulePolicyError>`](crate::operation::get_organization_custom_rule_policy::GetOrganizationCustomRulePolicyError)
    pub fn get_organization_custom_rule_policy(&self) -> crate::operation::get_organization_custom_rule_policy::builders::GetOrganizationCustomRulePolicyFluentBuilder{
        crate::operation::get_organization_custom_rule_policy::builders::GetOrganizationCustomRulePolicyFluentBuilder::new(self.handle.clone())
    }
}
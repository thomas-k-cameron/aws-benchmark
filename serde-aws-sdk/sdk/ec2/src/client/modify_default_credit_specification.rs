// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ModifyDefaultCreditSpecification`](crate::operation::modify_default_credit_specification::builders::ModifyDefaultCreditSpecificationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::modify_default_credit_specification::builders::ModifyDefaultCreditSpecificationFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::modify_default_credit_specification::builders::ModifyDefaultCreditSpecificationFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`instance_family(UnlimitedSupportedInstanceFamily)`](crate::operation::modify_default_credit_specification::builders::ModifyDefaultCreditSpecificationFluentBuilder::instance_family) / [`set_instance_family(Option<UnlimitedSupportedInstanceFamily>)`](crate::operation::modify_default_credit_specification::builders::ModifyDefaultCreditSpecificationFluentBuilder::set_instance_family): <p>The instance family.</p>
    ///   - [`cpu_credits(impl Into<String>)`](crate::operation::modify_default_credit_specification::builders::ModifyDefaultCreditSpecificationFluentBuilder::cpu_credits) / [`set_cpu_credits(Option<String>)`](crate::operation::modify_default_credit_specification::builders::ModifyDefaultCreditSpecificationFluentBuilder::set_cpu_credits): <p>The credit option for CPU usage of the instance family.</p>  <p>Valid Values: <code>standard</code> | <code>unlimited</code> </p>
    /// - On success, responds with [`ModifyDefaultCreditSpecificationOutput`](crate::operation::modify_default_credit_specification::ModifyDefaultCreditSpecificationOutput) with field(s):
    ///   - [`instance_family_credit_specification(Option<InstanceFamilyCreditSpecification>)`](crate::operation::modify_default_credit_specification::ModifyDefaultCreditSpecificationOutput::instance_family_credit_specification): <p>The default credit option for CPU usage of the instance family.</p>
    /// - On failure, responds with [`SdkError<ModifyDefaultCreditSpecificationError>`](crate::operation::modify_default_credit_specification::ModifyDefaultCreditSpecificationError)
    pub fn modify_default_credit_specification(&self) -> crate::operation::modify_default_credit_specification::builders::ModifyDefaultCreditSpecificationFluentBuilder{
        crate::operation::modify_default_credit_specification::builders::ModifyDefaultCreditSpecificationFluentBuilder::new(self.handle.clone())
    }
}
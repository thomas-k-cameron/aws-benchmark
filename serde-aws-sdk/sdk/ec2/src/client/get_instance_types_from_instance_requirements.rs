// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetInstanceTypesFromInstanceRequirements`](crate::operation::get_instance_types_from_instance_requirements::builders::GetInstanceTypesFromInstanceRequirementsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::get_instance_types_from_instance_requirements::builders::GetInstanceTypesFromInstanceRequirementsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::get_instance_types_from_instance_requirements::builders::GetInstanceTypesFromInstanceRequirementsFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::get_instance_types_from_instance_requirements::builders::GetInstanceTypesFromInstanceRequirementsFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`architecture_types(Vec<ArchitectureType>)`](crate::operation::get_instance_types_from_instance_requirements::builders::GetInstanceTypesFromInstanceRequirementsFluentBuilder::architecture_types) / [`set_architecture_types(Option<Vec<ArchitectureType>>)`](crate::operation::get_instance_types_from_instance_requirements::builders::GetInstanceTypesFromInstanceRequirementsFluentBuilder::set_architecture_types): <p>The processor architecture type.</p>
    ///   - [`virtualization_types(Vec<VirtualizationType>)`](crate::operation::get_instance_types_from_instance_requirements::builders::GetInstanceTypesFromInstanceRequirementsFluentBuilder::virtualization_types) / [`set_virtualization_types(Option<Vec<VirtualizationType>>)`](crate::operation::get_instance_types_from_instance_requirements::builders::GetInstanceTypesFromInstanceRequirementsFluentBuilder::set_virtualization_types): <p>The virtualization type.</p>
    ///   - [`instance_requirements(InstanceRequirementsRequest)`](crate::operation::get_instance_types_from_instance_requirements::builders::GetInstanceTypesFromInstanceRequirementsFluentBuilder::instance_requirements) / [`set_instance_requirements(Option<InstanceRequirementsRequest>)`](crate::operation::get_instance_types_from_instance_requirements::builders::GetInstanceTypesFromInstanceRequirementsFluentBuilder::set_instance_requirements): <p>The attributes required for the instance types.</p>
    ///   - [`max_results(i32)`](crate::operation::get_instance_types_from_instance_requirements::builders::GetInstanceTypesFromInstanceRequirementsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::get_instance_types_from_instance_requirements::builders::GetInstanceTypesFromInstanceRequirementsFluentBuilder::set_max_results): <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    ///   - [`next_token(impl Into<String>)`](crate::operation::get_instance_types_from_instance_requirements::builders::GetInstanceTypesFromInstanceRequirementsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::get_instance_types_from_instance_requirements::builders::GetInstanceTypesFromInstanceRequirementsFluentBuilder::set_next_token): <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    /// - On success, responds with [`GetInstanceTypesFromInstanceRequirementsOutput`](crate::operation::get_instance_types_from_instance_requirements::GetInstanceTypesFromInstanceRequirementsOutput) with field(s):
    ///   - [`instance_types(Option<Vec<InstanceTypeInfoFromInstanceRequirements>>)`](crate::operation::get_instance_types_from_instance_requirements::GetInstanceTypesFromInstanceRequirementsOutput::instance_types): <p>The instance types with the specified instance attributes.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::get_instance_types_from_instance_requirements::GetInstanceTypesFromInstanceRequirementsOutput::next_token): <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    /// - On failure, responds with [`SdkError<GetInstanceTypesFromInstanceRequirementsError>`](crate::operation::get_instance_types_from_instance_requirements::GetInstanceTypesFromInstanceRequirementsError)
    pub fn get_instance_types_from_instance_requirements(&self) -> crate::operation::get_instance_types_from_instance_requirements::builders::GetInstanceTypesFromInstanceRequirementsFluentBuilder{
        crate::operation::get_instance_types_from_instance_requirements::builders::GetInstanceTypesFromInstanceRequirementsFluentBuilder::new(self.handle.clone())
    }
}
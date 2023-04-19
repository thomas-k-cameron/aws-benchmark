// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutAccessPointConfigurationForObjectLambda`](crate::operation::put_access_point_configuration_for_object_lambda::builders::PutAccessPointConfigurationForObjectLambdaFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`account_id(impl Into<String>)`](crate::operation::put_access_point_configuration_for_object_lambda::builders::PutAccessPointConfigurationForObjectLambdaFluentBuilder::account_id) / [`set_account_id(Option<String>)`](crate::operation::put_access_point_configuration_for_object_lambda::builders::PutAccessPointConfigurationForObjectLambdaFluentBuilder::set_account_id): <p>The account ID for the account that owns the specified Object Lambda Access Point.</p>
    ///   - [`name(impl Into<String>)`](crate::operation::put_access_point_configuration_for_object_lambda::builders::PutAccessPointConfigurationForObjectLambdaFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::put_access_point_configuration_for_object_lambda::builders::PutAccessPointConfigurationForObjectLambdaFluentBuilder::set_name): <p>The name of the Object Lambda Access Point.</p>
    ///   - [`configuration(ObjectLambdaConfiguration)`](crate::operation::put_access_point_configuration_for_object_lambda::builders::PutAccessPointConfigurationForObjectLambdaFluentBuilder::configuration) / [`set_configuration(Option<ObjectLambdaConfiguration>)`](crate::operation::put_access_point_configuration_for_object_lambda::builders::PutAccessPointConfigurationForObjectLambdaFluentBuilder::set_configuration): <p>Object Lambda Access Point configuration document.</p>
    /// - On success, responds with [`PutAccessPointConfigurationForObjectLambdaOutput`](crate::operation::put_access_point_configuration_for_object_lambda::PutAccessPointConfigurationForObjectLambdaOutput)
    /// - On failure, responds with [`SdkError<PutAccessPointConfigurationForObjectLambdaError>`](crate::operation::put_access_point_configuration_for_object_lambda::PutAccessPointConfigurationForObjectLambdaError)
    pub fn put_access_point_configuration_for_object_lambda(&self) -> crate::operation::put_access_point_configuration_for_object_lambda::builders::PutAccessPointConfigurationForObjectLambdaFluentBuilder{
        crate::operation::put_access_point_configuration_for_object_lambda::builders::PutAccessPointConfigurationForObjectLambdaFluentBuilder::new(self.handle.clone())
    }
}
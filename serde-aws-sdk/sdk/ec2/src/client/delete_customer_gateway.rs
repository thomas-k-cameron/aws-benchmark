// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteCustomerGateway`](crate::operation::delete_customer_gateway::builders::DeleteCustomerGatewayFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`customer_gateway_id(impl Into<String>)`](crate::operation::delete_customer_gateway::builders::DeleteCustomerGatewayFluentBuilder::customer_gateway_id) / [`set_customer_gateway_id(Option<String>)`](crate::operation::delete_customer_gateway::builders::DeleteCustomerGatewayFluentBuilder::set_customer_gateway_id): <p>The ID of the customer gateway.</p>
    ///   - [`dry_run(bool)`](crate::operation::delete_customer_gateway::builders::DeleteCustomerGatewayFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::delete_customer_gateway::builders::DeleteCustomerGatewayFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`DeleteCustomerGatewayOutput`](crate::operation::delete_customer_gateway::DeleteCustomerGatewayOutput)
    /// - On failure, responds with [`SdkError<DeleteCustomerGatewayError>`](crate::operation::delete_customer_gateway::DeleteCustomerGatewayError)
    pub fn delete_customer_gateway(
        &self,
    ) -> crate::operation::delete_customer_gateway::builders::DeleteCustomerGatewayFluentBuilder
    {
        crate::operation::delete_customer_gateway::builders::DeleteCustomerGatewayFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
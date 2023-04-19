// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeTransitGatewayPolicyTables`](crate::operation::describe_transit_gateway_policy_tables::builders::DescribeTransitGatewayPolicyTablesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_transit_gateway_policy_tables::builders::DescribeTransitGatewayPolicyTablesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`transit_gateway_policy_table_ids(Vec<String>)`](crate::operation::describe_transit_gateway_policy_tables::builders::DescribeTransitGatewayPolicyTablesFluentBuilder::transit_gateway_policy_table_ids) / [`set_transit_gateway_policy_table_ids(Option<Vec<String>>)`](crate::operation::describe_transit_gateway_policy_tables::builders::DescribeTransitGatewayPolicyTablesFluentBuilder::set_transit_gateway_policy_table_ids): <p>The IDs of the transit gateway policy tables.</p>
    ///   - [`filters(Vec<Filter>)`](crate::operation::describe_transit_gateway_policy_tables::builders::DescribeTransitGatewayPolicyTablesFluentBuilder::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::operation::describe_transit_gateway_policy_tables::builders::DescribeTransitGatewayPolicyTablesFluentBuilder::set_filters): <p>The filters associated with the transit gateway policy table.</p>
    ///   - [`max_results(i32)`](crate::operation::describe_transit_gateway_policy_tables::builders::DescribeTransitGatewayPolicyTablesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_transit_gateway_policy_tables::builders::DescribeTransitGatewayPolicyTablesFluentBuilder::set_max_results): <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_transit_gateway_policy_tables::builders::DescribeTransitGatewayPolicyTablesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_transit_gateway_policy_tables::builders::DescribeTransitGatewayPolicyTablesFluentBuilder::set_next_token): <p>The token for the next page of results.</p>
    ///   - [`dry_run(bool)`](crate::operation::describe_transit_gateway_policy_tables::builders::DescribeTransitGatewayPolicyTablesFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::describe_transit_gateway_policy_tables::builders::DescribeTransitGatewayPolicyTablesFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`DescribeTransitGatewayPolicyTablesOutput`](crate::operation::describe_transit_gateway_policy_tables::DescribeTransitGatewayPolicyTablesOutput) with field(s):
    ///   - [`transit_gateway_policy_tables(Option<Vec<TransitGatewayPolicyTable>>)`](crate::operation::describe_transit_gateway_policy_tables::DescribeTransitGatewayPolicyTablesOutput::transit_gateway_policy_tables): <p>Describes the transit gateway policy tables.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_transit_gateway_policy_tables::DescribeTransitGatewayPolicyTablesOutput::next_token): <p>The token for the next page of results.</p>
    /// - On failure, responds with [`SdkError<DescribeTransitGatewayPolicyTablesError>`](crate::operation::describe_transit_gateway_policy_tables::DescribeTransitGatewayPolicyTablesError)
    pub fn describe_transit_gateway_policy_tables(&self) -> crate::operation::describe_transit_gateway_policy_tables::builders::DescribeTransitGatewayPolicyTablesFluentBuilder{
        crate::operation::describe_transit_gateway_policy_tables::builders::DescribeTransitGatewayPolicyTablesFluentBuilder::new(self.handle.clone())
    }
}
// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeCarrierGateways`](crate::operation::describe_carrier_gateways::builders::DescribeCarrierGatewaysFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_carrier_gateways::builders::DescribeCarrierGatewaysFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`carrier_gateway_ids(Vec<String>)`](crate::operation::describe_carrier_gateways::builders::DescribeCarrierGatewaysFluentBuilder::carrier_gateway_ids) / [`set_carrier_gateway_ids(Option<Vec<String>>)`](crate::operation::describe_carrier_gateways::builders::DescribeCarrierGatewaysFluentBuilder::set_carrier_gateway_ids): <p>One or more carrier gateway IDs.</p>
    ///   - [`filters(Vec<Filter>)`](crate::operation::describe_carrier_gateways::builders::DescribeCarrierGatewaysFluentBuilder::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::operation::describe_carrier_gateways::builders::DescribeCarrierGatewaysFluentBuilder::set_filters): <p>One or more filters.</p>  <ul>   <li> <p> <code>carrier-gateway-id</code> - The ID of the carrier gateway.</p> </li>   <li> <p> <code>state</code> - The state of the carrier gateway (<code>pending</code> | <code>failed</code> | <code>available</code> | <code>deleting</code> | <code>deleted</code>).</p> </li>   <li> <p> <code>owner-id</code> - The Amazon Web Services account ID of the owner of the carrier gateway.</p> </li>   <li> <p> <code>tag</code>:<key>      - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key      <code>Owner</code> and the value      <code>TeamA</code>, specify      <code>tag:Owner</code> for the filter name and      <code>TeamA</code> for the filter value.    </key></p> </li>   <li> <p> <code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p> </li>   <li> <p> <code>vpc-id</code> - The ID of the VPC associated with the carrier gateway.</p> </li>  </ul>
    ///   - [`max_results(i32)`](crate::operation::describe_carrier_gateways::builders::DescribeCarrierGatewaysFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_carrier_gateways::builders::DescribeCarrierGatewaysFluentBuilder::set_max_results): <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_carrier_gateways::builders::DescribeCarrierGatewaysFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_carrier_gateways::builders::DescribeCarrierGatewaysFluentBuilder::set_next_token): <p>The token for the next page of results.</p>
    ///   - [`dry_run(bool)`](crate::operation::describe_carrier_gateways::builders::DescribeCarrierGatewaysFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::describe_carrier_gateways::builders::DescribeCarrierGatewaysFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`DescribeCarrierGatewaysOutput`](crate::operation::describe_carrier_gateways::DescribeCarrierGatewaysOutput) with field(s):
    ///   - [`carrier_gateways(Option<Vec<CarrierGateway>>)`](crate::operation::describe_carrier_gateways::DescribeCarrierGatewaysOutput::carrier_gateways): <p>Information about the carrier gateway.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_carrier_gateways::DescribeCarrierGatewaysOutput::next_token): <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    /// - On failure, responds with [`SdkError<DescribeCarrierGatewaysError>`](crate::operation::describe_carrier_gateways::DescribeCarrierGatewaysError)
    pub fn describe_carrier_gateways(
        &self,
    ) -> crate::operation::describe_carrier_gateways::builders::DescribeCarrierGatewaysFluentBuilder
    {
        crate::operation::describe_carrier_gateways::builders::DescribeCarrierGatewaysFluentBuilder::new(self.handle.clone())
    }
}
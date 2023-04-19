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
pub struct SearchLocalGatewayRoutesInput {
    /// <p>The ID of the local gateway route table.</p>
    #[doc(hidden)]
    pub local_gateway_route_table_id: std::option::Option<std::string::String>,
    /// <p>One or more filters.</p>
    /// <ul>
    /// <li> <p> <code>prefix-list-id</code> - The ID of the prefix list.</p> </li>
    /// <li> <p> <code>route-search.exact-match</code> - The exact match of the specified filter.</p> </li>
    /// <li> <p> <code>route-search.longest-prefix-match</code> - The longest prefix that matches the route.</p> </li>
    /// <li> <p> <code>route-search.subnet-of-match</code> - The routes with a subnet that match the specified CIDR filter.</p> </li>
    /// <li> <p> <code>route-search.supernet-of-match</code> - The routes with a CIDR that encompass the CIDR filter. For example, if you have 10.0.1.0/29 and 10.0.1.0/31 routes in your route table and you specify <code>supernet-of-match</code> as 10.0.1.0/30, then the result returns 10.0.1.0/29.</p> </li>
    /// <li> <p> <code>state</code> - The state of the route.</p> </li>
    /// <li> <p> <code>type</code> - The route type.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub filters: std::option::Option<std::vec::Vec<crate::types::Filter>>,
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    #[doc(hidden)]
    pub max_results: std::option::Option<i32>,
    /// <p>The token for the next page of results.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
}
impl SearchLocalGatewayRoutesInput {
    /// <p>The ID of the local gateway route table.</p>
    pub fn local_gateway_route_table_id(&self) -> std::option::Option<&str> {
        self.local_gateway_route_table_id.as_deref()
    }
    /// <p>One or more filters.</p>
    /// <ul>
    /// <li> <p> <code>prefix-list-id</code> - The ID of the prefix list.</p> </li>
    /// <li> <p> <code>route-search.exact-match</code> - The exact match of the specified filter.</p> </li>
    /// <li> <p> <code>route-search.longest-prefix-match</code> - The longest prefix that matches the route.</p> </li>
    /// <li> <p> <code>route-search.subnet-of-match</code> - The routes with a subnet that match the specified CIDR filter.</p> </li>
    /// <li> <p> <code>route-search.supernet-of-match</code> - The routes with a CIDR that encompass the CIDR filter. For example, if you have 10.0.1.0/29 and 10.0.1.0/31 routes in your route table and you specify <code>supernet-of-match</code> as 10.0.1.0/30, then the result returns 10.0.1.0/29.</p> </li>
    /// <li> <p> <code>state</code> - The state of the route.</p> </li>
    /// <li> <p> <code>type</code> - The route type.</p> </li>
    /// </ul>
    pub fn filters(&self) -> std::option::Option<&[crate::types::Filter]> {
        self.filters.as_deref()
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn max_results(&self) -> std::option::Option<i32> {
        self.max_results
    }
    /// <p>The token for the next page of results.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
}
impl SearchLocalGatewayRoutesInput {
    /// Creates a new builder-style object to manufacture [`SearchLocalGatewayRoutesInput`](crate::operation::search_local_gateway_routes::SearchLocalGatewayRoutesInput).
    pub fn builder(
    ) -> crate::operation::search_local_gateway_routes::builders::SearchLocalGatewayRoutesInputBuilder
    {
        crate::operation::search_local_gateway_routes::builders::SearchLocalGatewayRoutesInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::search_local_gateway_routes::SearchLocalGatewayRoutesInput;
/// A builder for [`SearchLocalGatewayRoutesInput`](crate::operation::search_local_gateway_routes::SearchLocalGatewayRoutesInput).
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
pub struct SearchLocalGatewayRoutesInputBuilder {
    pub(crate) local_gateway_route_table_id: std::option::Option<std::string::String>,
    pub(crate) filters: std::option::Option<std::vec::Vec<crate::types::Filter>>,
    pub(crate) max_results: std::option::Option<i32>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    pub(crate) dry_run: std::option::Option<bool>,
}
impl SearchLocalGatewayRoutesInputBuilder {
    /// <p>The ID of the local gateway route table.</p>
    pub fn local_gateway_route_table_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.local_gateway_route_table_id = Some(input.into());
        self
    }
    /// <p>The ID of the local gateway route table.</p>
    pub fn set_local_gateway_route_table_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.local_gateway_route_table_id = input;
        self
    }
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>One or more filters.</p>
    /// <ul>
    /// <li> <p> <code>prefix-list-id</code> - The ID of the prefix list.</p> </li>
    /// <li> <p> <code>route-search.exact-match</code> - The exact match of the specified filter.</p> </li>
    /// <li> <p> <code>route-search.longest-prefix-match</code> - The longest prefix that matches the route.</p> </li>
    /// <li> <p> <code>route-search.subnet-of-match</code> - The routes with a subnet that match the specified CIDR filter.</p> </li>
    /// <li> <p> <code>route-search.supernet-of-match</code> - The routes with a CIDR that encompass the CIDR filter. For example, if you have 10.0.1.0/29 and 10.0.1.0/31 routes in your route table and you specify <code>supernet-of-match</code> as 10.0.1.0/30, then the result returns 10.0.1.0/29.</p> </li>
    /// <li> <p> <code>state</code> - The state of the route.</p> </li>
    /// <li> <p> <code>type</code> - The route type.</p> </li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        let mut v = self.filters.unwrap_or_default();
        v.push(input);
        self.filters = Some(v);
        self
    }
    /// <p>One or more filters.</p>
    /// <ul>
    /// <li> <p> <code>prefix-list-id</code> - The ID of the prefix list.</p> </li>
    /// <li> <p> <code>route-search.exact-match</code> - The exact match of the specified filter.</p> </li>
    /// <li> <p> <code>route-search.longest-prefix-match</code> - The longest prefix that matches the route.</p> </li>
    /// <li> <p> <code>route-search.subnet-of-match</code> - The routes with a subnet that match the specified CIDR filter.</p> </li>
    /// <li> <p> <code>route-search.supernet-of-match</code> - The routes with a CIDR that encompass the CIDR filter. For example, if you have 10.0.1.0/29 and 10.0.1.0/31 routes in your route table and you specify <code>supernet-of-match</code> as 10.0.1.0/30, then the result returns 10.0.1.0/29.</p> </li>
    /// <li> <p> <code>state</code> - The state of the route.</p> </li>
    /// <li> <p> <code>type</code> - The route type.</p> </li>
    /// </ul>
    pub fn set_filters(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Filter>>,
    ) -> Self {
        self.filters = input;
        self
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = Some(input);
        self
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>The token for the next page of results.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>The token for the next page of results.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// Consumes the builder and constructs a [`SearchLocalGatewayRoutesInput`](crate::operation::search_local_gateway_routes::SearchLocalGatewayRoutesInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::search_local_gateway_routes::SearchLocalGatewayRoutesInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::search_local_gateway_routes::SearchLocalGatewayRoutesInput {
                local_gateway_route_table_id: self.local_gateway_route_table_id,
                filters: self.filters,
                max_results: self.max_results,
                next_token: self.next_token,
                dry_run: self.dry_run,
            },
        )
    }
}
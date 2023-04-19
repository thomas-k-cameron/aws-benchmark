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
pub struct ListResourceEvaluationsInput {
    /// <p>Returns a <code>ResourceEvaluationFilters</code> object.</p>
    #[doc(hidden)]
    pub filters: std::option::Option<crate::types::ResourceEvaluationFilters>,
    /// <p>The maximum number of evaluations returned on each page. The default is 10. You cannot specify a number greater than 100. If you specify 0, Config uses the default.</p>
    #[doc(hidden)]
    pub limit: i32,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
}
impl ListResourceEvaluationsInput {
    /// <p>Returns a <code>ResourceEvaluationFilters</code> object.</p>
    pub fn filters(&self) -> std::option::Option<&crate::types::ResourceEvaluationFilters> {
        self.filters.as_ref()
    }
    /// <p>The maximum number of evaluations returned on each page. The default is 10. You cannot specify a number greater than 100. If you specify 0, Config uses the default.</p>
    pub fn limit(&self) -> i32 {
        self.limit
    }
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ListResourceEvaluationsInput {
    /// Creates a new builder-style object to manufacture [`ListResourceEvaluationsInput`](crate::operation::list_resource_evaluations::ListResourceEvaluationsInput).
    pub fn builder(
    ) -> crate::operation::list_resource_evaluations::builders::ListResourceEvaluationsInputBuilder
    {
        crate::operation::list_resource_evaluations::builders::ListResourceEvaluationsInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::list_resource_evaluations::ListResourceEvaluationsInput;
/// A builder for [`ListResourceEvaluationsInput`](crate::operation::list_resource_evaluations::ListResourceEvaluationsInput).
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
pub struct ListResourceEvaluationsInputBuilder {
    pub(crate) filters: std::option::Option<crate::types::ResourceEvaluationFilters>,
    pub(crate) limit: std::option::Option<i32>,
    pub(crate) next_token: std::option::Option<std::string::String>,
}
impl ListResourceEvaluationsInputBuilder {
    /// <p>Returns a <code>ResourceEvaluationFilters</code> object.</p>
    pub fn filters(mut self, input: crate::types::ResourceEvaluationFilters) -> Self {
        self.filters = Some(input);
        self
    }
    /// <p>Returns a <code>ResourceEvaluationFilters</code> object.</p>
    pub fn set_filters(
        mut self,
        input: std::option::Option<crate::types::ResourceEvaluationFilters>,
    ) -> Self {
        self.filters = input;
        self
    }
    /// <p>The maximum number of evaluations returned on each page. The default is 10. You cannot specify a number greater than 100. If you specify 0, Config uses the default.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.limit = Some(input);
        self
    }
    /// <p>The maximum number of evaluations returned on each page. The default is 10. You cannot specify a number greater than 100. If you specify 0, Config uses the default.</p>
    pub fn set_limit(mut self, input: std::option::Option<i32>) -> Self {
        self.limit = input;
        self
    }
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Consumes the builder and constructs a [`ListResourceEvaluationsInput`](crate::operation::list_resource_evaluations::ListResourceEvaluationsInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::list_resource_evaluations::ListResourceEvaluationsInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::list_resource_evaluations::ListResourceEvaluationsInput {
                filters: self.filters,
                limit: self.limit.unwrap_or_default(),
                next_token: self.next_token,
            },
        )
    }
}
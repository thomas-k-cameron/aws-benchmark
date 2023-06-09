// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An access point with an attached Lambda function used to access transformed data from an Amazon S3 bucket.</p>
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
pub struct ObjectLambdaAccessPoint {
    /// <p>The name of the Object Lambda Access Point.</p>
    #[doc(hidden)]
    pub name: std::option::Option<std::string::String>,
    /// <p>Specifies the ARN for the Object Lambda Access Point.</p>
    #[doc(hidden)]
    pub object_lambda_access_point_arn: std::option::Option<std::string::String>,
}
impl ObjectLambdaAccessPoint {
    /// <p>The name of the Object Lambda Access Point.</p>
    pub fn name(&self) -> std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>Specifies the ARN for the Object Lambda Access Point.</p>
    pub fn object_lambda_access_point_arn(&self) -> std::option::Option<&str> {
        self.object_lambda_access_point_arn.as_deref()
    }
}
impl ObjectLambdaAccessPoint {
    /// Creates a new builder-style object to manufacture [`ObjectLambdaAccessPoint`](crate::types::ObjectLambdaAccessPoint).
    pub fn builder() -> crate::types::builders::ObjectLambdaAccessPointBuilder {
        crate::types::builders::ObjectLambdaAccessPointBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::ObjectLambdaAccessPoint;
/// A builder for [`ObjectLambdaAccessPoint`](crate::types::ObjectLambdaAccessPoint).
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
pub struct ObjectLambdaAccessPointBuilder {
    pub(crate) name: std::option::Option<std::string::String>,
    pub(crate) object_lambda_access_point_arn: std::option::Option<std::string::String>,
}
impl ObjectLambdaAccessPointBuilder {
    /// <p>The name of the Object Lambda Access Point.</p>
    pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
        self.name = Some(input.into());
        self
    }
    /// <p>The name of the Object Lambda Access Point.</p>
    pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>Specifies the ARN for the Object Lambda Access Point.</p>
    pub fn object_lambda_access_point_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.object_lambda_access_point_arn = Some(input.into());
        self
    }
    /// <p>Specifies the ARN for the Object Lambda Access Point.</p>
    pub fn set_object_lambda_access_point_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.object_lambda_access_point_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`ObjectLambdaAccessPoint`](crate::types::ObjectLambdaAccessPoint).
    pub fn build(self) -> crate::types::ObjectLambdaAccessPoint {
        crate::types::ObjectLambdaAccessPoint {
            name: self.name,
            object_lambda_access_point_arn: self.object_lambda_access_point_arn,
        }
    }
}

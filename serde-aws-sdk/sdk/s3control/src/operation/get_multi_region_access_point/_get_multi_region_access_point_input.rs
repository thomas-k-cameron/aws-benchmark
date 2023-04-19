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
pub struct GetMultiRegionAccessPointInput {
    /// <p>The Amazon Web Services account ID for the owner of the Multi-Region Access Point.</p>
    #[doc(hidden)]
    pub account_id: std::option::Option<std::string::String>,
    /// <p>The name of the Multi-Region Access Point whose configuration information you want to receive. The name of the Multi-Region Access Point is different from the alias. For more information about the distinction between the name and the alias of an Multi-Region Access Point, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/CreatingMultiRegionAccessPoints.html#multi-region-access-point-naming">Managing Multi-Region Access Points</a> in the <i>Amazon S3 User Guide</i>.</p>
    #[doc(hidden)]
    pub name: std::option::Option<std::string::String>,
}
impl GetMultiRegionAccessPointInput {
    /// <p>The Amazon Web Services account ID for the owner of the Multi-Region Access Point.</p>
    pub fn account_id(&self) -> std::option::Option<&str> {
        self.account_id.as_deref()
    }
    /// <p>The name of the Multi-Region Access Point whose configuration information you want to receive. The name of the Multi-Region Access Point is different from the alias. For more information about the distinction between the name and the alias of an Multi-Region Access Point, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/CreatingMultiRegionAccessPoints.html#multi-region-access-point-naming">Managing Multi-Region Access Points</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn name(&self) -> std::option::Option<&str> {
        self.name.as_deref()
    }
}
impl GetMultiRegionAccessPointInput {
    /// Creates a new builder-style object to manufacture [`GetMultiRegionAccessPointInput`](crate::operation::get_multi_region_access_point::GetMultiRegionAccessPointInput).
    pub fn builder() -> crate::operation::get_multi_region_access_point::builders::GetMultiRegionAccessPointInputBuilder{
        crate::operation::get_multi_region_access_point::builders::GetMultiRegionAccessPointInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::get_multi_region_access_point::GetMultiRegionAccessPointInput;
/// A builder for [`GetMultiRegionAccessPointInput`](crate::operation::get_multi_region_access_point::GetMultiRegionAccessPointInput).
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
pub struct GetMultiRegionAccessPointInputBuilder {
    pub(crate) account_id: std::option::Option<std::string::String>,
    pub(crate) name: std::option::Option<std::string::String>,
}
impl GetMultiRegionAccessPointInputBuilder {
    /// <p>The Amazon Web Services account ID for the owner of the Multi-Region Access Point.</p>
    pub fn account_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.account_id = Some(input.into());
        self
    }
    /// <p>The Amazon Web Services account ID for the owner of the Multi-Region Access Point.</p>
    pub fn set_account_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.account_id = input;
        self
    }
    /// <p>The name of the Multi-Region Access Point whose configuration information you want to receive. The name of the Multi-Region Access Point is different from the alias. For more information about the distinction between the name and the alias of an Multi-Region Access Point, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/CreatingMultiRegionAccessPoints.html#multi-region-access-point-naming">Managing Multi-Region Access Points</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
        self.name = Some(input.into());
        self
    }
    /// <p>The name of the Multi-Region Access Point whose configuration information you want to receive. The name of the Multi-Region Access Point is different from the alias. For more information about the distinction between the name and the alias of an Multi-Region Access Point, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/CreatingMultiRegionAccessPoints.html#multi-region-access-point-naming">Managing Multi-Region Access Points</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// Consumes the builder and constructs a [`GetMultiRegionAccessPointInput`](crate::operation::get_multi_region_access_point::GetMultiRegionAccessPointInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::get_multi_region_access_point::GetMultiRegionAccessPointInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::get_multi_region_access_point::GetMultiRegionAccessPointInput {
                account_id: self.account_id,
                name: self.name,
            },
        )
    }
}
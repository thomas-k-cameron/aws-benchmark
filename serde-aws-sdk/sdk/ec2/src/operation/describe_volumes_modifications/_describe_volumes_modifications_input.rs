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
pub struct DescribeVolumesModificationsInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
    /// <p>The IDs of the volumes.</p>
    #[doc(hidden)]
    pub volume_ids: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>modification-state</code> - The current modification state (modifying | optimizing | completed | failed).</p> </li>
    /// <li> <p> <code>original-iops</code> - The original IOPS rate of the volume.</p> </li>
    /// <li> <p> <code>original-size</code> - The original size of the volume, in GiB.</p> </li>
    /// <li> <p> <code>original-volume-type</code> - The original volume type of the volume (standard | io1 | io2 | gp2 | sc1 | st1).</p> </li>
    /// <li> <p> <code>originalMultiAttachEnabled</code> - Indicates whether Multi-Attach support was enabled (true | false).</p> </li>
    /// <li> <p> <code>start-time</code> - The modification start time.</p> </li>
    /// <li> <p> <code>target-iops</code> - The target IOPS rate of the volume.</p> </li>
    /// <li> <p> <code>target-size</code> - The target size of the volume, in GiB.</p> </li>
    /// <li> <p> <code>target-volume-type</code> - The target volume type of the volume (standard | io1 | io2 | gp2 | sc1 | st1).</p> </li>
    /// <li> <p> <code>targetMultiAttachEnabled</code> - Indicates whether Multi-Attach support is to be enabled (true | false).</p> </li>
    /// <li> <p> <code>volume-id</code> - The ID of the volume.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub filters: std::option::Option<std::vec::Vec<crate::types::Filter>>,
    /// <p>The token returned by a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    /// <p>The maximum number of results (up to a limit of 500) to be returned in a paginated request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    #[doc(hidden)]
    pub max_results: std::option::Option<i32>,
}
impl DescribeVolumesModificationsInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The IDs of the volumes.</p>
    pub fn volume_ids(&self) -> std::option::Option<&[std::string::String]> {
        self.volume_ids.as_deref()
    }
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>modification-state</code> - The current modification state (modifying | optimizing | completed | failed).</p> </li>
    /// <li> <p> <code>original-iops</code> - The original IOPS rate of the volume.</p> </li>
    /// <li> <p> <code>original-size</code> - The original size of the volume, in GiB.</p> </li>
    /// <li> <p> <code>original-volume-type</code> - The original volume type of the volume (standard | io1 | io2 | gp2 | sc1 | st1).</p> </li>
    /// <li> <p> <code>originalMultiAttachEnabled</code> - Indicates whether Multi-Attach support was enabled (true | false).</p> </li>
    /// <li> <p> <code>start-time</code> - The modification start time.</p> </li>
    /// <li> <p> <code>target-iops</code> - The target IOPS rate of the volume.</p> </li>
    /// <li> <p> <code>target-size</code> - The target size of the volume, in GiB.</p> </li>
    /// <li> <p> <code>target-volume-type</code> - The target volume type of the volume (standard | io1 | io2 | gp2 | sc1 | st1).</p> </li>
    /// <li> <p> <code>targetMultiAttachEnabled</code> - Indicates whether Multi-Attach support is to be enabled (true | false).</p> </li>
    /// <li> <p> <code>volume-id</code> - The ID of the volume.</p> </li>
    /// </ul>
    pub fn filters(&self) -> std::option::Option<&[crate::types::Filter]> {
        self.filters.as_deref()
    }
    /// <p>The token returned by a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The maximum number of results (up to a limit of 500) to be returned in a paginated request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    pub fn max_results(&self) -> std::option::Option<i32> {
        self.max_results
    }
}
impl DescribeVolumesModificationsInput {
    /// Creates a new builder-style object to manufacture [`DescribeVolumesModificationsInput`](crate::operation::describe_volumes_modifications::DescribeVolumesModificationsInput).
    pub fn builder() -> crate::operation::describe_volumes_modifications::builders::DescribeVolumesModificationsInputBuilder{
        crate::operation::describe_volumes_modifications::builders::DescribeVolumesModificationsInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::describe_volumes_modifications::DescribeVolumesModificationsInput;
/// A builder for [`DescribeVolumesModificationsInput`](crate::operation::describe_volumes_modifications::DescribeVolumesModificationsInput).
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
pub struct DescribeVolumesModificationsInputBuilder {
    pub(crate) dry_run: std::option::Option<bool>,
    pub(crate) volume_ids: std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) filters: std::option::Option<std::vec::Vec<crate::types::Filter>>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    pub(crate) max_results: std::option::Option<i32>,
}
impl DescribeVolumesModificationsInputBuilder {
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
    /// Appends an item to `volume_ids`.
    ///
    /// To override the contents of this collection use [`set_volume_ids`](Self::set_volume_ids).
    ///
    /// <p>The IDs of the volumes.</p>
    pub fn volume_ids(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.volume_ids.unwrap_or_default();
        v.push(input.into());
        self.volume_ids = Some(v);
        self
    }
    /// <p>The IDs of the volumes.</p>
    pub fn set_volume_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.volume_ids = input;
        self
    }
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>modification-state</code> - The current modification state (modifying | optimizing | completed | failed).</p> </li>
    /// <li> <p> <code>original-iops</code> - The original IOPS rate of the volume.</p> </li>
    /// <li> <p> <code>original-size</code> - The original size of the volume, in GiB.</p> </li>
    /// <li> <p> <code>original-volume-type</code> - The original volume type of the volume (standard | io1 | io2 | gp2 | sc1 | st1).</p> </li>
    /// <li> <p> <code>originalMultiAttachEnabled</code> - Indicates whether Multi-Attach support was enabled (true | false).</p> </li>
    /// <li> <p> <code>start-time</code> - The modification start time.</p> </li>
    /// <li> <p> <code>target-iops</code> - The target IOPS rate of the volume.</p> </li>
    /// <li> <p> <code>target-size</code> - The target size of the volume, in GiB.</p> </li>
    /// <li> <p> <code>target-volume-type</code> - The target volume type of the volume (standard | io1 | io2 | gp2 | sc1 | st1).</p> </li>
    /// <li> <p> <code>targetMultiAttachEnabled</code> - Indicates whether Multi-Attach support is to be enabled (true | false).</p> </li>
    /// <li> <p> <code>volume-id</code> - The ID of the volume.</p> </li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        let mut v = self.filters.unwrap_or_default();
        v.push(input);
        self.filters = Some(v);
        self
    }
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>modification-state</code> - The current modification state (modifying | optimizing | completed | failed).</p> </li>
    /// <li> <p> <code>original-iops</code> - The original IOPS rate of the volume.</p> </li>
    /// <li> <p> <code>original-size</code> - The original size of the volume, in GiB.</p> </li>
    /// <li> <p> <code>original-volume-type</code> - The original volume type of the volume (standard | io1 | io2 | gp2 | sc1 | st1).</p> </li>
    /// <li> <p> <code>originalMultiAttachEnabled</code> - Indicates whether Multi-Attach support was enabled (true | false).</p> </li>
    /// <li> <p> <code>start-time</code> - The modification start time.</p> </li>
    /// <li> <p> <code>target-iops</code> - The target IOPS rate of the volume.</p> </li>
    /// <li> <p> <code>target-size</code> - The target size of the volume, in GiB.</p> </li>
    /// <li> <p> <code>target-volume-type</code> - The target volume type of the volume (standard | io1 | io2 | gp2 | sc1 | st1).</p> </li>
    /// <li> <p> <code>targetMultiAttachEnabled</code> - Indicates whether Multi-Attach support is to be enabled (true | false).</p> </li>
    /// <li> <p> <code>volume-id</code> - The ID of the volume.</p> </li>
    /// </ul>
    pub fn set_filters(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Filter>>,
    ) -> Self {
        self.filters = input;
        self
    }
    /// <p>The token returned by a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>The token returned by a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The maximum number of results (up to a limit of 500) to be returned in a paginated request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = Some(input);
        self
    }
    /// <p>The maximum number of results (up to a limit of 500) to be returned in a paginated request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeVolumesModificationsInput`](crate::operation::describe_volumes_modifications::DescribeVolumesModificationsInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::describe_volumes_modifications::DescribeVolumesModificationsInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::describe_volumes_modifications::DescribeVolumesModificationsInput {
                dry_run: self.dry_run,
                volume_ids: self.volume_ids,
                filters: self.filters,
                next_token: self.next_token,
                max_results: self.max_results,
            },
        )
    }
}
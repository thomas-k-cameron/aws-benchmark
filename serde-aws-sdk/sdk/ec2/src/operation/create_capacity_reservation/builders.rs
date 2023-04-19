// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_capacity_reservation::_create_capacity_reservation_output::CreateCapacityReservationOutputBuilder;

pub use crate::operation::create_capacity_reservation::_create_capacity_reservation_input::CreateCapacityReservationInputBuilder;

/// Fluent builder constructing a request to `CreateCapacityReservation`.
///
/// <p>Creates a new Capacity Reservation with the specified attributes.</p>
/// <p>Capacity Reservations enable you to reserve capacity for your Amazon EC2 instances in a specific Availability Zone for any duration. This gives you the flexibility to selectively add capacity reservations and still get the Regional RI discounts for that usage. By creating Capacity Reservations, you ensure that you always have access to Amazon EC2 capacity when you need it, for as long as you need it. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-capacity-reservations.html">Capacity Reservations</a> in the <i>Amazon EC2 User Guide</i>.</p>
/// <p>Your request to create a Capacity Reservation could fail if Amazon EC2 does not have sufficient capacity to fulfill the request. If your request fails due to Amazon EC2 capacity constraints, either try again at a later time, try in a different Availability Zone, or request a smaller capacity reservation. If your application is flexible across instance types and sizes, try to create a Capacity Reservation with different instance attributes.</p>
/// <p>Your request could also fail if the requested quantity exceeds your On-Demand Instance limit for the selected instance type. If your request fails due to limit constraints, increase your On-Demand Instance limit for the required instance type and try again. For more information about increasing your instance limits, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-resource-limits.html">Amazon EC2 Service Quotas</a> in the <i>Amazon EC2 User Guide</i>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateCapacityReservationFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::create_capacity_reservation::builders::CreateCapacityReservationInputBuilder
            }
impl CreateCapacityReservationFluentBuilder {
    /// Creates a new `CreateCapacityReservation`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::create_capacity_reservation::CreateCapacityReservation,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::create_capacity_reservation::CreateCapacityReservationError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        Ok(crate::client::customize::CustomizableOperation { handle, operation })
    }

    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> std::result::Result<
        crate::operation::create_capacity_reservation::CreateCapacityReservationOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::create_capacity_reservation::CreateCapacityReservationError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    #[cfg(aws_sdk_unstable)]
    /// This function replaces the parameter with new one.
    /// It is useful when you want to replace the existing data with de-serialized data.
    /// ```compile_fail
    /// let result_future = async {
    ///     let deserialized_parameters: crate::operation::create_capacity_reservation::builders::CreateCapacityReservationInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.create_capacity_reservation().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::create_capacity_reservation::builders::CreateCapacityReservationInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensure Idempotency</a>.</p>
    pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensure Idempotency</a>.</p>
    pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>The instance type for which to reserve capacity. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html">Instance types</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn instance_type(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.instance_type(input.into());
        self
    }
    /// <p>The instance type for which to reserve capacity. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html">Instance types</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn set_instance_type(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_instance_type(input);
        self
    }
    /// <p>The type of operating system for which to reserve capacity.</p>
    pub fn instance_platform(
        mut self,
        input: crate::types::CapacityReservationInstancePlatform,
    ) -> Self {
        self.inner = self.inner.instance_platform(input);
        self
    }
    /// <p>The type of operating system for which to reserve capacity.</p>
    pub fn set_instance_platform(
        mut self,
        input: std::option::Option<crate::types::CapacityReservationInstancePlatform>,
    ) -> Self {
        self.inner = self.inner.set_instance_platform(input);
        self
    }
    /// <p>The Availability Zone in which to create the Capacity Reservation.</p>
    pub fn availability_zone(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.availability_zone(input.into());
        self
    }
    /// <p>The Availability Zone in which to create the Capacity Reservation.</p>
    pub fn set_availability_zone(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_availability_zone(input);
        self
    }
    /// <p>The ID of the Availability Zone in which to create the Capacity Reservation.</p>
    pub fn availability_zone_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.availability_zone_id(input.into());
        self
    }
    /// <p>The ID of the Availability Zone in which to create the Capacity Reservation.</p>
    pub fn set_availability_zone_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_availability_zone_id(input);
        self
    }
    /// <p>Indicates the tenancy of the Capacity Reservation. A Capacity Reservation can have one of the following tenancy settings:</p>
    /// <ul>
    /// <li> <p> <code>default</code> - The Capacity Reservation is created on hardware that is shared with other Amazon Web Services accounts.</p> </li>
    /// <li> <p> <code>dedicated</code> - The Capacity Reservation is created on single-tenant hardware that is dedicated to a single Amazon Web Services account.</p> </li>
    /// </ul>
    pub fn tenancy(mut self, input: crate::types::CapacityReservationTenancy) -> Self {
        self.inner = self.inner.tenancy(input);
        self
    }
    /// <p>Indicates the tenancy of the Capacity Reservation. A Capacity Reservation can have one of the following tenancy settings:</p>
    /// <ul>
    /// <li> <p> <code>default</code> - The Capacity Reservation is created on hardware that is shared with other Amazon Web Services accounts.</p> </li>
    /// <li> <p> <code>dedicated</code> - The Capacity Reservation is created on single-tenant hardware that is dedicated to a single Amazon Web Services account.</p> </li>
    /// </ul>
    pub fn set_tenancy(
        mut self,
        input: std::option::Option<crate::types::CapacityReservationTenancy>,
    ) -> Self {
        self.inner = self.inner.set_tenancy(input);
        self
    }
    /// <p>The number of instances for which to reserve capacity.</p>
    /// <p>Valid range: 1 - 1000</p>
    pub fn instance_count(mut self, input: i32) -> Self {
        self.inner = self.inner.instance_count(input);
        self
    }
    /// <p>The number of instances for which to reserve capacity.</p>
    /// <p>Valid range: 1 - 1000</p>
    pub fn set_instance_count(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_instance_count(input);
        self
    }
    /// <p>Indicates whether the Capacity Reservation supports EBS-optimized instances. This optimization provides dedicated throughput to Amazon EBS and an optimized configuration stack to provide optimal I/O performance. This optimization isn't available with all instance types. Additional usage charges apply when using an EBS- optimized instance.</p>
    pub fn ebs_optimized(mut self, input: bool) -> Self {
        self.inner = self.inner.ebs_optimized(input);
        self
    }
    /// <p>Indicates whether the Capacity Reservation supports EBS-optimized instances. This optimization provides dedicated throughput to Amazon EBS and an optimized configuration stack to provide optimal I/O performance. This optimization isn't available with all instance types. Additional usage charges apply when using an EBS- optimized instance.</p>
    pub fn set_ebs_optimized(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_ebs_optimized(input);
        self
    }
    /// <p> <i>Deprecated.</i> </p>
    pub fn ephemeral_storage(mut self, input: bool) -> Self {
        self.inner = self.inner.ephemeral_storage(input);
        self
    }
    /// <p> <i>Deprecated.</i> </p>
    pub fn set_ephemeral_storage(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_ephemeral_storage(input);
        self
    }
    /// <p>The date and time at which the Capacity Reservation expires. When a Capacity Reservation expires, the reserved capacity is released and you can no longer launch instances into it. The Capacity Reservation's state changes to <code>expired</code> when it reaches its end date and time.</p>
    /// <p>You must provide an <code>EndDate</code> value if <code>EndDateType</code> is <code>limited</code>. Omit <code>EndDate</code> if <code>EndDateType</code> is <code>unlimited</code>.</p>
    /// <p>If the <code>EndDateType</code> is <code>limited</code>, the Capacity Reservation is cancelled within an hour from the specified time. For example, if you specify 5/31/2019, 13:30:55, the Capacity Reservation is guaranteed to end between 13:30:55 and 14:30:55 on 5/31/2019.</p>
    pub fn end_date(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.end_date(input);
        self
    }
    /// <p>The date and time at which the Capacity Reservation expires. When a Capacity Reservation expires, the reserved capacity is released and you can no longer launch instances into it. The Capacity Reservation's state changes to <code>expired</code> when it reaches its end date and time.</p>
    /// <p>You must provide an <code>EndDate</code> value if <code>EndDateType</code> is <code>limited</code>. Omit <code>EndDate</code> if <code>EndDateType</code> is <code>unlimited</code>.</p>
    /// <p>If the <code>EndDateType</code> is <code>limited</code>, the Capacity Reservation is cancelled within an hour from the specified time. For example, if you specify 5/31/2019, 13:30:55, the Capacity Reservation is guaranteed to end between 13:30:55 and 14:30:55 on 5/31/2019.</p>
    pub fn set_end_date(mut self, input: std::option::Option<aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_end_date(input);
        self
    }
    /// <p>Indicates the way in which the Capacity Reservation ends. A Capacity Reservation can have one of the following end types:</p>
    /// <ul>
    /// <li> <p> <code>unlimited</code> - The Capacity Reservation remains active until you explicitly cancel it. Do not provide an <code>EndDate</code> if the <code>EndDateType</code> is <code>unlimited</code>.</p> </li>
    /// <li> <p> <code>limited</code> - The Capacity Reservation expires automatically at a specified date and time. You must provide an <code>EndDate</code> value if the <code>EndDateType</code> value is <code>limited</code>.</p> </li>
    /// </ul>
    pub fn end_date_type(mut self, input: crate::types::EndDateType) -> Self {
        self.inner = self.inner.end_date_type(input);
        self
    }
    /// <p>Indicates the way in which the Capacity Reservation ends. A Capacity Reservation can have one of the following end types:</p>
    /// <ul>
    /// <li> <p> <code>unlimited</code> - The Capacity Reservation remains active until you explicitly cancel it. Do not provide an <code>EndDate</code> if the <code>EndDateType</code> is <code>unlimited</code>.</p> </li>
    /// <li> <p> <code>limited</code> - The Capacity Reservation expires automatically at a specified date and time. You must provide an <code>EndDate</code> value if the <code>EndDateType</code> value is <code>limited</code>.</p> </li>
    /// </ul>
    pub fn set_end_date_type(
        mut self,
        input: std::option::Option<crate::types::EndDateType>,
    ) -> Self {
        self.inner = self.inner.set_end_date_type(input);
        self
    }
    /// <p>Indicates the type of instance launches that the Capacity Reservation accepts. The options include:</p>
    /// <ul>
    /// <li> <p> <code>open</code> - The Capacity Reservation automatically matches all instances that have matching attributes (instance type, platform, and Availability Zone). Instances that have matching attributes run in the Capacity Reservation automatically without specifying any additional parameters.</p> </li>
    /// <li> <p> <code>targeted</code> - The Capacity Reservation only accepts instances that have matching attributes (instance type, platform, and Availability Zone), and explicitly target the Capacity Reservation. This ensures that only permitted instances can use the reserved capacity. </p> </li>
    /// </ul>
    /// <p>Default: <code>open</code> </p>
    pub fn instance_match_criteria(mut self, input: crate::types::InstanceMatchCriteria) -> Self {
        self.inner = self.inner.instance_match_criteria(input);
        self
    }
    /// <p>Indicates the type of instance launches that the Capacity Reservation accepts. The options include:</p>
    /// <ul>
    /// <li> <p> <code>open</code> - The Capacity Reservation automatically matches all instances that have matching attributes (instance type, platform, and Availability Zone). Instances that have matching attributes run in the Capacity Reservation automatically without specifying any additional parameters.</p> </li>
    /// <li> <p> <code>targeted</code> - The Capacity Reservation only accepts instances that have matching attributes (instance type, platform, and Availability Zone), and explicitly target the Capacity Reservation. This ensures that only permitted instances can use the reserved capacity. </p> </li>
    /// </ul>
    /// <p>Default: <code>open</code> </p>
    pub fn set_instance_match_criteria(
        mut self,
        input: std::option::Option<crate::types::InstanceMatchCriteria>,
    ) -> Self {
        self.inner = self.inner.set_instance_match_criteria(input);
        self
    }
    /// Appends an item to `TagSpecifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The tags to apply to the Capacity Reservation during launch.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        self.inner = self.inner.tag_specifications(input);
        self
    }
    /// <p>The tags to apply to the Capacity Reservation during launch.</p>
    pub fn set_tag_specifications(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::TagSpecification>>,
    ) -> Self {
        self.inner = self.inner.set_tag_specifications(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Outpost on which to create the Capacity Reservation.</p>
    pub fn outpost_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.outpost_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Outpost on which to create the Capacity Reservation.</p>
    pub fn set_outpost_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_outpost_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the cluster placement group in which to create the Capacity Reservation. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/cr-cpg.html"> Capacity Reservations for cluster placement groups</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn placement_group_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.placement_group_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the cluster placement group in which to create the Capacity Reservation. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/cr-cpg.html"> Capacity Reservations for cluster placement groups</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn set_placement_group_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_placement_group_arn(input);
        self
    }
}
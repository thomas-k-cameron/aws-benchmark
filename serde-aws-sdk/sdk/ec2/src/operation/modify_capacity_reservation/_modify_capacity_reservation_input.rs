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
pub struct ModifyCapacityReservationInput {
    /// <p>The ID of the Capacity Reservation.</p>
    #[doc(hidden)]
    pub capacity_reservation_id: std::option::Option<std::string::String>,
    /// <p>The number of instances for which to reserve capacity. The number of instances can't be increased or decreased by more than <code>1000</code> in a single request.</p>
    #[doc(hidden)]
    pub instance_count: std::option::Option<i32>,
    /// <p>The date and time at which the Capacity Reservation expires. When a Capacity Reservation expires, the reserved capacity is released and you can no longer launch instances into it. The Capacity Reservation's state changes to <code>expired</code> when it reaches its end date and time.</p>
    /// <p>The Capacity Reservation is cancelled within an hour from the specified time. For example, if you specify 5/31/2019, 13:30:55, the Capacity Reservation is guaranteed to end between 13:30:55 and 14:30:55 on 5/31/2019.</p>
    /// <p>You must provide an <code>EndDate</code> value if <code>EndDateType</code> is <code>limited</code>. Omit <code>EndDate</code> if <code>EndDateType</code> is <code>unlimited</code>.</p>
    #[doc(hidden)]
    pub end_date: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>Indicates the way in which the Capacity Reservation ends. A Capacity Reservation can have one of the following end types:</p>
    /// <ul>
    /// <li> <p> <code>unlimited</code> - The Capacity Reservation remains active until you explicitly cancel it. Do not provide an <code>EndDate</code> value if <code>EndDateType</code> is <code>unlimited</code>.</p> </li>
    /// <li> <p> <code>limited</code> - The Capacity Reservation expires automatically at a specified date and time. You must provide an <code>EndDate</code> value if <code>EndDateType</code> is <code>limited</code>.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub end_date_type: std::option::Option<crate::types::EndDateType>,
    /// <p>Reserved. Capacity Reservations you have created are accepted by default.</p>
    #[doc(hidden)]
    pub accept: std::option::Option<bool>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
    /// <p>Reserved for future use.</p>
    #[doc(hidden)]
    pub additional_info: std::option::Option<std::string::String>,
}
impl ModifyCapacityReservationInput {
    /// <p>The ID of the Capacity Reservation.</p>
    pub fn capacity_reservation_id(&self) -> std::option::Option<&str> {
        self.capacity_reservation_id.as_deref()
    }
    /// <p>The number of instances for which to reserve capacity. The number of instances can't be increased or decreased by more than <code>1000</code> in a single request.</p>
    pub fn instance_count(&self) -> std::option::Option<i32> {
        self.instance_count
    }
    /// <p>The date and time at which the Capacity Reservation expires. When a Capacity Reservation expires, the reserved capacity is released and you can no longer launch instances into it. The Capacity Reservation's state changes to <code>expired</code> when it reaches its end date and time.</p>
    /// <p>The Capacity Reservation is cancelled within an hour from the specified time. For example, if you specify 5/31/2019, 13:30:55, the Capacity Reservation is guaranteed to end between 13:30:55 and 14:30:55 on 5/31/2019.</p>
    /// <p>You must provide an <code>EndDate</code> value if <code>EndDateType</code> is <code>limited</code>. Omit <code>EndDate</code> if <code>EndDateType</code> is <code>unlimited</code>.</p>
    pub fn end_date(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.end_date.as_ref()
    }
    /// <p>Indicates the way in which the Capacity Reservation ends. A Capacity Reservation can have one of the following end types:</p>
    /// <ul>
    /// <li> <p> <code>unlimited</code> - The Capacity Reservation remains active until you explicitly cancel it. Do not provide an <code>EndDate</code> value if <code>EndDateType</code> is <code>unlimited</code>.</p> </li>
    /// <li> <p> <code>limited</code> - The Capacity Reservation expires automatically at a specified date and time. You must provide an <code>EndDate</code> value if <code>EndDateType</code> is <code>limited</code>.</p> </li>
    /// </ul>
    pub fn end_date_type(&self) -> std::option::Option<&crate::types::EndDateType> {
        self.end_date_type.as_ref()
    }
    /// <p>Reserved. Capacity Reservations you have created are accepted by default.</p>
    pub fn accept(&self) -> std::option::Option<bool> {
        self.accept
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
    /// <p>Reserved for future use.</p>
    pub fn additional_info(&self) -> std::option::Option<&str> {
        self.additional_info.as_deref()
    }
}
impl ModifyCapacityReservationInput {
    /// Creates a new builder-style object to manufacture [`ModifyCapacityReservationInput`](crate::operation::modify_capacity_reservation::ModifyCapacityReservationInput).
    pub fn builder() -> crate::operation::modify_capacity_reservation::builders::ModifyCapacityReservationInputBuilder{
        crate::operation::modify_capacity_reservation::builders::ModifyCapacityReservationInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::modify_capacity_reservation::ModifyCapacityReservationInput;
/// A builder for [`ModifyCapacityReservationInput`](crate::operation::modify_capacity_reservation::ModifyCapacityReservationInput).
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
pub struct ModifyCapacityReservationInputBuilder {
    pub(crate) capacity_reservation_id: std::option::Option<std::string::String>,
    pub(crate) instance_count: std::option::Option<i32>,
    pub(crate) end_date: std::option::Option<aws_smithy_types::DateTime>,
    pub(crate) end_date_type: std::option::Option<crate::types::EndDateType>,
    pub(crate) accept: std::option::Option<bool>,
    pub(crate) dry_run: std::option::Option<bool>,
    pub(crate) additional_info: std::option::Option<std::string::String>,
}
impl ModifyCapacityReservationInputBuilder {
    /// <p>The ID of the Capacity Reservation.</p>
    pub fn capacity_reservation_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.capacity_reservation_id = Some(input.into());
        self
    }
    /// <p>The ID of the Capacity Reservation.</p>
    pub fn set_capacity_reservation_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.capacity_reservation_id = input;
        self
    }
    /// <p>The number of instances for which to reserve capacity. The number of instances can't be increased or decreased by more than <code>1000</code> in a single request.</p>
    pub fn instance_count(mut self, input: i32) -> Self {
        self.instance_count = Some(input);
        self
    }
    /// <p>The number of instances for which to reserve capacity. The number of instances can't be increased or decreased by more than <code>1000</code> in a single request.</p>
    pub fn set_instance_count(mut self, input: std::option::Option<i32>) -> Self {
        self.instance_count = input;
        self
    }
    /// <p>The date and time at which the Capacity Reservation expires. When a Capacity Reservation expires, the reserved capacity is released and you can no longer launch instances into it. The Capacity Reservation's state changes to <code>expired</code> when it reaches its end date and time.</p>
    /// <p>The Capacity Reservation is cancelled within an hour from the specified time. For example, if you specify 5/31/2019, 13:30:55, the Capacity Reservation is guaranteed to end between 13:30:55 and 14:30:55 on 5/31/2019.</p>
    /// <p>You must provide an <code>EndDate</code> value if <code>EndDateType</code> is <code>limited</code>. Omit <code>EndDate</code> if <code>EndDateType</code> is <code>unlimited</code>.</p>
    pub fn end_date(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.end_date = Some(input);
        self
    }
    /// <p>The date and time at which the Capacity Reservation expires. When a Capacity Reservation expires, the reserved capacity is released and you can no longer launch instances into it. The Capacity Reservation's state changes to <code>expired</code> when it reaches its end date and time.</p>
    /// <p>The Capacity Reservation is cancelled within an hour from the specified time. For example, if you specify 5/31/2019, 13:30:55, the Capacity Reservation is guaranteed to end between 13:30:55 and 14:30:55 on 5/31/2019.</p>
    /// <p>You must provide an <code>EndDate</code> value if <code>EndDateType</code> is <code>limited</code>. Omit <code>EndDate</code> if <code>EndDateType</code> is <code>unlimited</code>.</p>
    pub fn set_end_date(mut self, input: std::option::Option<aws_smithy_types::DateTime>) -> Self {
        self.end_date = input;
        self
    }
    /// <p>Indicates the way in which the Capacity Reservation ends. A Capacity Reservation can have one of the following end types:</p>
    /// <ul>
    /// <li> <p> <code>unlimited</code> - The Capacity Reservation remains active until you explicitly cancel it. Do not provide an <code>EndDate</code> value if <code>EndDateType</code> is <code>unlimited</code>.</p> </li>
    /// <li> <p> <code>limited</code> - The Capacity Reservation expires automatically at a specified date and time. You must provide an <code>EndDate</code> value if <code>EndDateType</code> is <code>limited</code>.</p> </li>
    /// </ul>
    pub fn end_date_type(mut self, input: crate::types::EndDateType) -> Self {
        self.end_date_type = Some(input);
        self
    }
    /// <p>Indicates the way in which the Capacity Reservation ends. A Capacity Reservation can have one of the following end types:</p>
    /// <ul>
    /// <li> <p> <code>unlimited</code> - The Capacity Reservation remains active until you explicitly cancel it. Do not provide an <code>EndDate</code> value if <code>EndDateType</code> is <code>unlimited</code>.</p> </li>
    /// <li> <p> <code>limited</code> - The Capacity Reservation expires automatically at a specified date and time. You must provide an <code>EndDate</code> value if <code>EndDateType</code> is <code>limited</code>.</p> </li>
    /// </ul>
    pub fn set_end_date_type(
        mut self,
        input: std::option::Option<crate::types::EndDateType>,
    ) -> Self {
        self.end_date_type = input;
        self
    }
    /// <p>Reserved. Capacity Reservations you have created are accepted by default.</p>
    pub fn accept(mut self, input: bool) -> Self {
        self.accept = Some(input);
        self
    }
    /// <p>Reserved. Capacity Reservations you have created are accepted by default.</p>
    pub fn set_accept(mut self, input: std::option::Option<bool>) -> Self {
        self.accept = input;
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
    /// <p>Reserved for future use.</p>
    pub fn additional_info(mut self, input: impl Into<std::string::String>) -> Self {
        self.additional_info = Some(input.into());
        self
    }
    /// <p>Reserved for future use.</p>
    pub fn set_additional_info(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.additional_info = input;
        self
    }
    /// Consumes the builder and constructs a [`ModifyCapacityReservationInput`](crate::operation::modify_capacity_reservation::ModifyCapacityReservationInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::modify_capacity_reservation::ModifyCapacityReservationInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::modify_capacity_reservation::ModifyCapacityReservationInput {
                capacity_reservation_id: self.capacity_reservation_id,
                instance_count: self.instance_count,
                end_date: self.end_date,
                end_date_type: self.end_date_type,
                accept: self.accept,
                dry_run: self.dry_run,
                additional_info: self.additional_info,
            },
        )
    }
}
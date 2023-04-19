// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the strategy for using unused Capacity Reservations for fulfilling On-Demand capacity.</p> <note>
/// <p>This strategy can only be used if the EC2 Fleet is of type <code>instant</code>.</p>
/// </note>
/// <p>For more information about Capacity Reservations, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-capacity-reservations.html">On-Demand Capacity Reservations</a> in the <i>Amazon EC2 User Guide</i>. For examples of using Capacity Reservations in an EC2 Fleet, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-fleet-examples.html">EC2 Fleet example configurations</a> in the <i>Amazon EC2 User Guide</i>.</p>
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
pub struct CapacityReservationOptions {
    /// <p>Indicates whether to use unused Capacity Reservations for fulfilling On-Demand capacity.</p>
    /// <p>If you specify <code>use-capacity-reservations-first</code>, the fleet uses unused Capacity Reservations to fulfill On-Demand capacity up to the target On-Demand capacity. If multiple instance pools have unused Capacity Reservations, the On-Demand allocation strategy (<code>lowest-price</code> or <code>prioritized</code>) is applied. If the number of unused Capacity Reservations is less than the On-Demand target capacity, the remaining On-Demand target capacity is launched according to the On-Demand allocation strategy (<code>lowest-price</code> or <code>prioritized</code>).</p>
    /// <p>If you do not specify a value, the fleet fulfils the On-Demand capacity according to the chosen On-Demand allocation strategy.</p>
    #[doc(hidden)]
    pub usage_strategy: std::option::Option<crate::types::FleetCapacityReservationUsageStrategy>,
}
impl CapacityReservationOptions {
    /// <p>Indicates whether to use unused Capacity Reservations for fulfilling On-Demand capacity.</p>
    /// <p>If you specify <code>use-capacity-reservations-first</code>, the fleet uses unused Capacity Reservations to fulfill On-Demand capacity up to the target On-Demand capacity. If multiple instance pools have unused Capacity Reservations, the On-Demand allocation strategy (<code>lowest-price</code> or <code>prioritized</code>) is applied. If the number of unused Capacity Reservations is less than the On-Demand target capacity, the remaining On-Demand target capacity is launched according to the On-Demand allocation strategy (<code>lowest-price</code> or <code>prioritized</code>).</p>
    /// <p>If you do not specify a value, the fleet fulfils the On-Demand capacity according to the chosen On-Demand allocation strategy.</p>
    pub fn usage_strategy(
        &self,
    ) -> std::option::Option<&crate::types::FleetCapacityReservationUsageStrategy> {
        self.usage_strategy.as_ref()
    }
}
impl CapacityReservationOptions {
    /// Creates a new builder-style object to manufacture [`CapacityReservationOptions`](crate::types::CapacityReservationOptions).
    pub fn builder() -> crate::types::builders::CapacityReservationOptionsBuilder {
        crate::types::builders::CapacityReservationOptionsBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::CapacityReservationOptions;
/// A builder for [`CapacityReservationOptions`](crate::types::CapacityReservationOptions).
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
pub struct CapacityReservationOptionsBuilder {
    pub(crate) usage_strategy:
        std::option::Option<crate::types::FleetCapacityReservationUsageStrategy>,
}
impl CapacityReservationOptionsBuilder {
    /// <p>Indicates whether to use unused Capacity Reservations for fulfilling On-Demand capacity.</p>
    /// <p>If you specify <code>use-capacity-reservations-first</code>, the fleet uses unused Capacity Reservations to fulfill On-Demand capacity up to the target On-Demand capacity. If multiple instance pools have unused Capacity Reservations, the On-Demand allocation strategy (<code>lowest-price</code> or <code>prioritized</code>) is applied. If the number of unused Capacity Reservations is less than the On-Demand target capacity, the remaining On-Demand target capacity is launched according to the On-Demand allocation strategy (<code>lowest-price</code> or <code>prioritized</code>).</p>
    /// <p>If you do not specify a value, the fleet fulfils the On-Demand capacity according to the chosen On-Demand allocation strategy.</p>
    pub fn usage_strategy(
        mut self,
        input: crate::types::FleetCapacityReservationUsageStrategy,
    ) -> Self {
        self.usage_strategy = Some(input);
        self
    }
    /// <p>Indicates whether to use unused Capacity Reservations for fulfilling On-Demand capacity.</p>
    /// <p>If you specify <code>use-capacity-reservations-first</code>, the fleet uses unused Capacity Reservations to fulfill On-Demand capacity up to the target On-Demand capacity. If multiple instance pools have unused Capacity Reservations, the On-Demand allocation strategy (<code>lowest-price</code> or <code>prioritized</code>) is applied. If the number of unused Capacity Reservations is less than the On-Demand target capacity, the remaining On-Demand target capacity is launched according to the On-Demand allocation strategy (<code>lowest-price</code> or <code>prioritized</code>).</p>
    /// <p>If you do not specify a value, the fleet fulfils the On-Demand capacity according to the chosen On-Demand allocation strategy.</p>
    pub fn set_usage_strategy(
        mut self,
        input: std::option::Option<crate::types::FleetCapacityReservationUsageStrategy>,
    ) -> Self {
        self.usage_strategy = input;
        self
    }
    /// Consumes the builder and constructs a [`CapacityReservationOptions`](crate::types::CapacityReservationOptions).
    pub fn build(self) -> crate::types::CapacityReservationOptions {
        crate::types::CapacityReservationOptions {
            usage_strategy: self.usage_strategy,
        }
    }
}
// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PurchaseReservedInstancesOffering`](crate::operation::purchase_reserved_instances_offering::builders::PurchaseReservedInstancesOfferingFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`instance_count(i32)`](crate::operation::purchase_reserved_instances_offering::builders::PurchaseReservedInstancesOfferingFluentBuilder::instance_count) / [`set_instance_count(Option<i32>)`](crate::operation::purchase_reserved_instances_offering::builders::PurchaseReservedInstancesOfferingFluentBuilder::set_instance_count): <p>The number of Reserved Instances to purchase.</p>
    ///   - [`reserved_instances_offering_id(impl Into<String>)`](crate::operation::purchase_reserved_instances_offering::builders::PurchaseReservedInstancesOfferingFluentBuilder::reserved_instances_offering_id) / [`set_reserved_instances_offering_id(Option<String>)`](crate::operation::purchase_reserved_instances_offering::builders::PurchaseReservedInstancesOfferingFluentBuilder::set_reserved_instances_offering_id): <p>The ID of the Reserved Instance offering to purchase.</p>
    ///   - [`dry_run(bool)`](crate::operation::purchase_reserved_instances_offering::builders::PurchaseReservedInstancesOfferingFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::purchase_reserved_instances_offering::builders::PurchaseReservedInstancesOfferingFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`limit_price(ReservedInstanceLimitPrice)`](crate::operation::purchase_reserved_instances_offering::builders::PurchaseReservedInstancesOfferingFluentBuilder::limit_price) / [`set_limit_price(Option<ReservedInstanceLimitPrice>)`](crate::operation::purchase_reserved_instances_offering::builders::PurchaseReservedInstancesOfferingFluentBuilder::set_limit_price): <p>Specified for Reserved Instance Marketplace offerings to limit the total order and ensure that the Reserved Instances are not purchased at unexpected prices.</p>
    ///   - [`purchase_time(DateTime)`](crate::operation::purchase_reserved_instances_offering::builders::PurchaseReservedInstancesOfferingFluentBuilder::purchase_time) / [`set_purchase_time(Option<DateTime>)`](crate::operation::purchase_reserved_instances_offering::builders::PurchaseReservedInstancesOfferingFluentBuilder::set_purchase_time): <p>The time at which to purchase the Reserved Instance, in UTC format (for example, <i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z).</p>
    /// - On success, responds with [`PurchaseReservedInstancesOfferingOutput`](crate::operation::purchase_reserved_instances_offering::PurchaseReservedInstancesOfferingOutput) with field(s):
    ///   - [`reserved_instances_id(Option<String>)`](crate::operation::purchase_reserved_instances_offering::PurchaseReservedInstancesOfferingOutput::reserved_instances_id): <p>The IDs of the purchased Reserved Instances. If your purchase crosses into a discounted pricing tier, the final Reserved Instances IDs might change. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/concepts-reserved-instances-application.html#crossing-pricing-tiers">Crossing pricing tiers</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    /// - On failure, responds with [`SdkError<PurchaseReservedInstancesOfferingError>`](crate::operation::purchase_reserved_instances_offering::PurchaseReservedInstancesOfferingError)
    pub fn purchase_reserved_instances_offering(&self) -> crate::operation::purchase_reserved_instances_offering::builders::PurchaseReservedInstancesOfferingFluentBuilder{
        crate::operation::purchase_reserved_instances_offering::builders::PurchaseReservedInstancesOfferingFluentBuilder::new(self.handle.clone())
    }
}
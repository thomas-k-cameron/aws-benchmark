// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ModifyTrafficMirrorFilterRule`](crate::operation::modify_traffic_mirror_filter_rule::builders::ModifyTrafficMirrorFilterRuleFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`traffic_mirror_filter_rule_id(impl Into<String>)`](crate::operation::modify_traffic_mirror_filter_rule::builders::ModifyTrafficMirrorFilterRuleFluentBuilder::traffic_mirror_filter_rule_id) / [`set_traffic_mirror_filter_rule_id(Option<String>)`](crate::operation::modify_traffic_mirror_filter_rule::builders::ModifyTrafficMirrorFilterRuleFluentBuilder::set_traffic_mirror_filter_rule_id): <p>The ID of the Traffic Mirror rule.</p>
    ///   - [`traffic_direction(TrafficDirection)`](crate::operation::modify_traffic_mirror_filter_rule::builders::ModifyTrafficMirrorFilterRuleFluentBuilder::traffic_direction) / [`set_traffic_direction(Option<TrafficDirection>)`](crate::operation::modify_traffic_mirror_filter_rule::builders::ModifyTrafficMirrorFilterRuleFluentBuilder::set_traffic_direction): <p>The type of traffic to assign to the rule.</p>
    ///   - [`rule_number(i32)`](crate::operation::modify_traffic_mirror_filter_rule::builders::ModifyTrafficMirrorFilterRuleFluentBuilder::rule_number) / [`set_rule_number(Option<i32>)`](crate::operation::modify_traffic_mirror_filter_rule::builders::ModifyTrafficMirrorFilterRuleFluentBuilder::set_rule_number): <p>The number of the Traffic Mirror rule. This number must be unique for each Traffic Mirror rule in a given direction. The rules are processed in ascending order by rule number.</p>
    ///   - [`rule_action(TrafficMirrorRuleAction)`](crate::operation::modify_traffic_mirror_filter_rule::builders::ModifyTrafficMirrorFilterRuleFluentBuilder::rule_action) / [`set_rule_action(Option<TrafficMirrorRuleAction>)`](crate::operation::modify_traffic_mirror_filter_rule::builders::ModifyTrafficMirrorFilterRuleFluentBuilder::set_rule_action): <p>The action to assign to the rule.</p>
    ///   - [`destination_port_range(TrafficMirrorPortRangeRequest)`](crate::operation::modify_traffic_mirror_filter_rule::builders::ModifyTrafficMirrorFilterRuleFluentBuilder::destination_port_range) / [`set_destination_port_range(Option<TrafficMirrorPortRangeRequest>)`](crate::operation::modify_traffic_mirror_filter_rule::builders::ModifyTrafficMirrorFilterRuleFluentBuilder::set_destination_port_range): <p>The destination ports that are associated with the Traffic Mirror rule.</p>
    ///   - [`source_port_range(TrafficMirrorPortRangeRequest)`](crate::operation::modify_traffic_mirror_filter_rule::builders::ModifyTrafficMirrorFilterRuleFluentBuilder::source_port_range) / [`set_source_port_range(Option<TrafficMirrorPortRangeRequest>)`](crate::operation::modify_traffic_mirror_filter_rule::builders::ModifyTrafficMirrorFilterRuleFluentBuilder::set_source_port_range): <p>The port range to assign to the Traffic Mirror rule.</p>
    ///   - [`protocol(i32)`](crate::operation::modify_traffic_mirror_filter_rule::builders::ModifyTrafficMirrorFilterRuleFluentBuilder::protocol) / [`set_protocol(Option<i32>)`](crate::operation::modify_traffic_mirror_filter_rule::builders::ModifyTrafficMirrorFilterRuleFluentBuilder::set_protocol): <p>The protocol, for example TCP, to assign to the Traffic Mirror rule.</p>
    ///   - [`destination_cidr_block(impl Into<String>)`](crate::operation::modify_traffic_mirror_filter_rule::builders::ModifyTrafficMirrorFilterRuleFluentBuilder::destination_cidr_block) / [`set_destination_cidr_block(Option<String>)`](crate::operation::modify_traffic_mirror_filter_rule::builders::ModifyTrafficMirrorFilterRuleFluentBuilder::set_destination_cidr_block): <p>The destination CIDR block to assign to the Traffic Mirror rule.</p>
    ///   - [`source_cidr_block(impl Into<String>)`](crate::operation::modify_traffic_mirror_filter_rule::builders::ModifyTrafficMirrorFilterRuleFluentBuilder::source_cidr_block) / [`set_source_cidr_block(Option<String>)`](crate::operation::modify_traffic_mirror_filter_rule::builders::ModifyTrafficMirrorFilterRuleFluentBuilder::set_source_cidr_block): <p>The source CIDR block to assign to the Traffic Mirror rule.</p>
    ///   - [`description(impl Into<String>)`](crate::operation::modify_traffic_mirror_filter_rule::builders::ModifyTrafficMirrorFilterRuleFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::modify_traffic_mirror_filter_rule::builders::ModifyTrafficMirrorFilterRuleFluentBuilder::set_description): <p>The description to assign to the Traffic Mirror rule.</p>
    ///   - [`remove_fields(Vec<TrafficMirrorFilterRuleField>)`](crate::operation::modify_traffic_mirror_filter_rule::builders::ModifyTrafficMirrorFilterRuleFluentBuilder::remove_fields) / [`set_remove_fields(Option<Vec<TrafficMirrorFilterRuleField>>)`](crate::operation::modify_traffic_mirror_filter_rule::builders::ModifyTrafficMirrorFilterRuleFluentBuilder::set_remove_fields): <p>The properties that you want to remove from the Traffic Mirror filter rule.</p>  <p>When you remove a property from a Traffic Mirror filter rule, the property is set to the default.</p>
    ///   - [`dry_run(bool)`](crate::operation::modify_traffic_mirror_filter_rule::builders::ModifyTrafficMirrorFilterRuleFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::modify_traffic_mirror_filter_rule::builders::ModifyTrafficMirrorFilterRuleFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`ModifyTrafficMirrorFilterRuleOutput`](crate::operation::modify_traffic_mirror_filter_rule::ModifyTrafficMirrorFilterRuleOutput) with field(s):
    ///   - [`traffic_mirror_filter_rule(Option<TrafficMirrorFilterRule>)`](crate::operation::modify_traffic_mirror_filter_rule::ModifyTrafficMirrorFilterRuleOutput::traffic_mirror_filter_rule): <p>Modifies a Traffic Mirror rule.</p>
    /// - On failure, responds with [`SdkError<ModifyTrafficMirrorFilterRuleError>`](crate::operation::modify_traffic_mirror_filter_rule::ModifyTrafficMirrorFilterRuleError)
    pub fn modify_traffic_mirror_filter_rule(&self) -> crate::operation::modify_traffic_mirror_filter_rule::builders::ModifyTrafficMirrorFilterRuleFluentBuilder{
        crate::operation::modify_traffic_mirror_filter_rule::builders::ModifyTrafficMirrorFilterRuleFluentBuilder::new(self.handle.clone())
    }
}
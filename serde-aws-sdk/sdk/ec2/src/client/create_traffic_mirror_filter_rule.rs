// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateTrafficMirrorFilterRule`](crate::operation::create_traffic_mirror_filter_rule::builders::CreateTrafficMirrorFilterRuleFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`traffic_mirror_filter_id(impl Into<String>)`](crate::operation::create_traffic_mirror_filter_rule::builders::CreateTrafficMirrorFilterRuleFluentBuilder::traffic_mirror_filter_id) / [`set_traffic_mirror_filter_id(Option<String>)`](crate::operation::create_traffic_mirror_filter_rule::builders::CreateTrafficMirrorFilterRuleFluentBuilder::set_traffic_mirror_filter_id): <p>The ID of the filter that this rule is associated with.</p>
    ///   - [`traffic_direction(TrafficDirection)`](crate::operation::create_traffic_mirror_filter_rule::builders::CreateTrafficMirrorFilterRuleFluentBuilder::traffic_direction) / [`set_traffic_direction(Option<TrafficDirection>)`](crate::operation::create_traffic_mirror_filter_rule::builders::CreateTrafficMirrorFilterRuleFluentBuilder::set_traffic_direction): <p>The type of traffic.</p>
    ///   - [`rule_number(i32)`](crate::operation::create_traffic_mirror_filter_rule::builders::CreateTrafficMirrorFilterRuleFluentBuilder::rule_number) / [`set_rule_number(Option<i32>)`](crate::operation::create_traffic_mirror_filter_rule::builders::CreateTrafficMirrorFilterRuleFluentBuilder::set_rule_number): <p>The number of the Traffic Mirror rule. This number must be unique for each Traffic Mirror rule in a given direction. The rules are processed in ascending order by rule number.</p>
    ///   - [`rule_action(TrafficMirrorRuleAction)`](crate::operation::create_traffic_mirror_filter_rule::builders::CreateTrafficMirrorFilterRuleFluentBuilder::rule_action) / [`set_rule_action(Option<TrafficMirrorRuleAction>)`](crate::operation::create_traffic_mirror_filter_rule::builders::CreateTrafficMirrorFilterRuleFluentBuilder::set_rule_action): <p>The action to take on the filtered traffic.</p>
    ///   - [`destination_port_range(TrafficMirrorPortRangeRequest)`](crate::operation::create_traffic_mirror_filter_rule::builders::CreateTrafficMirrorFilterRuleFluentBuilder::destination_port_range) / [`set_destination_port_range(Option<TrafficMirrorPortRangeRequest>)`](crate::operation::create_traffic_mirror_filter_rule::builders::CreateTrafficMirrorFilterRuleFluentBuilder::set_destination_port_range): <p>The destination port range.</p>
    ///   - [`source_port_range(TrafficMirrorPortRangeRequest)`](crate::operation::create_traffic_mirror_filter_rule::builders::CreateTrafficMirrorFilterRuleFluentBuilder::source_port_range) / [`set_source_port_range(Option<TrafficMirrorPortRangeRequest>)`](crate::operation::create_traffic_mirror_filter_rule::builders::CreateTrafficMirrorFilterRuleFluentBuilder::set_source_port_range): <p>The source port range.</p>
    ///   - [`protocol(i32)`](crate::operation::create_traffic_mirror_filter_rule::builders::CreateTrafficMirrorFilterRuleFluentBuilder::protocol) / [`set_protocol(Option<i32>)`](crate::operation::create_traffic_mirror_filter_rule::builders::CreateTrafficMirrorFilterRuleFluentBuilder::set_protocol): <p>The protocol, for example UDP, to assign to the Traffic Mirror rule.</p>  <p>For information about the protocol value, see <a href="https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml">Protocol Numbers</a> on the Internet Assigned Numbers Authority (IANA) website.</p>
    ///   - [`destination_cidr_block(impl Into<String>)`](crate::operation::create_traffic_mirror_filter_rule::builders::CreateTrafficMirrorFilterRuleFluentBuilder::destination_cidr_block) / [`set_destination_cidr_block(Option<String>)`](crate::operation::create_traffic_mirror_filter_rule::builders::CreateTrafficMirrorFilterRuleFluentBuilder::set_destination_cidr_block): <p>The destination CIDR block to assign to the Traffic Mirror rule.</p>
    ///   - [`source_cidr_block(impl Into<String>)`](crate::operation::create_traffic_mirror_filter_rule::builders::CreateTrafficMirrorFilterRuleFluentBuilder::source_cidr_block) / [`set_source_cidr_block(Option<String>)`](crate::operation::create_traffic_mirror_filter_rule::builders::CreateTrafficMirrorFilterRuleFluentBuilder::set_source_cidr_block): <p>The source CIDR block to assign to the Traffic Mirror rule.</p>
    ///   - [`description(impl Into<String>)`](crate::operation::create_traffic_mirror_filter_rule::builders::CreateTrafficMirrorFilterRuleFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_traffic_mirror_filter_rule::builders::CreateTrafficMirrorFilterRuleFluentBuilder::set_description): <p>The description of the Traffic Mirror rule.</p>
    ///   - [`dry_run(bool)`](crate::operation::create_traffic_mirror_filter_rule::builders::CreateTrafficMirrorFilterRuleFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::create_traffic_mirror_filter_rule::builders::CreateTrafficMirrorFilterRuleFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`client_token(impl Into<String>)`](crate::operation::create_traffic_mirror_filter_rule::builders::CreateTrafficMirrorFilterRuleFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::create_traffic_mirror_filter_rule::builders::CreateTrafficMirrorFilterRuleFluentBuilder::set_client_token): <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    /// - On success, responds with [`CreateTrafficMirrorFilterRuleOutput`](crate::operation::create_traffic_mirror_filter_rule::CreateTrafficMirrorFilterRuleOutput) with field(s):
    ///   - [`traffic_mirror_filter_rule(Option<TrafficMirrorFilterRule>)`](crate::operation::create_traffic_mirror_filter_rule::CreateTrafficMirrorFilterRuleOutput::traffic_mirror_filter_rule): <p>The Traffic Mirror rule.</p>
    ///   - [`client_token(Option<String>)`](crate::operation::create_traffic_mirror_filter_rule::CreateTrafficMirrorFilterRuleOutput::client_token): <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    /// - On failure, responds with [`SdkError<CreateTrafficMirrorFilterRuleError>`](crate::operation::create_traffic_mirror_filter_rule::CreateTrafficMirrorFilterRuleError)
    pub fn create_traffic_mirror_filter_rule(&self) -> crate::operation::create_traffic_mirror_filter_rule::builders::CreateTrafficMirrorFilterRuleFluentBuilder{
        crate::operation::create_traffic_mirror_filter_rule::builders::CreateTrafficMirrorFilterRuleFluentBuilder::new(self.handle.clone())
    }
}
// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_traffic_mirror_filter_rule_list(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<
    std::vec::Vec<crate::types::TrafficMirrorFilterRule>,
    aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut out = std::vec::Vec::new();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("item") /* member com.amazonaws.ec2#TrafficMirrorFilterRuleList$member */ =>  {
                out.push(
                    crate::protocol_serde::shape_traffic_mirror_filter_rule::de_traffic_mirror_filter_rule(&mut tag)
                    ?
                );
            }
            ,
            _ => {}
        }
    }
    Ok(out)
}
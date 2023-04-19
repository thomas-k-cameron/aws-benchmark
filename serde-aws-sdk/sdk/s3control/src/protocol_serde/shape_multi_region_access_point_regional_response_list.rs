// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_multi_region_access_point_regional_response_list(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<
    std::vec::Vec<crate::types::MultiRegionAccessPointRegionalResponse>,
    aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut out = std::vec::Vec::new();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Region") /* member com.amazonaws.s3control#MultiRegionAccessPointRegionalResponseList$member */ =>  {
                out.push(
                    crate::protocol_serde::shape_multi_region_access_point_regional_response::de_multi_region_access_point_regional_response(&mut tag)
                    ?
                );
            }
            ,
            _ => {}
        }
    }
    Ok(out)
}
// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_multi_region_access_points_async_response(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<
    crate::types::MultiRegionAccessPointsAsyncResponse,
    aws_smithy_xml::decode::XmlDecodeError,
> {
    #[allow(unused_mut)]
    let mut builder = crate::types::MultiRegionAccessPointsAsyncResponse::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Regions") /* Regions com.amazonaws.s3control#MultiRegionAccessPointsAsyncResponse$Regions */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_multi_region_access_point_regional_response_list::de_multi_region_access_point_regional_response_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_regions(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
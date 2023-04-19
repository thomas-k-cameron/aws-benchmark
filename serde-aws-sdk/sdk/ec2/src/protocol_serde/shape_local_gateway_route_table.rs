// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_local_gateway_route_table(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::LocalGatewayRouteTable, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::LocalGatewayRouteTable::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("localGatewayRouteTableId") /* LocalGatewayRouteTableId com.amazonaws.ec2#LocalGatewayRouteTable$LocalGatewayRouteTableId */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_local_gateway_route_table_id(var_1);
            }
            ,
            s if s.matches("localGatewayRouteTableArn") /* LocalGatewayRouteTableArn com.amazonaws.ec2#LocalGatewayRouteTable$LocalGatewayRouteTableArn */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_local_gateway_route_table_arn(var_2);
            }
            ,
            s if s.matches("localGatewayId") /* LocalGatewayId com.amazonaws.ec2#LocalGatewayRouteTable$LocalGatewayId */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_local_gateway_id(var_3);
            }
            ,
            s if s.matches("outpostArn") /* OutpostArn com.amazonaws.ec2#LocalGatewayRouteTable$OutpostArn */ =>  {
                let var_4 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_outpost_arn(var_4);
            }
            ,
            s if s.matches("ownerId") /* OwnerId com.amazonaws.ec2#LocalGatewayRouteTable$OwnerId */ =>  {
                let var_5 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_owner_id(var_5);
            }
            ,
            s if s.matches("state") /* State com.amazonaws.ec2#LocalGatewayRouteTable$State */ =>  {
                let var_6 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_state(var_6);
            }
            ,
            s if s.matches("tagSet") /* Tags com.amazonaws.ec2#LocalGatewayRouteTable$Tags */ =>  {
                let var_7 =
                    Some(
                        crate::protocol_serde::shape_tag_list::de_tag_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tags(var_7);
            }
            ,
            s if s.matches("mode") /* Mode com.amazonaws.ec2#LocalGatewayRouteTable$Mode */ =>  {
                let var_8 =
                    Some(
                        Result::<crate::types::LocalGatewayRouteTableMode, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::LocalGatewayRouteTableMode::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_mode(var_8);
            }
            ,
            s if s.matches("stateReason") /* StateReason com.amazonaws.ec2#LocalGatewayRouteTable$StateReason */ =>  {
                let var_9 =
                    Some(
                        crate::protocol_serde::shape_state_reason::de_state_reason(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_state_reason(var_9);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
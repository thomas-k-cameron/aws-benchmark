// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_transit_gateway_connect_peer(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::TransitGatewayConnectPeer, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::TransitGatewayConnectPeer::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("transitGatewayAttachmentId") /* TransitGatewayAttachmentId com.amazonaws.ec2#TransitGatewayConnectPeer$TransitGatewayAttachmentId */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_transit_gateway_attachment_id(var_1);
            }
            ,
            s if s.matches("transitGatewayConnectPeerId") /* TransitGatewayConnectPeerId com.amazonaws.ec2#TransitGatewayConnectPeer$TransitGatewayConnectPeerId */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_transit_gateway_connect_peer_id(var_2);
            }
            ,
            s if s.matches("state") /* State com.amazonaws.ec2#TransitGatewayConnectPeer$State */ =>  {
                let var_3 =
                    Some(
                        Result::<crate::types::TransitGatewayConnectPeerState, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::TransitGatewayConnectPeerState::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_state(var_3);
            }
            ,
            s if s.matches("creationTime") /* CreationTime com.amazonaws.ec2#TransitGatewayConnectPeer$CreationTime */ =>  {
                let var_4 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.ec2#DateTime`)"))
                        ?
                    )
                ;
                builder = builder.set_creation_time(var_4);
            }
            ,
            s if s.matches("connectPeerConfiguration") /* ConnectPeerConfiguration com.amazonaws.ec2#TransitGatewayConnectPeer$ConnectPeerConfiguration */ =>  {
                let var_5 =
                    Some(
                        crate::protocol_serde::shape_transit_gateway_connect_peer_configuration::de_transit_gateway_connect_peer_configuration(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_connect_peer_configuration(var_5);
            }
            ,
            s if s.matches("tagSet") /* Tags com.amazonaws.ec2#TransitGatewayConnectPeer$Tags */ =>  {
                let var_6 =
                    Some(
                        crate::protocol_serde::shape_tag_list::de_tag_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tags(var_6);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_instance_network_interface(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::InstanceNetworkInterface, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::InstanceNetworkInterface::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("association") /* Association com.amazonaws.ec2#InstanceNetworkInterface$Association */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_instance_network_interface_association::de_instance_network_interface_association(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_association(var_1);
            }
            ,
            s if s.matches("attachment") /* Attachment com.amazonaws.ec2#InstanceNetworkInterface$Attachment */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_instance_network_interface_attachment::de_instance_network_interface_attachment(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_attachment(var_2);
            }
            ,
            s if s.matches("description") /* Description com.amazonaws.ec2#InstanceNetworkInterface$Description */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_description(var_3);
            }
            ,
            s if s.matches("groupSet") /* Groups com.amazonaws.ec2#InstanceNetworkInterface$Groups */ =>  {
                let var_4 =
                    Some(
                        crate::protocol_serde::shape_group_identifier_list::de_group_identifier_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_groups(var_4);
            }
            ,
            s if s.matches("ipv6AddressesSet") /* Ipv6Addresses com.amazonaws.ec2#InstanceNetworkInterface$Ipv6Addresses */ =>  {
                let var_5 =
                    Some(
                        crate::protocol_serde::shape_instance_ipv6_address_list::de_instance_ipv6_address_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_ipv6_addresses(var_5);
            }
            ,
            s if s.matches("macAddress") /* MacAddress com.amazonaws.ec2#InstanceNetworkInterface$MacAddress */ =>  {
                let var_6 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_mac_address(var_6);
            }
            ,
            s if s.matches("networkInterfaceId") /* NetworkInterfaceId com.amazonaws.ec2#InstanceNetworkInterface$NetworkInterfaceId */ =>  {
                let var_7 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_network_interface_id(var_7);
            }
            ,
            s if s.matches("ownerId") /* OwnerId com.amazonaws.ec2#InstanceNetworkInterface$OwnerId */ =>  {
                let var_8 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_owner_id(var_8);
            }
            ,
            s if s.matches("privateDnsName") /* PrivateDnsName com.amazonaws.ec2#InstanceNetworkInterface$PrivateDnsName */ =>  {
                let var_9 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_private_dns_name(var_9);
            }
            ,
            s if s.matches("privateIpAddress") /* PrivateIpAddress com.amazonaws.ec2#InstanceNetworkInterface$PrivateIpAddress */ =>  {
                let var_10 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_private_ip_address(var_10);
            }
            ,
            s if s.matches("privateIpAddressesSet") /* PrivateIpAddresses com.amazonaws.ec2#InstanceNetworkInterface$PrivateIpAddresses */ =>  {
                let var_11 =
                    Some(
                        crate::protocol_serde::shape_instance_private_ip_address_list::de_instance_private_ip_address_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_private_ip_addresses(var_11);
            }
            ,
            s if s.matches("sourceDestCheck") /* SourceDestCheck com.amazonaws.ec2#InstanceNetworkInterface$SourceDestCheck */ =>  {
                let var_12 =
                    Some(
                         {
                            <bool as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.ec2#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_source_dest_check(var_12);
            }
            ,
            s if s.matches("status") /* Status com.amazonaws.ec2#InstanceNetworkInterface$Status */ =>  {
                let var_13 =
                    Some(
                        Result::<crate::types::NetworkInterfaceStatus, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::NetworkInterfaceStatus::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_status(var_13);
            }
            ,
            s if s.matches("subnetId") /* SubnetId com.amazonaws.ec2#InstanceNetworkInterface$SubnetId */ =>  {
                let var_14 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_subnet_id(var_14);
            }
            ,
            s if s.matches("vpcId") /* VpcId com.amazonaws.ec2#InstanceNetworkInterface$VpcId */ =>  {
                let var_15 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_vpc_id(var_15);
            }
            ,
            s if s.matches("interfaceType") /* InterfaceType com.amazonaws.ec2#InstanceNetworkInterface$InterfaceType */ =>  {
                let var_16 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_interface_type(var_16);
            }
            ,
            s if s.matches("ipv4PrefixSet") /* Ipv4Prefixes com.amazonaws.ec2#InstanceNetworkInterface$Ipv4Prefixes */ =>  {
                let var_17 =
                    Some(
                        crate::protocol_serde::shape_instance_ipv4_prefix_list::de_instance_ipv4_prefix_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_ipv4_prefixes(var_17);
            }
            ,
            s if s.matches("ipv6PrefixSet") /* Ipv6Prefixes com.amazonaws.ec2#InstanceNetworkInterface$Ipv6Prefixes */ =>  {
                let var_18 =
                    Some(
                        crate::protocol_serde::shape_instance_ipv6_prefix_list::de_instance_ipv6_prefix_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_ipv6_prefixes(var_18);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
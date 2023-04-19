// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_subnet(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::Subnet, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::Subnet::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("availabilityZone") /* AvailabilityZone com.amazonaws.ec2#Subnet$AvailabilityZone */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_availability_zone(var_1);
            }
            ,
            s if s.matches("availabilityZoneId") /* AvailabilityZoneId com.amazonaws.ec2#Subnet$AvailabilityZoneId */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_availability_zone_id(var_2);
            }
            ,
            s if s.matches("availableIpAddressCount") /* AvailableIpAddressCount com.amazonaws.ec2#Subnet$AvailableIpAddressCount */ =>  {
                let var_3 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#Integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_available_ip_address_count(var_3);
            }
            ,
            s if s.matches("cidrBlock") /* CidrBlock com.amazonaws.ec2#Subnet$CidrBlock */ =>  {
                let var_4 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_cidr_block(var_4);
            }
            ,
            s if s.matches("defaultForAz") /* DefaultForAz com.amazonaws.ec2#Subnet$DefaultForAz */ =>  {
                let var_5 =
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
                builder = builder.set_default_for_az(var_5);
            }
            ,
            s if s.matches("enableLniAtDeviceIndex") /* EnableLniAtDeviceIndex com.amazonaws.ec2#Subnet$EnableLniAtDeviceIndex */ =>  {
                let var_6 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#Integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_enable_lni_at_device_index(var_6);
            }
            ,
            s if s.matches("mapPublicIpOnLaunch") /* MapPublicIpOnLaunch com.amazonaws.ec2#Subnet$MapPublicIpOnLaunch */ =>  {
                let var_7 =
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
                builder = builder.set_map_public_ip_on_launch(var_7);
            }
            ,
            s if s.matches("mapCustomerOwnedIpOnLaunch") /* MapCustomerOwnedIpOnLaunch com.amazonaws.ec2#Subnet$MapCustomerOwnedIpOnLaunch */ =>  {
                let var_8 =
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
                builder = builder.set_map_customer_owned_ip_on_launch(var_8);
            }
            ,
            s if s.matches("customerOwnedIpv4Pool") /* CustomerOwnedIpv4Pool com.amazonaws.ec2#Subnet$CustomerOwnedIpv4Pool */ =>  {
                let var_9 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_customer_owned_ipv4_pool(var_9);
            }
            ,
            s if s.matches("state") /* State com.amazonaws.ec2#Subnet$State */ =>  {
                let var_10 =
                    Some(
                        Result::<crate::types::SubnetState, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::SubnetState::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_state(var_10);
            }
            ,
            s if s.matches("subnetId") /* SubnetId com.amazonaws.ec2#Subnet$SubnetId */ =>  {
                let var_11 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_subnet_id(var_11);
            }
            ,
            s if s.matches("vpcId") /* VpcId com.amazonaws.ec2#Subnet$VpcId */ =>  {
                let var_12 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_vpc_id(var_12);
            }
            ,
            s if s.matches("ownerId") /* OwnerId com.amazonaws.ec2#Subnet$OwnerId */ =>  {
                let var_13 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_owner_id(var_13);
            }
            ,
            s if s.matches("assignIpv6AddressOnCreation") /* AssignIpv6AddressOnCreation com.amazonaws.ec2#Subnet$AssignIpv6AddressOnCreation */ =>  {
                let var_14 =
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
                builder = builder.set_assign_ipv6_address_on_creation(var_14);
            }
            ,
            s if s.matches("ipv6CidrBlockAssociationSet") /* Ipv6CidrBlockAssociationSet com.amazonaws.ec2#Subnet$Ipv6CidrBlockAssociationSet */ =>  {
                let var_15 =
                    Some(
                        crate::protocol_serde::shape_subnet_ipv6_cidr_block_association_set::de_subnet_ipv6_cidr_block_association_set(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_ipv6_cidr_block_association_set(var_15);
            }
            ,
            s if s.matches("tagSet") /* Tags com.amazonaws.ec2#Subnet$Tags */ =>  {
                let var_16 =
                    Some(
                        crate::protocol_serde::shape_tag_list::de_tag_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tags(var_16);
            }
            ,
            s if s.matches("subnetArn") /* SubnetArn com.amazonaws.ec2#Subnet$SubnetArn */ =>  {
                let var_17 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_subnet_arn(var_17);
            }
            ,
            s if s.matches("outpostArn") /* OutpostArn com.amazonaws.ec2#Subnet$OutpostArn */ =>  {
                let var_18 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_outpost_arn(var_18);
            }
            ,
            s if s.matches("enableDns64") /* EnableDns64 com.amazonaws.ec2#Subnet$EnableDns64 */ =>  {
                let var_19 =
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
                builder = builder.set_enable_dns64(var_19);
            }
            ,
            s if s.matches("ipv6Native") /* Ipv6Native com.amazonaws.ec2#Subnet$Ipv6Native */ =>  {
                let var_20 =
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
                builder = builder.set_ipv6_native(var_20);
            }
            ,
            s if s.matches("privateDnsNameOptionsOnLaunch") /* PrivateDnsNameOptionsOnLaunch com.amazonaws.ec2#Subnet$PrivateDnsNameOptionsOnLaunch */ =>  {
                let var_21 =
                    Some(
                        crate::protocol_serde::shape_private_dns_name_options_on_launch::de_private_dns_name_options_on_launch(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_private_dns_name_options_on_launch(var_21);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
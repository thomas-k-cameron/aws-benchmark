// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_resource_record_set(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::ResourceRecordSet, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::ResourceRecordSet::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Name") /* Name com.amazonaws.route53#ResourceRecordSet$Name */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_name(var_1);
            }
            ,
            s if s.matches("Type") /* Type com.amazonaws.route53#ResourceRecordSet$Type */ =>  {
                let var_2 =
                    Some(
                        Result::<crate::types::RrType, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::RrType::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_type(var_2);
            }
            ,
            s if s.matches("SetIdentifier") /* SetIdentifier com.amazonaws.route53#ResourceRecordSet$SetIdentifier */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_set_identifier(var_3);
            }
            ,
            s if s.matches("Weight") /* Weight com.amazonaws.route53#ResourceRecordSet$Weight */ =>  {
                let var_4 =
                    Some(
                         {
                            <i64 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (long: `com.amazonaws.route53#ResourceRecordSetWeight`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_weight(var_4);
            }
            ,
            s if s.matches("Region") /* Region com.amazonaws.route53#ResourceRecordSet$Region */ =>  {
                let var_5 =
                    Some(
                        Result::<crate::types::ResourceRecordSetRegion, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::ResourceRecordSetRegion::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_region(var_5);
            }
            ,
            s if s.matches("GeoLocation") /* GeoLocation com.amazonaws.route53#ResourceRecordSet$GeoLocation */ =>  {
                let var_6 =
                    Some(
                        crate::protocol_serde::shape_geo_location::de_geo_location(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_geo_location(var_6);
            }
            ,
            s if s.matches("Failover") /* Failover com.amazonaws.route53#ResourceRecordSet$Failover */ =>  {
                let var_7 =
                    Some(
                        Result::<crate::types::ResourceRecordSetFailover, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::ResourceRecordSetFailover::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_failover(var_7);
            }
            ,
            s if s.matches("MultiValueAnswer") /* MultiValueAnswer com.amazonaws.route53#ResourceRecordSet$MultiValueAnswer */ =>  {
                let var_8 =
                    Some(
                         {
                            <bool as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.route53#ResourceRecordSetMultiValueAnswer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_multi_value_answer(var_8);
            }
            ,
            s if s.matches("TTL") /* TTL com.amazonaws.route53#ResourceRecordSet$TTL */ =>  {
                let var_9 =
                    Some(
                         {
                            <i64 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (long: `com.amazonaws.route53#TTL`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_ttl(var_9);
            }
            ,
            s if s.matches("ResourceRecords") /* ResourceRecords com.amazonaws.route53#ResourceRecordSet$ResourceRecords */ =>  {
                let var_10 =
                    Some(
                        crate::protocol_serde::shape_resource_records::de_resource_records(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_resource_records(var_10);
            }
            ,
            s if s.matches("AliasTarget") /* AliasTarget com.amazonaws.route53#ResourceRecordSet$AliasTarget */ =>  {
                let var_11 =
                    Some(
                        crate::protocol_serde::shape_alias_target::de_alias_target(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_alias_target(var_11);
            }
            ,
            s if s.matches("HealthCheckId") /* HealthCheckId com.amazonaws.route53#ResourceRecordSet$HealthCheckId */ =>  {
                let var_12 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_health_check_id(var_12);
            }
            ,
            s if s.matches("TrafficPolicyInstanceId") /* TrafficPolicyInstanceId com.amazonaws.route53#ResourceRecordSet$TrafficPolicyInstanceId */ =>  {
                let var_13 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_traffic_policy_instance_id(var_13);
            }
            ,
            s if s.matches("CidrRoutingConfig") /* CidrRoutingConfig com.amazonaws.route53#ResourceRecordSet$CidrRoutingConfig */ =>  {
                let var_14 =
                    Some(
                        crate::protocol_serde::shape_cidr_routing_config::de_cidr_routing_config(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_cidr_routing_config(var_14);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}

pub fn ser_resource_record_set(
    input: &crate::types::ResourceRecordSet,
    writer: aws_smithy_xml::encode::ElWriter,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_15) = &input.name {
        let mut inner_writer = scope.start_el("Name").finish();
        inner_writer.data(var_15.as_str());
    }
    if let Some(var_16) = &input.r#type {
        let mut inner_writer = scope.start_el("Type").finish();
        inner_writer.data(var_16.as_str());
    }
    if let Some(var_17) = &input.set_identifier {
        let mut inner_writer = scope.start_el("SetIdentifier").finish();
        inner_writer.data(var_17.as_str());
    }
    if let Some(var_18) = &input.weight {
        let mut inner_writer = scope.start_el("Weight").finish();
        inner_writer.data(aws_smithy_types::primitive::Encoder::from(*var_18).encode());
    }
    if let Some(var_19) = &input.region {
        let mut inner_writer = scope.start_el("Region").finish();
        inner_writer.data(var_19.as_str());
    }
    if let Some(var_20) = &input.geo_location {
        let inner_writer = scope.start_el("GeoLocation");
        crate::protocol_serde::shape_geo_location::ser_geo_location(var_20, inner_writer)?
    }
    if let Some(var_21) = &input.failover {
        let mut inner_writer = scope.start_el("Failover").finish();
        inner_writer.data(var_21.as_str());
    }
    if let Some(var_22) = &input.multi_value_answer {
        let mut inner_writer = scope.start_el("MultiValueAnswer").finish();
        inner_writer.data(aws_smithy_types::primitive::Encoder::from(*var_22).encode());
    }
    if let Some(var_23) = &input.ttl {
        let mut inner_writer = scope.start_el("TTL").finish();
        inner_writer.data(aws_smithy_types::primitive::Encoder::from(*var_23).encode());
    }
    if let Some(var_24) = &input.resource_records {
        let mut inner_writer = scope.start_el("ResourceRecords").finish();
        for list_item_25 in var_24 {
            {
                let inner_writer = inner_writer.start_el("ResourceRecord");
                crate::protocol_serde::shape_resource_record::ser_resource_record(
                    list_item_25,
                    inner_writer,
                )?
            }
        }
    }
    if let Some(var_26) = &input.alias_target {
        let inner_writer = scope.start_el("AliasTarget");
        crate::protocol_serde::shape_alias_target::ser_alias_target(var_26, inner_writer)?
    }
    if let Some(var_27) = &input.health_check_id {
        let mut inner_writer = scope.start_el("HealthCheckId").finish();
        inner_writer.data(var_27.as_str());
    }
    if let Some(var_28) = &input.traffic_policy_instance_id {
        let mut inner_writer = scope.start_el("TrafficPolicyInstanceId").finish();
        inner_writer.data(var_28.as_str());
    }
    if let Some(var_29) = &input.cidr_routing_config {
        let inner_writer = scope.start_el("CidrRoutingConfig");
        crate::protocol_serde::shape_cidr_routing_config::ser_cidr_routing_config(
            var_29,
            inner_writer,
        )?
    }
    scope.finish();
    Ok(())
}
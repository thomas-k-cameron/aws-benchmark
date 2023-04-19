// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_modify_fleet_input_input(
    input: &crate::operation::modify_fleet::ModifyFleetInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "ModifyFleet", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DryRun");
    if let Some(var_2) = &input.dry_run {
        scope_1.boolean(*var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("ExcessCapacityTerminationPolicy");
    if let Some(var_4) = &input.excess_capacity_termination_policy {
        scope_3.string(var_4.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("LaunchTemplateConfig");
    if let Some(var_6) = &input.launch_template_configs {
        let mut list_8 = scope_5.start_list(true, Some("item"));
        for item_7 in var_6 {
            #[allow(unused_mut)]
            let mut entry_9 = list_8.entry();
            crate::protocol_serde::shape_fleet_launch_template_config_request::ser_fleet_launch_template_config_request(entry_9, item_7)?;
        }
        list_8.finish();
    }
    #[allow(unused_mut)]
    let mut scope_10 = writer.prefix("FleetId");
    if let Some(var_11) = &input.fleet_id {
        scope_10.string(var_11);
    }
    #[allow(unused_mut)]
    let mut scope_12 = writer.prefix("TargetCapacitySpecification");
    if let Some(var_13) = &input.target_capacity_specification {
        crate::protocol_serde::shape_target_capacity_specification_request::ser_target_capacity_specification_request(scope_12, var_13)?;
    }
    #[allow(unused_mut)]
    let mut scope_14 = writer.prefix("Context");
    if let Some(var_15) = &input.context {
        scope_14.string(var_15);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}
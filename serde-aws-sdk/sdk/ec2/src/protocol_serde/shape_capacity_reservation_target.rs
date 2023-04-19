// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_capacity_reservation_target(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::types::CapacityReservationTarget,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("CapacityReservationId");
    if let Some(var_2) = &input.capacity_reservation_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("CapacityReservationResourceGroupArn");
    if let Some(var_4) = &input.capacity_reservation_resource_group_arn {
        scope_3.string(var_4);
    }
    Ok(())
}
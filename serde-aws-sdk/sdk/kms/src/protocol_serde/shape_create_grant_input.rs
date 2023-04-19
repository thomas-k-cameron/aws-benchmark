// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_grant_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_grant::CreateGrantInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.key_id {
        object.key("KeyId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.grantee_principal {
        object.key("GranteePrincipal").string(var_2.as_str());
    }
    if let Some(var_3) = &input.retiring_principal {
        object.key("RetiringPrincipal").string(var_3.as_str());
    }
    if let Some(var_4) = &input.operations {
        let mut array_5 = object.key("Operations").start_array();
        for item_6 in var_4 {
            {
                array_5.value().string(item_6.as_str());
            }
        }
        array_5.finish();
    }
    if let Some(var_7) = &input.constraints {
        #[allow(unused_mut)]
        let mut object_8 = object.key("Constraints").start_object();
        crate::protocol_serde::shape_grant_constraints::ser_grant_constraints(
            &mut object_8,
            var_7,
        )?;
        object_8.finish();
    }
    if let Some(var_9) = &input.grant_tokens {
        let mut array_10 = object.key("GrantTokens").start_array();
        for item_11 in var_9 {
            {
                array_10.value().string(item_11.as_str());
            }
        }
        array_10.finish();
    }
    if let Some(var_12) = &input.name {
        object.key("Name").string(var_12.as_str());
    }
    Ok(())
}
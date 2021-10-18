// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_query_forecast_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::QueryForecastInput,
) {
    if let Some(var_1) = &input.forecast_arn {
        object.key("ForecastArn").string(var_1);
    }
    if let Some(var_2) = &input.start_date {
        object.key("StartDate").string(var_2);
    }
    if let Some(var_3) = &input.end_date {
        object.key("EndDate").string(var_3);
    }
    if let Some(var_4) = &input.filters {
        let mut object_5 = object.key("Filters").start_object();
        for (key_6, value_7) in var_4 {
            {
                object_5.key(key_6).string(value_7);
            }
        }
        object_5.finish();
    }
    if let Some(var_8) = &input.next_token {
        object.key("NextToken").string(var_8);
    }
}
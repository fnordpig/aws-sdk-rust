// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn stdoptionoptionsmithytypesinstant_epoch_seconds_deser<'de, D>(
    _deser: D,
) -> Result<std::option::Option<smithy_types::Instant>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;
    Ok(
        std::option::Option::<crate::instant_epoch::InstantEpoch>::deserialize(_deser)?
            .map(|el| el.0),
    )
}
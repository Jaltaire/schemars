use alloc::borrow::Cow;
use std::format;
use optional_field01::Field;
use crate::{JsonSchema, Schema, SchemaGenerator};
use crate::_private::allow_null;

impl<T: JsonSchema> JsonSchema for Field<T> {
    fn schema_name() -> Cow<'static, str> {
        format!("MissingOrNullable_{}", T::schema_name()).into()
    }

    fn schema_id() -> Cow<'static, str> {
        format!("Field<{}>", T::schema_id()).into()
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let mut schema = generator.subschema_for::<T>();

        allow_null(generator, &mut schema);

        schema
    }

    fn _schemars_private_non_optional_json_schema(generator: &mut SchemaGenerator) -> Schema {
        T::_schemars_private_non_optional_json_schema(generator)
    }

    fn _schemars_private_is_option() -> bool {
        true
    }
}

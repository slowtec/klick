use klick_boundary::ValueId;

use crate::forms;

type FieldSet = forms::FieldSet<ValueId>;

const JSON_FIELD_SETS: &[u8] = include_bytes!("form_field_sets.json");

pub fn field_sets() -> Vec<FieldSet> {
    serde_json::from_slice(JSON_FIELD_SETS).unwrap()
}

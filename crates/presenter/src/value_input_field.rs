use klick_domain::InputValueId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputValueFieldType {
    Checkbox,
    FloatInput,
    IntegerInput,
    TextArea,
    TextInput,
}

pub trait InputValueFieldTypeHint {
    fn field_type_hint(&self) -> Option<InputValueFieldType>;
}

impl InputValueFieldTypeHint for InputValueId {
    fn field_type_hint(&self) -> Option<InputValueFieldType> {
        None
    }
}

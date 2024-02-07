use crate::forms::{self, FieldType, MinMax};

use super::fields::{FieldId, ScenarioFieldId};
type Id = FieldId;

pub type FieldSet = forms::FieldSet<Id>;
pub type Field = forms::Field<Id>;

#[allow(clippy::too_many_lines)]
pub fn field_sets() -> Vec<FieldSet> {
    vec![
    FieldSet {
        title: Some("Auswertungsszenarien für Lachgasemissionen"),
        fields: vec![
            Field {
                id: Id::Scenario(ScenarioFieldId::N2oCustomFactor),
                description: Some(
                    "Über dieses Eingabefeld können Sie (z.B. anhand einer eigenen Abschätzung oder einer Messkampagne) einen Wert für den EF N₂O eintragen.",
                ),
                required: false,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "% des TN(Zulauf)",
                    ),
                    limits: MinMax {
                        min: Some(
                            0.0,
                        ),
                        max: Some(
                            100.0,
                        ),
                    },
                    unit: "%",
                },
            },
        ],
    },
]
}

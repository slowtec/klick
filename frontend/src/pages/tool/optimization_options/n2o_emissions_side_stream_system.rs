use super::Card;
use leptos::*;

use crate::{
    forms::{render_field_sets, FieldType},
    pages::tool::{
        field_sets::{Field, FieldSet},
        fields::{FieldId, ScenarioFieldId},
        FieldSignal,
    },
};

pub fn options(n2o_side_stream_cover_is_open: RwSignal<Option<bool>>) -> impl IntoView {
    let field_set = field_set();
    let (signals1, form1, _required_fields) = render_field_sets(vec![field_set]);
    create_effect(move |_| {
        let field_signal = signals1.get(&FieldId::Scenario(
            ScenarioFieldId::N2OSideStreamCoverIsOpen.into(),
        ));
        match field_signal.and_then(FieldSignal::get_bool) {
            Some(v) => n2o_side_stream_cover_is_open.set(Some(!v)),
            None => n2o_side_stream_cover_is_open.set(None),
        }
    });
    view! {
      <Card title = "Lachgasemissionen von Nebenstromanlagen" bg_color="bg-yellow">
        <p class="my-2">
          "Da es sich bei den Nebenstromanlagen um relativ kleine Becken handelt, können die
          Lachgasemissionen hier durch Abdeckung und Abluftbehandlung (Oxidation) beseitigt werden."
        </p>
        { form1 }
        <p class="my-2">
        "Im Sinne der Nachhaltigkeit und der Kreislaufschließung kann anstelle der Nebenstromanlage eine Stickstoffrückgewinnungsanlage errichtet werden."
        </p>
      </Card>
    }
}

fn field_set() -> FieldSet {
    let id = FieldId::Scenario(ScenarioFieldId::N2OSideStreamCoverIsOpen);
    let custom_factor_field = Field {
        id,
        description: None,
        required: false,
        field_type: FieldType::Bool {
            initial_value: None,
        },
    };
    let fields = vec![custom_factor_field];
    FieldSet {
        title: None,
        fields,
    }
}

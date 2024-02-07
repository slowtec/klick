use leptos::*;
use strum::AsRefStr;

use klick_app_components::forms::{self, FieldSignal, FieldType};
use klick_presenter::ValueLabel;

#[derive(AsRefStr, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Id {
    ProjectName,
}

impl ValueLabel for Id {
    fn label(&self) -> &str {
        match self {
            Self::ProjectName => "Projektname",
        }
    }
}

pub type FieldSet = forms::FieldSet<Id>;
pub type Field = forms::Field<Id>;

#[component]
pub fn ProjectNameInputForm(project_name: RwSignal<Option<String>>) -> impl IntoView {
    let field_sets = field_sets();
    let (signals, set_views, _) = forms::render_field_sets(field_sets);

    create_effect(move |_| {
        let name = signals
            .get(&Id::ProjectName)
            .and_then(FieldSignal::get_text);
        project_name.set(name);
    });

    set_views
}

pub fn field_sets() -> Vec<FieldSet> {
    vec![FieldSet {
        title: None,
        fields: vec![Field {
            id: Id::ProjectName,
            description: Some(DESCRIPTION),
            required: false,
            field_type: FieldType::Text {
                initial_value: None,
                placeholder: Some("Projektname"),
                max_len: None,
            },
        }],
    }]
}

const DESCRIPTION: &str =
"In diesem Feld können Sie einen Namen für Ihr Projekt hinterlegen. <br>
Falls Sie sich <b>angemeldet</b> haben,
wird der Projektname zur Speicherung Ihrer Eingabewerte und Ergebnisse unter dem Reiter Projekte verwendet.
Diese Daten werden unverschluesselt auf einem Server hinterlegt, Dritte können diese Daten allerdings
nicht einsehen und die UTBW wird diese Daten nicht weitergeben oder weiterverarbeiten.<br>

Wenn Sie sich <b>nicht angemeldet</b> haben, können Sie das Tool natürlich dennoch in vollem Umfang nutzen.
Ihre Daten inkl. des Projektnamens werden dabei ausschließlich lokal auf Ihrer Festplatte gespeichert
sowie nur in Ihrem Browser verarbeitet.";

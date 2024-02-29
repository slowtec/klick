use leptos::*;

use crate::{
    forms::{render_field_sets, FieldType, MinMax},
    pages::tool::{
        field_sets::{Field, FieldSet},
        fields::{FieldId, ScenarioFieldId},
        FieldSignal,
    },
};

use super::{Card, Cite, InfoBox, DWA_MERKBLATT_URL};

pub fn options(//custom_factor_bhkw: RwSignal<Option<f64>>, // FIXME needs rename to the correct signal
) -> impl IntoView {
    let field_set = field_set();
    let (signals, chp_view, _required_fields) = render_field_sets(vec![field_set]);
    let _custom_factor = signals
        .get(&FieldId::Scenario(ScenarioFieldId::CH4ChpCustomFactor))
        .and_then(FieldSignal::get_float_output_signal)
        .unwrap();

    // create_effect(move |_| { // FIXME needs to be reactived
    //     if let Some(custom_factor) = custom_factor.get() {
    //         custom_factor_bhkw.set(Some(custom_factor));
    //     }
    // });
    view! {
      <Card title = "Fossile CO₂-Emissionen aus Abwasser">
        <p>
          "Der überwiegende Teil des aus dem Abwasser freigesetzten CO₂ ist biogenen Ursprungs. Dieses CO₂ gilt daher als
          klimaneutral und wird in der Treibhausgasbilanz nicht berücksichtigt. Ein kleinerer Teil des CO₂ führt auf
          fossile und synthetische Verbindungen zurück. Schätzungen gehen davon aus, dass dies im kommunalen Abwasser
          anteilig bis zu 5–10% der organischen Fracht ausmachen kann (Law et al. 2013). Für Abwässer mit hohen Anteilen
          an gewerblichen/industriellen Einleitern (> 45 %) kann die fossile CO₂ -Fracht sogar höher liegen (UBA 2022)."
        </p>
        <p>
          "Im Folgenden können Sie auf Basis des gesamten organischer Kohlenstoffs (total organic carbon, TOCZulauf)
          des Kläranlagenzulaufs abgeschätzt, wie hoch/niedrig der Anteil an fossilem CO₂ in der biologischen Reinigung
          ist. Über einen wählbaren CO₂-EF (fossil) können sie berechnen wie sich dies auf die Klimabilanz Ihrer
          Kläranlage auswirkt."
        </p>
        <p>
          "Wenn Sie in der Datenerfassung keinen Wert im „TOCZulauf“- Eingabefeld eingetragen haben schätzt das Tool die
          fossilen CO₂-Emissionen aus der biologischen Reinigung über den angegebenen CSBZulauf mittels des theoretischen
          Umrechnungsfaktors von CSB:TOC von 1:2, 6) abgeleitet aus der chemischen Gleichung C + O₂ --> CO₂). Wenn Sie das
          untenstehende „CO₂-EF (fossil)“-Eingabefeld freilassen wird ein gemittelter CO₂-EF (fossil) von
          3,85 (nach Law et al. 2013) angenommen."
        </p>
        { chp_view }
        <InfoBox text = "Zusätzlich zu den fossilen CO₂-Emissionen aus der biologischen Reinigung, wurde ein
          erheblicher Anteil dieser Emissionen in Klärschlämmen und im Klärgas gemessen">
            <Cite source = "Auszug aus dem UBA Text 149/2022 (S. 5)" url = DWA_MERKBLATT_URL>
            "Die Untersuchungsergebnisse zeigen, dass Klärschlämme aus kommunalen Anlagen mit untergeordneten gewerblichen
            Abwässern (< 45 %, berechnet als mittlere Auslastung der Einwohnerwerte abzüglich der angeschlossenen Einwohnerzahl)
            ca. 80 % biogene Kohlenstoffanteile und Faulgase ca. 85 % biogene Kohlenstoffanteile aufweisen. Der fossile
            Kohlenstoff ist hierbei wahrscheinlich auf schwer abbaubare synthetische Produkte bzw. fossile Rohstoffe
            zurückzuführen. […] Bestimmt wurden Anteile von ca. 28 bis 71 % im Klärschlamm und ca. 11 bis 88 % im Faulgas."
            </Cite>
          </InfoBox>
        <p>
          "Diese Anteile an fossilem CO₂ könnte z.B. aus dem Klärgas abgetrennt und einer technischen Nutzung
          zugeführt werden, um das THG-Emissionspotenzial der Kläranlage weiter zu reduzieren."
        </p>
      </Card>
    }
}

fn field_set() -> FieldSet {
    let id = FieldId::Scenario(ScenarioFieldId::CH4ChpCustomFactor); // FIXME rename needed
    let custom_factor_field = Field {
        id,
        description: Some("xxx Über dieses Eingabefeld können Sie (z.B. basierend auf einer eigenen Abschätzung oder einer Messkampagne) einen Wert für den EF CH₄ eintragen."),
        required: true,
        field_type: FieldType::Float {
            initial_value: Some(3.85), // FIXME needs to be a default value but not set here
            placeholder: None, // FIXME in grey the default value
            limits: MinMax {
                min: Some(0.0),
                max: Some(100.0),
            },
            unit: "%",
        },
    };
    let fields = vec![custom_factor_field];
    FieldSet {
        title: None,
        fields,
    }
}

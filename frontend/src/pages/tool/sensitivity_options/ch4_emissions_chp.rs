use klick_app_charts::BarChartRadioInput;
use klick_presenter::Lng;
use leptos::*;
use serde::{Deserialize, Serialize};
use strum::AsRefStr;

use klick_domain as domain;

use crate::{
    forms::{render_field_sets, FieldType, MinMax},
    pages::tool::{
        field_sets::{Field, FieldSet},
        fields::{FieldId, ScenarioFieldId},
        FieldSignal,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, AsRefStr, Serialize, Deserialize)]
enum Id {
    CalcMethod,
    CustomFactor,
}

use super::{Card, Cite, InfoBox, DWA_MERKBLATT_URL};

const CH4_DEFAULT_CUSTOM_FACTOR: f64 = 3.0;

pub fn options(
    // incoming signals
    output: ReadSignal<Option<domain::EmissionsCalculationOutcome>>,
    // outgoing signals
    selected_scenario_chp: RwSignal<Option<u64>>,
    selected_scenario_name_chp: RwSignal<String>,
    custom_factor_bhkw: RwSignal<Option<f64>>,
    // incoming signals
    barchart_arguments_radio_inputs_bhkw: ReadSignal<
        Vec<klick_app_charts::BarChartRadioInputArguments>,
    >,
) -> impl IntoView {
    let field_set = field_set();
    let (signals, chp_view, _required_fields) = render_field_sets(vec![field_set]);
    let custom_factor = signals
        .get(&FieldId::Scenario(ScenarioFieldId::CH4ChpCustomFactor))
        .and_then(FieldSignal::get_float_output_signal)
        .unwrap();

    create_effect(move |_| {
        if let Some(custom_factor) = custom_factor.get() {
            custom_factor_bhkw.set(Some(custom_factor));
        } else {
            custom_factor_bhkw.set(Some(CH4_DEFAULT_CUSTOM_FACTOR));
        }
    });

    view! {
      <div class =move || {
        if barchart_arguments_radio_inputs_bhkw.get().len() > 0 {
          None
        } else {
          Some("hidden")
        }
      }>
      <Card title = "Methanemissionen aus Blockheizkraftwerken (BHKW)" bg_color="bg-blue">
        <InfoBox text = "BHKW weisen je nach Modell und Alter unterschiedliche Methanschlupfe auf">
          <Cite source = "Auszug aus dem DWA-Merkblatt 230-1 (2022, S. 25)" url = DWA_MERKBLATT_URL>
            "Auch bei der Gasverwertung entstehen prozessbedingte Methan-Emissionen:
            BHKW-Motoren arbeiten nach dem Vier-Takt-Prinzip.
            Dabei sind zum Gasaustausch für eine kurze Zeit beim Übergang vom vierten (Ausstoßen)
            in den ersten (Ansaugen) Takt sowohl das Einlass- als auch das Auslassventil (teilweise) geöffnet.
            Durch diese Überschneidung können unter Umständen geringe Mengen unverbrannten Faulgases in den Abgasstrom gelangen.
            Ottomotoren haben dabei einen Methanschlupf im Bereich von 1 % bis 2 %
            Zündstrahlmotoren (sind für Faulgas nicht relevant) liegen höher in der Größenordnung von 2 % bis 3 %.
            Mikrogasturbinen (typische Leistungsklasse von 30 kW bis 65 kW) können dagegen einen
            Methanschlupf < 1 % erreichen (STMWI 2016)."
          </Cite>
        </InfoBox>
        <p>
          "Mit der folgenden Auswahl bzw. Eingabe eines eigenen Emissionsfaktors (EF) für das BHKW Ihrer Kläranlage
          kann Ihre Klimabilanz bezüglich der Methanemissionen verfeinert abgeschätzt werden:"
        </p>
        <div class="my-4 ml-4">
        {
          move || view! {
            // show when vector has elements
          <BarChartRadioInput
            width = 900.0
            height = 300.0
            data  = barchart_arguments_radio_inputs_bhkw.get()
            selected_bar = selected_scenario_chp
            emission_factor_label = Some("CH₄ EF")
          />
          }
        }
        { chp_view }
        <p>
          "Es ist das Szenario \"" { move || selected_scenario_name_chp.get() } "\" ausgewählt in t CO₂ Äquivalente/Jahr.
          Durch Anklicken kann ein anderes Szenario ausgewählt werden."
        </p>
        <InfoBox text = "Zusatzinformation zum Methanschlupf:">
          <Cite source = "Auszug aus dem DWA-Merkblatt 230-1 (2022, S. 25)" url = DWA_MERKBLATT_URL>
            "Die Gaszusammensetzung, Brennraumtemperatur (Gasfeuchte), Brennraumgestaltung und Betriebsweise beeinflussen die Verbrennungsvorgänge.
            Bei hohen Sauerstoffkonzentrationen (Magerbetrieb), welche für die Reduktion der NOₓ,-Bildung bei hohen Temperaturen notwendig sind,
            steigt der Methanschlupf.
            Neben der Betriebsweise hat auch die Aggregateleistung einen Einfluss auf den Methan-schlupf.
            So hat sich bei Messungen im Betrieb gezeigt, dass unter Volllast in der Regel weniger Methan über das Abgas emittiert wird
            als bei Teillastbetrieb.
            Bei Mikrogasturbinen ist dieser Effekt sehr stark ausgeprägt
            und kann zu einem Anstieg bis auf > 5 % im 60-%-Teillastbetrieb führen (STMWI 2016)."
          </Cite>
        </InfoBox>
        <div class="border-t pt-3 mt-4 border-gray-900/10">
          { move || {
              output.get().map(|out|
                view! {
                   <p>
                   </p>
                  <dl class="mx-3 my-2 grid grid-cols-2 text-sm">
                    <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">"Methanemissionen aus Blockheizkraftwerken (BHKW)"</dt>
                    <dd class="text-lg py-1 px-3">
                      { format!("{:.1}", f64::from(out.co2_equivalents.ch4_combined_heat_and_power_plant)).replace('.',",") }
                      <span class="ml-2 text-gray-400">{ "t CO₂-Äq./a" }</span>
                    </dd>
                    <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">"Gesamtemissionen"</dt>
                    <dd class="text-lg py-1 px-3">
                      { format!("{:.1}", f64::from(out.co2_equivalents.total_emissions)).replace('.',",") }
                      <span class="ml-2 text-gray-400">{ "t CO₂-Äq./a" }</span>
                    </dd>
                  </dl>
                }
              )
            }
          }
        </div>
        </div>
      </Card>
      </div>
    }
}

fn field_set() -> FieldSet {
    let id = FieldId::Scenario(ScenarioFieldId::CH4ChpCustomFactor);
    let custom_factor_field = Field {
        id,
        description: Some("Über dieses Eingabefeld können Sie (z.B. basierend auf einer eigenen Abschätzung oder einer Messkampagne) einen Wert für den EF CH₄ eintragen."),
        required: false,
        field_type: FieldType::Float {
            initial_value: None,
            placeholder: Some(Lng::De.format_number(f64::from(CH4_DEFAULT_CUSTOM_FACTOR))),
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

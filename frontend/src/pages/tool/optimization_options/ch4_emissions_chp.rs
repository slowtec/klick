use leptos::*;
use serde::{Deserialize, Serialize};
use strum::AsRefStr;

use klick_domain as domain;

use crate::{
    forms::{render_field_sets, FieldType, MinMax, SelectOption},
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

use super::{Card, Cite, InfoBox, ScenarioHint, DWA_MERKBLATT_URL};

pub fn options(
    input_data: Signal<Option<domain::EmissionInfluencingValues>>,
    n2o_emission_factor_method: Signal<Option<domain::N2oEmissionFactorCalcMethod>>,
) -> impl IntoView {
    let field_set = field_set();

    let (signals, chp_view, _required_fields) = render_field_sets(vec![field_set]);

    let selection = signals
        .get(&FieldId::Scenario(ScenarioFieldId::CH4ChpCalculationMethod))
        .and_then(FieldSignal::get_selection_signal)
        .unwrap();

    let custom_factor = signals
        .get(&FieldId::Scenario(ScenarioFieldId::CH4ChpCustomFactor))
        .and_then(FieldSignal::get_float_output_signal)
        .unwrap();

    let output = RwSignal::new(Option::<(domain::CO2Equivalents, domain::EmissionFactors)>::None);

    create_effect(move |_| {
        log::debug!("Calculate with CH₄ CHP emission factor");
        let Some(input_data) = input_data.get() else {
            log::debug!("No input data");
            output.set(None);
            return;
        };

        let Some(sel) = selection.get() else {
            log::debug!("No calculation method selected");
            output.set(None);
            return;
        };

        let ch4_chp_emission_factor = match sel {
            1 => domain::CH4ChpEmissionFactorCalcMethod::MicroGasTurbines,
            2 => domain::CH4ChpEmissionFactorCalcMethod::GasolineEngine,
            3 => domain::CH4ChpEmissionFactorCalcMethod::JetEngine,
            4 => {
                let Some(f) = custom_factor.get() else {
                    log::warn!("No custom factor defined");
                    output.set(None);
                    return;
                };
                domain::CH4ChpEmissionFactorCalcMethod::Custom(domain::units::Factor::new(
                    f / 100.0,
                ))
            }
            _ => {
                output.set(None);
                return;
            }
        };

        log::debug!("Calculate with CH₄ CHP emission factor {ch4_chp_emission_factor:?}");
        let scenario = domain::EmissionFactorCalculationMethods {
            n2o: n2o_emission_factor_method
                .get()
                .unwrap_or(domain::N2oEmissionFactorCalcMethod::Ipcc2019),
            ch4: Some(ch4_chp_emission_factor),
        };
        let output_data = domain::calculate_emissions(&input_data, scenario);
        output.set(Some(output_data));
    });

    view! {
      <Card title = "Methanemissionen aus Blockheizkraftwerken (BHKW)" >
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
          { chp_view }
        </div>
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
          <ScenarioHint output = output.into() n2o_emission_factor_method />
          { move || {
              output.get().map(|out|
                view! {
                   <p>
                   </p>
                  <dl class="mx-3 my-2 grid grid-cols-2 text-sm">
                    <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">"Methanemissionen aus Blockheizkraftwerken (BHKW)"</dt>
                    <dd class="text-lg py-1 px-3">
                      { format!("{:.1}", f64::from(out.0.ch4_combined_heat_and_power_plant)).replace('.',",") }
                      <span class="ml-2 text-gray-400">{ "t" }</span>
                    </dd>
                    <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">"Gesamtemissionen"</dt>
                    <dd class="text-lg py-1 px-3">
                      { format!("{:.1}", f64::from(out.0.emissions)).replace('.',",") }
                      <span class="ml-2 text-gray-400">{ "t" }</span>
                    </dd>
                  </dl>
                }
              )
            }
          }
        </div>

      </Card>
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
            placeholder: None,
            limits: MinMax {
                min: Some(0.0),
                max: Some(100.0),
            },
            unit: "%",
        },
    };

    let id = FieldId::Scenario(ScenarioFieldId::CH4ChpCalculationMethod);
    let calc_method_field = Field {
        id,
        description: None,
        required: false,
        field_type: FieldType::Selection {
            initial_value: None,
            options: vec![
                SelectOption {
                    label: "Mikrogasturbinen (EF max. 1 %)",
                    value: 1,
                },
                SelectOption {
                    label: "Ottomotor (EF MW = 1,5 %)",
                    value: 2,
                },
                SelectOption {
                    label: "Zündstrahlmotor (EF MW = 2,5 %)",
                    value: 3,
                },
                SelectOption {
                    label: "BHKW CH₄-EF benutzerdefiniert",
                    value: 4,
                },
            ],
        },
    };

    let fields = vec![calc_method_field, custom_factor_field];

    FieldSet {
        title: None,
        fields,
    }
}

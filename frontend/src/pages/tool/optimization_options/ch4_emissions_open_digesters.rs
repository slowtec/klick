use leptos::*;
use serde::{Deserialize, Serialize};
use strum::AsRefStr;

use klick_domain as domain;
use klick_presenter::ValueLabel;

use crate::{
    forms::{render_field_sets, Field, FieldSet, FieldType},
    pages::tool::FieldSignal,
};

use super::{Card, Cite, InfoBox, ScenarioHint, DWA_MERKBLATT_URL};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, AsRefStr, Serialize, Deserialize)]
enum Id {
    SludgeBags,
    SludgeStorageContainers,
}

impl ValueLabel for Id {
    fn label(&self) -> &str {
        match self {
            Self::SludgeBags => "Schließen der Schlammtaschen",
            Self::SludgeStorageContainers => "Schließen der Schlammstapelbehälter",
        }
    }
}

pub fn options(
    input_data: Signal<Option<domain::EmissionInfluencingValues>>,
    n2o_emission_factor_method: Signal<Option<domain::N2oEmissionFactorCalcMethod>>,
) -> impl IntoView {
    let sludge_bags_are_open = Field {
        id: Id::SludgeBags,
        description: None,
        required: false,
        field_type: FieldType::Bool {
            initial_value: None,
        },
    };

    let sludge_storage_containers_are_open = Field {
        id: Id::SludgeStorageContainers,
        description: None,
        required: false,
        field_type: FieldType::Bool {
            initial_value: None,
        },
    };

    let field_set = FieldSet {
        title: None,
        fields: vec![sludge_bags_are_open, sludge_storage_containers_are_open],
    };

    let (signals, fields_view, _required_fields) = render_field_sets(vec![field_set]);

    let output = RwSignal::new(Option::<(domain::CO2Equivalents, domain::EmissionFactors)>::None);

    create_effect(move |_| {
        let Some(mut input_data) = input_data.get() else {
            log::debug!("No input data");
            output.set(None);
            return;
        };

        let n2o_emission_factor = n2o_emission_factor_method
            .get()
            .unwrap_or(domain::N2oEmissionFactorCalcMethod::Ipcc2019);

        let scenario = domain::EmissionFactorCalculationMethods {
            n2o: n2o_emission_factor,
            ch4: None,
        };

        let field_signal = signals.get(&Id::SludgeBags);

        input_data.sewage_sludge_treatment.sludge_bags_are_open =
            if let Some(v) = field_signal.and_then(FieldSignal::get_bool) {
                !v
            } else {
                let value = !input_data.sewage_sludge_treatment.sludge_bags_are_open;
                field_signal
                    .and_then(FieldSignal::get_bool_signal)
                    .unwrap()
                    .set(value);
                value
            };

        let field_signal = signals.get(&Id::SludgeStorageContainers);

        input_data
            .sewage_sludge_treatment
            .sludge_storage_containers_are_open =
            if let Some(v) = field_signal.and_then(FieldSignal::get_bool) {
                !v
            } else {
                let value = !input_data
                    .sewage_sludge_treatment
                    .sludge_storage_containers_are_open;
                field_signal
                    .and_then(FieldSignal::get_bool_signal)
                    .unwrap()
                    .set(value);
                value
            };

        let output_data = domain::calculate_emissions(&input_data, scenario);
        output.set(Some(output_data));
    });

    view! {
      <Card title = "Methanemissionen aus offenen Faultürmen und bei der Schlammlagerung">
        <p>
          "Das Schließen von "<b>"Schlammtaschen"</b>" an Faultürmen wirkt sich durch die Eindämmung von Methanschlupfen positiv auf die Klimabilanz von Kläranlagen aus.
          Die Anzahl der Faultürme wird hierbei proportional berücksichtigt (siehe Eingabe im Feld „Anzahl der Faultürme“)."
        </p>
        <div class="my-4 ml-4">
          { fields_view }
        </div>
        <InfoBox text = "Auch Schlammstapelbehälter tragen maßgeblich zu Methanemissionen bei, falls diese nicht geschlossen sind/werden.">
          <Cite source = "Auszug aus dem DWA-Merkblatt 230-1 (S. 24)" url = DWA_MERKBLATT_URL >
            "Messungen von OSHITA et al. (2014) zeigen mit einer im Nacheindicker vorliegenden CH"<sub>4</sub>"-Konzentration,
            die 16 % über der theoretischen Löslichkeit von Methan lag,
            die Relevanz des Nachgasungspotenzials für die Reduzierung der THG-Emission aus der Schlammbehandlung.
            Im Klärwerk Köhlbrandhöft werden seit Abdeckung des Nacheindickers ca. 1.100 m"<sup>3</sup>" CH"<sub>4</sub>"/d
            zusätzlich der Gasverwertung zugeführt,
            welche ohne Abdeckung und Verwertung als Treibhausgas emittiert worden wären (SCHÄFER 2020),
            was 1,6 % der täglichen Gasmenge entspricht."
          </Cite>
        </InfoBox>

        <div class="border-t pt-3 mt-4 border-gray-900/10">
          <ScenarioHint output = output.into() n2o_emission_factor_method />
          { move || {
              output.get().map(|out|
                view! {
                  <dl class="mx-3 my-2 grid grid-cols-2 text-sm">

                    <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">"Schließen der Schlammtaschen"</dt>
                    <dd class="text-lg py-1 px-3">
                      { format!("{:.1}", f64::from(out.0.ch4_sludge_bags)).replace('.',",") }
                      <span class="ml-2 text-gray-400">{ "t" }</span>
                    </dd>

                    <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">"Schließen der Schlammstapelbehälter"</dt>
                    <dd class="text-lg py-1 px-3">
                      { format!("{:.1}", f64::from(out.0.ch4_sludge_storage_containers)).replace('.',",") }
                      <span class="ml-2 text-gray-400">{ "t" }</span>
                    </dd>

                    <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">"Gesamtemissionen der Kläranlage"</dt>
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

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
    output: ReadSignal<
        Option<(
            domain::CO2Equivalents,
            domain::EmissionFactors,
            domain::EmissionFactorCalculationMethods,
        )>,
    >,
    sludge_bags_are_open: RwSignal<Option<bool>>,
    sludge_storage_containers_are_open: RwSignal<Option<bool>>,
) -> impl IntoView {
    let sludge_bags_are_open_field = Field {
        id: Id::SludgeBags,
        description: None,
        required: false,
        field_type: FieldType::Bool {
            initial_value: None,
        },
    };

    let sludge_storage_containers_are_open_field = Field {
        id: Id::SludgeStorageContainers,
        description: None,
        required: false,
        field_type: FieldType::Bool {
            initial_value: None,
        },
    };

    let field_set = FieldSet {
        title: None,
        fields: vec![
            sludge_bags_are_open_field,
            sludge_storage_containers_are_open_field,
        ],
    };

    let (signals, fields_view, _required_fields) = render_field_sets(vec![field_set]);

    create_effect(move |_| {
        let field_signal = signals.get(&Id::SludgeBags);
        match field_signal.and_then(FieldSignal::get_bool) {
            Some(v) => sludge_bags_are_open.set(Some(!v)),
            None => sludge_bags_are_open.set(None),
        }

        let field_signal = signals.get(&Id::SludgeStorageContainers);
        match field_signal.and_then(FieldSignal::get_bool) {
            Some(v) => sludge_storage_containers_are_open.set(Some(!v)),
            None => sludge_storage_containers_are_open.set(None),
        }
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
          <ScenarioHint output = output />
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

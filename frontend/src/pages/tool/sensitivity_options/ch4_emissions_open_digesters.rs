use leptos::*;
use serde::{Deserialize, Serialize};
use strum::AsRefStr;

use klick_domain as domain;
use klick_presenter::Lng;
use klick_presenter::ValueLabel;

use crate::{
    forms::{render_field_sets, FieldType, MinMax},
    pages::tool::{
        field_sets::{Field, FieldSet},
        fields::{FieldId, ScenarioFieldId},
        FieldSignal,
    },
};

use super::{Card, Cite, InfoBox, ScenarioHint, DWA_MERKBLATT_URL};

pub fn options(
    output: ReadSignal<Option<domain::EmissionsCalculationOutcome>>,
    sludge_bags_are_open: RwSignal<Option<bool>>,
    custom_sludge_bags_factor: RwSignal<Option<f64>>,
    sludge_storage_containers_are_open: RwSignal<Option<bool>>,
    custom_sludge_storage_containers_factor: RwSignal<Option<f64>>,
) -> impl IntoView {
    let custom_factor_field = Field {
        id: FieldId::Scenario(ScenarioFieldId::SludgeBagsCustomFactor),
        description: Some("Über dieses Eingabefeld können Sie (z.B. basierend auf einer eigenen Abschätzung oder einer Messkampagne) einen Wert für den EF CH₄ eintragen."),
        required: false,
        field_type: FieldType::Float {
            initial_value: None,
            placeholder: Some(Lng::De.format_number(f64::from(klick_domain::constants::EMISSION_FACTOR_SLUDGE_BAGS))),
            limits: MinMax {
                min: Some(0.0),
                max: Some(100.0),
            },
            unit: "m³(CH₄)/h",
        },
    };

    let field_set = FieldSet {
        title: None,
        fields: vec![custom_factor_field],
    };

    let (signals1, fields_view1, _required_fields) = render_field_sets(vec![field_set]);

    let custom_factor_field2 = Field {
        id: FieldId::Scenario(ScenarioFieldId::SludgeStorageCustomFactor),
        description: Some("Über dieses Eingabefeld können Sie (z.B. basierend auf einer eigenen Abschätzung oder einer Messkampagne) einen Wert für den EF CH₄ eintragen."),
        required: false,
        field_type: FieldType::Float {
            initial_value: None,
            placeholder: Some(Lng::De.format_number(f64::from(klick_domain::constants::EMISSION_FACTOR_SLUDGE_STORAGE))),
            limits: MinMax {
                min: Some(0.0),
                max: Some(100.0),
            },
            unit: "%",
        },
    };

    let field_set = FieldSet {
        title: None,
        fields: vec![custom_factor_field2],
    };

    let (signals2, fields_view2, _required_fields) = render_field_sets(vec![field_set]);

    let custom_sludge_bags_factor_field = signals1
        .get(&FieldId::Scenario(ScenarioFieldId::SludgeBagsCustomFactor))
        .and_then(FieldSignal::get_float_output_signal)
        .unwrap();

    let custom_sludge_storage_containers_factor_field = signals2
        .get(&FieldId::Scenario(ScenarioFieldId::SludgeStorageCustomFactor).into())
        .and_then(FieldSignal::get_float_output_signal)
        .unwrap();

    create_effect(move |_| {
        match custom_sludge_bags_factor_field.get() {
            Some(_v) => custom_sludge_bags_factor.set(custom_sludge_bags_factor_field.get()),
            None => custom_sludge_bags_factor.set(Some(f64::from(
                klick_domain::constants::EMISSION_FACTOR_SLUDGE_BAGS,
            ))),
        }
        match custom_sludge_storage_containers_factor_field.get() {
            Some(_v) => custom_sludge_storage_containers_factor
                .set(custom_sludge_storage_containers_factor_field.get()),
            None => custom_sludge_storage_containers_factor.set(Some(f64::from(
                klick_domain::constants::EMISSION_FACTOR_SLUDGE_STORAGE,
            ))),
        }
    });

    view! {
      <Card title = "Methanemissionen aus offenen Faultürmen und bei der Schlammlagerung" bg_color="bg-blue">
        <p>
          "Das Schließen von "<b>"Schlammtaschen"</b>" an Faultürmen wirkt sich durch die Eindämmung von
          Methanschlupfen positiv auf die Klimabilanz von Kläranlagen aus.
          "
        <InfoBox text = "Die Anzahl der Faultürme wird hierbei proportional berücksichtigt (siehe Eingabe im Feld „Anzahl der Faultürme“).">
          <Cite source = "" url = "" >
          "Der CH₄-EF für den Methanschlupf an einer Schlammtasche betrug in einer Studie von Li (2020) 1,25 m³/h. Falls Sie
          (in der Datenerfassung) 'offene Schlammtaschen' ausgewählt haben, können Sie nachfolgend anhand dessen die positiven
          Auswirkungen des Schließens der Schlammtaschen auf Ihre Klimabilanz abschätzen."
          </Cite>
        </InfoBox>
        </p>

        <p>
          "Sie können auch einen eigenen CH₄-EF [in m³/h] für den Methanschlupf an Ihrem/Ihren \
          Faulturm/Faultürmen (z.B. anhand einer Mess-/Monitoringkampagne) eintragen eingeben und bilanzieren lassen."
        </p>
        <div class="my-4 ml-4">
          { fields_view1 }
        </div>
        <InfoBox text = "Auch die Schlammlagerung trägt maßgeblich zu Methanemissionen bei, falls diese nicht geschlossen sind/werden.">
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

        <p>"Falls Sie (in der Datenerfassung) 'offene Schlammlager' ausgewählt haben, können Sie nachfolgend die positiven
        Auswirkungen geschlossener Schlammlager auf Ihre Klimabilanz abschätzen. Diese wird anhand des CH₄-EF von 1,6 %
        Methanschlupf der Gesamtklärgansmenge nach Schäfer (2020) verwendet. Das nebenstehende Eingabefeld ermöglicht es
        aber auch einen eigenen CH₄-EF [in %] für Ihren Schlammstapelbehälter (z.B. anhand einer Restgas potentialanalyse)
        eintragen und bilanzieren lassen."
        </p>
        <div class="my-4 ml-4">
          { fields_view2 }
        </div>

        <div class="border-t pt-3 mt-4 border-gray-900/10">
          <ScenarioHint output = output />
          { move || {
              output.get().map(|out|
                view! {
                  <dl class="mx-3 my-2 grid grid-cols-2 text-sm">

                    <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">"Schließen der Schlammtaschen"</dt>
                    <dd class="text-lg py-1 px-3">
                      { format!("{:.1}", f64::from(out.co2_equivalents.ch4_sludge_bags)).replace('.',",") }
                      <span class="ml-2 text-gray-400">{ "t CO₂-Äq./a" }</span>
                    </dd>

                    <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">"Schließen der Schlammlagerung"</dt>
                    <dd class="text-lg py-1 px-3">
                      { format!("{:.1}", f64::from(out.co2_equivalents.ch4_sludge_storage_containers)).replace('.',",") }
                      <span class="ml-2 text-gray-400">{ "t CO₂-Äq./a" }</span>
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

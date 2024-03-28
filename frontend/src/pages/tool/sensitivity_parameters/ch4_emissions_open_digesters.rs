use leptos::*;

use klick_app_components::forms::*;
use klick_boundary::FormData;
use klick_presenter::{Lng, ScenarioFieldId, ValueLabel};

use crate::pages::tool::{CalculationOutcome, Card, Cite, InfoBox, DWA_MERKBLATT_URL};

#[component]
pub fn CH4EmissionsOpenDigesters(
    form_data: RwSignal<FormData>,
    input_data: ReadSignal<FormData>,
    outcome: Signal<CalculationOutcome>,
) -> impl IntoView {
    let show_sludge_bags_controls = Signal::derive(move || {
        // a better way could be to check out.co2_equivalents.ch4_sludge_bags > 0.0
        form_data.with(|d| {
            !d.plant_profile
                .sewage_sludge_treatment
                .sludge_bags_are_closed
                .unwrap_or(false)
        })
    });
    let show_sludge_storage_containers_controls = Signal::derive(move || {
        // a better way could be to check out.co2_equivalents.ch4_sludge_storage_containers > 0.0
        form_data.with(|d| {
            !d.plant_profile
                .sewage_sludge_treatment
                .sludge_storage_containers_are_closed
                .unwrap_or(false)
        })
    });
    let show_dialog = Signal::derive(move || {
        let digester_count = form_data.with(|d| {
            d.plant_profile
                .sewage_sludge_treatment
                .digester_count
                .unwrap_or(0)
        });
        let sewage_gas_produced = form_data.with(|d| {
            d.plant_profile
                .energy_consumption
                .sewage_gas_produced
                .unwrap_or(0.0)
        });
        (show_sludge_bags_controls.get() || show_sludge_storage_containers_controls.get())
            && (sewage_gas_produced > 0.0 || digester_count > 0)
    });

    let custom_factor_field = Field {
             label: ScenarioFieldId::SludgeBagsCustomFactor.label(),
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
                 on_change: Callback::new(move |v| {
                     form_data.update(|d| {
                         d.sensitivity_parameters
                         .ch4_sewage_sludge_emissions
                         .emission_factor_sludge_bags = v;
                     });
                 }),
                 input: Signal::derive(move || {
                     input_data.with(|d| {
                         d.sensitivity_parameters
                         .ch4_sewage_sludge_emissions
                         .emission_factor_sludge_bags
                     })
                 }),
             },
         };

    let field_set = FieldSet {
        title: None,
        fields: vec![custom_factor_field],
    };

    let (fields_view1, _, _) = render_field_sets(vec![field_set]);

    let custom_factor_field2 = Field {
             label: ScenarioFieldId::SludgeStorageCustomFactor.label(),
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
                 on_change: Callback::new(move |v| {
                     form_data.update(|d| {
                         d.sensitivity_parameters
                         .ch4_sewage_sludge_emissions
                         .emission_factor_sludge_storage_containers = v;
                     });
                 }),
                 input: Signal::derive(move || {
                     input_data.with(|d| {
                         d.sensitivity_parameters
                         .ch4_sewage_sludge_emissions
                         .emission_factor_sludge_storage_containers
                     })
                 }),
             },
         };

    let field_set = FieldSet {
        title: None,
        fields: vec![custom_factor_field2],
    };

    let (fields_view2, _, _) = render_field_sets(vec![field_set]);

    // FIXME: set default values in page::tool::default_values
    // create_effect(move |_| {
    //     match custom_sludge_bags_factor_field.get() {
    //         Some(_v) => custom_sludge_bags_factor.set(custom_sludge_bags_factor_field.get()),
    //         None => custom_sludge_bags_factor.set(Some(f64::from(
    //             klick_domain::constants::EMISSION_FACTOR_SLUDGE_BAGS,
    //         ))),
    //     }
    //     match custom_sludge_storage_containers_factor_field.get() {
    //         Some(_v) => custom_sludge_storage_containers_factor
    //             .set(custom_sludge_storage_containers_factor_field.get()),
    //         None => custom_sludge_storage_containers_factor.set(Some(f64::from(
    //             klick_domain::constants::EMISSION_FACTOR_SLUDGE_STORAGE,
    //         ))),
    //     }
    // });

    view! {
      <div class = move || { if show_dialog.get() { None } else { Some("hidden") } } >
        <Card id = "sensitivity-open-digesters" title = "Methanemissionen aus offenen Faultürmen und bei der Schlammlagerung" bg_color="bg-blue">
          <div class = move || { if show_sludge_bags_controls.get() { None } else { Some("hidden") } } >
             <p class="my-2">
             "Durch "<b>"offene Schlammtaschen an Faultürmen"</b>" kann Methan entweichen. Nachfolgend kann für den
             Methanschlupf (z.B. aus einer Messkampagne oder als Schätzwert) ein Emissionsfaktor CH₄-EF [in m³/h] bilanziert werden."
             </p>
              <p class="my-2">
             "Wenn Sie das Feld leer lassen, wird mit einem gemittelten EF von 1,25 m³/h nach Li (2020) gerechnet.
             In beiden Fällen wird die Anzahl der Faultürme anteilig berücksichtigt (siehe Eingabe „Anzahl der Faultürme“ in der Datenerfassung)."
             </p>
             <div class="my-4 ml-4">
               { fields_view1 }
             </div>
          </div>
          <div class = move || { if show_sludge_storage_containers_controls.get() { None } else { Some("hidden") } } >
            <InfoBox text = "Die Schlammlagerung trägt maßgeblich zu Methanemissionen bei">
              <Cite source = "Auszug aus dem DWA-Merkblatt 230-1 (S. 24)" url = DWA_MERKBLATT_URL >
              "In Abhängigkeit vom technischen Ausfaulgrad der Schlammfaulung und der Lagerzeit können bei der
              Faulschlammlagerung noch bis zu 15 kg CO"<sub>2</sub>"-Äquivalente/(E·a) emittiert werden (Quelle: DWA 2020).
              Das entspricht einem Methanbildungspotenzial von 576 g CH"<sub>4</sub>"/(E·a). Für die Methan-Emissionen aus
              der Lagerung und Entwässerung von ausgefaultem Schlamm wird von PARRAVICINI et al. (2016) ein Bereich
              von 2 % bis 4,5 % der Methanproduktion angegeben."
              </Cite>
            </InfoBox>

           <p class="my-2">
             "Wenn Sie (in der Datenerfassung) 'offene Schlammlager' ausgewählt haben, können Sie die Auswirkungen des
             Methanschlupfes auf die Klimabilanz Ihrer Kläranlage abschätzen. Das folgende Eingabefeld ermöglicht Ihnen
             dazu die Bilanzierung eines CH₄-EF [%] für Ihren Schlammstapelbehälter (z.B. auf Basis einer Restgaspotentialanalyse).
             Wenn Sie das Feld leer lassen, wird der Referenzwert von Parravicini et al. (2016) CH₄-EF = 2 % der gesamten Klärgasmenge verwendet."
           </p>
           <div class="my-4 ml-4">
             { fields_view2 }
           </div>
         </div>

         <div class="border-t pt-3 mt-4 border-gray-900/10">
           { move || {
               let show_sludge_bags_controls_class = match show_sludge_bags_controls.get() {
                     false => "hidden".to_string(),
                     true => String::new(),
               };
               let show_sludge_storage_containers_controls_class = match show_sludge_storage_containers_controls.get() {
                   false => "hidden".to_string(),
                   true => String::new(),
               };
               outcome.with(|out|out.sensitivity.output.as_ref().map(|out|{
                 view! {
                   <dl class="mx-3 my-2 grid grid-cols-2 text-sm">
                     <dt class={ format!("text-lg font-semibold text-right px-3 py-1 text-gray-500 {show_sludge_bags_controls_class}") }
                     >
                        "CH₄ Schlupf Schlammtaschen"
                     </dt>
                     <dd class={ format!("text-lg py-1 px-3 {show_sludge_bags_controls_class}") }
                     >
                       { format!("{:.1}", f64::from(out.co2_equivalents.ch4_sludge_bags)).replace('.',",") }
                       <span class="ml-2 text-gray-400">{ "t CO₂-Äq./a" }</span>
                     </dd>
                     <dt class={ format!("text-lg font-semibold text-right px-3 py-1 text-gray-500 {show_sludge_storage_containers_controls_class}") }
                     >
                        "CH₄ Schlupf Schlammlagerung"
                     </dt>
                     <dd class={ format!("text-lg py-1 px-3 {show_sludge_storage_containers_controls_class}") } >
                       { format!("{:.1}", f64::from(out.co2_equivalents.ch4_sludge_storage_containers)).replace('.',",") }
                       <span class="ml-2 text-gray-400">{ "t CO₂-Äq./a" }</span>
                     </dd>
                     <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">"Gesamtemissionen"</dt>
                     <dd class="text-lg py-1 px-3">
                       { format!("{:.1}", f64::from(out.co2_equivalents.total_emissions)).replace('.',",") }
                       <span class="ml-2 text-gray-400">{ "t CO₂-Äq./a" }</span>
                     </dd>
                   </dl>
                 }
               }))
             }
           }
         </div>
        </Card>
      </div>
    }
}

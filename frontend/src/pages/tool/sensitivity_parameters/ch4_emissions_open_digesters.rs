use leptos::*;

use klick_app_components::forms::*;
use klick_boundary::FormData;
use klick_domain::{output_value::required, InputValueId as Id, OutputValueId as Out, Value};

use crate::pages::tool::{
    fields::create_field, CalculationOutcome, Card, Cite, InfoBox, DWA_MERKBLATT_URL,
};

#[allow(clippy::too_many_lines)] // TODO
#[component]
pub fn CH4EmissionsOpenDigesters(
    form_data: RwSignal<FormData>,
    input_data: ReadSignal<FormData>,
    outcome: Signal<CalculationOutcome>,
    accessibility_always_show_option: Option<RwSignal<bool>>,
) -> impl IntoView {
    let show_sludge_bags_controls = Signal::derive(move || {
        // a better way could be to check out.co2_equivalents.ch4_sludge_bags > 0.0
        form_data.with(|d| {
            !d.get(&Id::SludgeTreatmentBagsAreOpen)
                .cloned()
                .map(Value::as_bool_unchecked)
                .is_some_and(|v| !v)
        })
    });
    let show_sludge_storage_containers_controls = Signal::derive(move || {
        // a better way could be to check out.co2_equivalents.ch4_sludge_storage_containers > 0.0
        form_data.with(|d| {
            !d.get(&Id::SludgeTreatmentStorageContainersAreOpen)
                .cloned()
                .map(Value::as_bool_unchecked)
                .is_some_and(|v| !v)
        })
    });
    let show_dialog = Signal::derive(move || {
        let digester_count = form_data.with(|d| {
            d.get(&Id::SludgeTreatmentDigesterCount)
                .cloned()
                .map(Value::as_count_unchecked)
                .map(u64::from)
                .unwrap_or_default()
        });
        let sewage_gas_produced = form_data.with(|d| {
            d.get(&Id::SewageGasProduced)
                .cloned()
                .map(Value::as_qubicmeters_unchecked)
                .map(f64::from)
                .unwrap_or_default()
        });
        (show_sludge_bags_controls.get() || show_sludge_storage_containers_controls.get())
            && (sewage_gas_produced > 0.0 || digester_count > 0)
    });

    let id = Id::SensitivitySludgeBagsCustomFactor;

    let custom_factor_field = create_field(form_data.write_only(), input_data, id);

    let field_set = FieldSet {
        title: None,
        fields: vec![custom_factor_field],
    };

    let (fields_view1, _, _) = render_field_sets(vec![field_set], accessibility_always_show_option);

    let id = Id::SensitivitySludgeStorageCustomFactor;
    let custom_factor_field2 = create_field(form_data.write_only(), input_data, id);

    let field_set = FieldSet {
        title: None,
        fields: vec![custom_factor_field2],
    };

    let (fields_view2, _, _) = render_field_sets(vec![field_set], accessibility_always_show_option);

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
        <Card id = "sensitivity-open-digesters" title = "Methanemissionen aus offenen Faultürmen und bei der Schlammlagerung" bg_color="bg-blue" accessibility_always_show_option>
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
            <InfoBox text = "Die Schlammlagerung trägt maßgeblich zu Methanemissionen bei" accessibility_always_show_option>
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
               let show_sludge_bags_controls_class = if show_sludge_bags_controls.get() { String::new() } else { "hidden".to_string() };
               let show_sludge_storage_containers_controls_class = if show_sludge_storage_containers_controls.get() { String::new() } else { "hidden".to_string() };
               outcome.with(|out|out.output.as_ref().map(|out|{
                 view! {
                   <dl class="mx-3 my-2 grid grid-cols-2 text-sm">
                     <dt class={ format!("text-lg font-semibold text-right px-3 py-1 text-gray-500 {show_sludge_bags_controls_class}") }
                     >
                        "CH₄ Schlupf Schlammtaschen"
                     </dt>
                     <dd class={ format!("text-lg py-1 px-3 {show_sludge_bags_controls_class}") }
                     >
                       { format!("{:.1}", f64::from(required!(Out::Ch4SludgeBags, out).unwrap())).replace('.',",") }
                       <span class="ml-2 text-gray-400">{ "t CO₂-Äq./a" }</span>
                     </dd>
                     <dt class={ format!("text-lg font-semibold text-right px-3 py-1 text-gray-500 {show_sludge_storage_containers_controls_class}") }
                     >
                        "CH₄ Schlupf Schlammlagerung"
                     </dt>
                     <dd class={ format!("text-lg py-1 px-3 {show_sludge_storage_containers_controls_class}") } >
                       { format!("{:.1}", f64::from(required!(Out::Ch4SludgeStorageContainers, out).unwrap())).replace('.',",") }
                       <span class="ml-2 text-gray-400">{ "t CO₂-Äq./a" }</span>
                     </dd>
                     <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">"Gesamtemissionen"</dt>
                     <dd class="text-lg py-1 px-3">
                       { format!("{:.1}", f64::from(required!(Out::TotalEmissions, out).unwrap())).replace('.',",") }
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

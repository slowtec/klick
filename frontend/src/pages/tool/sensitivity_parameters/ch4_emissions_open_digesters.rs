use leptos::*;
use leptos_fluent::*;

use klick_app_components::forms::*;
use klick_boundary::FormData;
use klick_domain::{
    required_output_value_id as required, InputValueId as Id, OutputValueId as Out, Value,
};

use crate::current_lang;
use crate::pages::tool::{
    fields::create_field, CalculationOutcome, Card, Cite, InfoBox, DWA_MERKBLATT_URL,
};
use klick_presenter::{Lng, ValueLabel};

#[allow(clippy::too_many_lines)] // TODO
#[component]
pub fn CH4EmissionsOpenDigesters(
    form_data: RwSignal<FormData>,
    sensitivity_outcome: Signal<CalculationOutcome>,
    accessibility_always_show_option: Option<RwSignal<bool>>,
    lang: Lng,
) -> impl IntoView {
    let show_sludge_bags_controls = Signal::derive(move || {
        // a better way could be to check out.co2_equivalents.ch4_sludge_bags > 0.0
        form_data.with(|d| {
            !d.get(&Id::ProfileSludgeBagsAreOpen)
                .cloned()
                .map(Value::as_bool_unchecked)
                .is_some_and(|v| !v)
        })
    });
    let show_sludge_storage_containers_controls = Signal::derive(move || {
        // a better way could be to check out.co2_equivalents.ch4_sludge_storage_containers > 0.0
        form_data.with(|d| {
            !d.get(&Id::ProfileSludgeStorageContainersAreOpen)
                .cloned()
                .map(Value::as_bool_unchecked)
                .is_some_and(|v| !v)
        })
    });
    let show_dialog = Signal::derive(move || {
        let digester_count = form_data.with(|d| {
            d.get(&Id::ProfileSludgeDigesterCount)
                .cloned()
                .map(Value::as_count_unchecked)
                .map(u64::from)
                .unwrap_or_default()
        });
        let sewage_gas_produced = form_data.with(|d| {
            d.get(&Id::ProfileSewageGasProduced)
                .cloned()
                .map(Value::as_qubicmeters_unchecked)
                .map(f64::from)
                .unwrap_or_default()
        });
        (show_sludge_bags_controls.get() || show_sludge_storage_containers_controls.get())
            && (sewage_gas_produced > 0.0 || digester_count > 0)
    });

    let id = Id::SensitivitySludgeBagsCustomFactor;

    let custom_factor_field = create_field(form_data.write_only(), form_data.into(), id);

    let field_set = FieldSet {
        title: None,
        fields: vec![custom_factor_field],

        draw_border: false,
    };

    let (fields_view1, _, _) = render_field_sets(
        vec![field_set],
        accessibility_always_show_option,
        current_lang(),
    );

    let id = Id::SensitivitySludgeStorageCustomFactor;
    let custom_factor_field2 = create_field(form_data.write_only(), form_data.into(), id);

    let field_set = FieldSet {
        title: None,
        fields: vec![custom_factor_field2],

        draw_border: false,
    };

    let (fields_view2, _, _) = render_field_sets(
        vec![field_set],
        accessibility_always_show_option,
        current_lang(),
    );

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
        <Card id = "sensitivity-open-digesters" title = move_tr!("sensitivity-open-digesters").get() bg_color="bg-blue" accessibility_always_show_option>
          <div class = move || { if show_sludge_bags_controls.get() { None } else { Some("hidden") } } >
             <p class="my-2">
               <div inner_html={ move_tr!("sensitivity-open-digesters-p-1") }></div>
             </p>
             <p class="my-2">
               <div inner_html={ move_tr!("sensitivity-open-digesters-p-2") }></div>
             </p>
             <div class="my-4 ml-4">
               { fields_view1 }
             </div>
          </div>
          <div class = move || { if show_sludge_storage_containers_controls.get() { None } else { Some("hidden") } } >
            <InfoBox text = move_tr!("sensitivity-open-digesters_1-text").get() accessibility_always_show_option>
              <Cite source = move_tr!("sensitivity-open-digesters_1-cite-source").get() url = DWA_MERKBLATT_URL>
                { move_tr!("sensitivity-open-digesters_1-cite-text") }
              </Cite>
            </InfoBox>

           <p class="my-2">
             <div inner_html={ move_tr!("sensitivity-open-digesters-p-3") }></div>
           </p>
           <div class="my-4 ml-4">
             { fields_view2 }
           </div>
         </div>

         <div class="border-t pt-3 mt-4 border-gray-900/10">
           { move || {
               let show_sludge_bags_controls_class = if show_sludge_bags_controls.get() { String::new() } else { "hidden".to_string() };
               let show_sludge_storage_containers_controls_class = if show_sludge_storage_containers_controls.get() { String::new() } else { "hidden".to_string() };
               sensitivity_outcome.with(|out|out.output.as_ref().map(|out|{
                 view! {
                   <dl class="mx-3 my-2 grid grid-cols-2 text-sm">
                     <dt class={ format!("text-lg font-semibold text-right px-3 py-1 text-gray-500 {show_sludge_bags_controls_class}") }
                     >
                     { Out::Ch4SludgeBags.label(lang) }
                     </dt>
                     <dd class={ format!("text-lg py-1 px-3 {show_sludge_bags_controls_class}") }
                     >
                       { crate::current_lang().get().format_number_with_fixed_precision(f64::from(required!(Out::Ch4SludgeBags, out).unwrap()), 2) }
                       <span class="ml-2 text-gray-400">{ "t CO₂-Äq./a" }</span>
                     </dd>
                     <dt class={ format!("text-lg font-semibold text-right px-3 py-1 text-gray-500 {show_sludge_storage_containers_controls_class}") }
                     >
                     { Out::Ch4SludgeStorageContainers.label(lang) }
                     </dt>
                     <dd class={ format!("text-lg py-1 px-3 {show_sludge_storage_containers_controls_class}") } >
                       { crate::current_lang().get().format_number_with_fixed_precision(f64::from(required!(Out::Ch4SludgeStorageContainers, out).unwrap()), 2) }
                       <span class="ml-2 text-gray-400">{ "t CO₂-Äq./a" }</span>
                     </dd>
                     <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">{ Out::TotalEmissions.label(lang) }</dt>
                     <dd class="text-lg py-1 px-3">
                       { crate::current_lang().get().format_number_with_fixed_precision(f64::from(required!(Out::TotalEmissions, out).unwrap()), 2) }
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

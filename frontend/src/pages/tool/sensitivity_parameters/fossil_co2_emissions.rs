use leptos::*;
use leptos_fluent::*;

use klick_app_components::forms::*;
use klick_boundary::FormData;
use klick_domain::{
    required_output_value_id as required, InputValueId as Id, OutputValueId as Out,
};

use crate::pages::tool::{
    fields::create_field, CalculationOutcome, Card, Cite, InfoBox, DWA_MERKBLATT_URL,
};
use klick_presenter::{Lng, ValueLabel};

#[component]
pub fn FossilCO2Emissions(
    form_data: RwSignal<FormData>,
    sensitivity_outcome: Signal<CalculationOutcome>,
    accessibility_always_show_option: Option<RwSignal<bool>>,
    lang: Lng,
) -> impl IntoView {
    let field_set = field_set(form_data.write_only(), form_data.into());
    let (form1, _, _) = render_field_sets(
        vec![field_set],
        accessibility_always_show_option,
        crate::current_lang(),
    );

    view! {
       <Card id="sensitivity-fossil-co2" title = move_tr!("sensitivity-fossil-co2").get() bg_color="bg-blue" accessibility_always_show_option>
         <p class="my-2">
           <div inner_html={ move_tr!("sensitivity-fossil-co2-1") }></div>
         </p>
         <p class="my-2">
           <div inner_html={ move_tr!("sensitivity-fossil-co2-2") }></div>
         </p>
         <p class="my-2">
           <div inner_html={ move_tr!("sensitivity-fossil-co2-3") }></div>
         </p>
         { form1 }
         <InfoBox text = move_tr!("sensitivity-fossil-co2-infobox-text").get() accessibility_always_show_option>
             <Cite source = move_tr!("sensitivity-fossil-co2-infobox-cite-source").get() url = DWA_MERKBLATT_URL>
             { move_tr!("sensitivity-fossil-co2-infobox-cite-text") }
             </Cite>
           </InfoBox>
         <p class="my-2">
           <div inner_html={ move_tr!("sensitivity-fossil-co2-4") }></div>
         </p>
         <div class="border-t pt-3 mt-4 border-gray-900/10">
         { move || {
          sensitivity_outcome.with(|out|out.output.as_ref().map(|out|{
               let out = &out;
               view! {
                 <dl class="mx-3 my-2 grid grid-cols-2 text-sm">
                   <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">{ Out::FossilEmissions.label(lang) }</dt>
                   <dd class="text-lg py-1 px-3">
                     { crate::current_lang().get().format_number_with_fixed_precision(f64::from(required!(Out::FossilEmissions, out).unwrap()), 2) }
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
    }
}

fn field_set(form_data: WriteSignal<FormData>, input_data: Signal<FormData>) -> FieldSet {
    let id = Id::SensitivityCO2FossilCustomFactor;

    let custom_factor_field = create_field(form_data, input_data, id);

    let fields = vec![custom_factor_field];
    FieldSet {
        title: None,
        fields,
        draw_border: false,
    }
}

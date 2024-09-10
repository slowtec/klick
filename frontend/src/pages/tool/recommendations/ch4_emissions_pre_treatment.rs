use leptos::*;
use leptos_fluent::*;

use crate::pages::tool::{Card, Cite, InfoBox, DWA_MERKBLATT_URL};

pub fn options(accessibility_always_show_option: Option<RwSignal<bool>>) -> impl IntoView {
    view! {
      <Card id="recommendation-ch4-pre-treatment" title = move_tr!("recommendation-ch4-pre-treatment").get() bg_color="bg-yellow" accessibility_always_show_option>
        <p>
          <div inner_html={ move_tr!("recommendation-ch4-pre-treatment-1") }></div>
        </p>
        <ul class="ml-5 mt-4 list-disc">
          <li class="my-2">
            <InfoBox text = move_tr!("recommenations_ch4_emissions_pre_treatment_infobox_1-text").get() accessibility_always_show_option>
              <Cite source = move_tr!("recommenations_ch4_emissions_pre_treatment_infobox_1-cite-source").get() url = DWA_MERKBLATT_URL>
                <div inner_html={ move_tr!("recommenations_ch4_emissions_pre_treatment_infobox_1-cite-text") }></div>
              </Cite>
            </InfoBox>
          </li>
          <li class="my-2">
            <InfoBox text = move_tr!("recommenations_ch4_emissions_pre_treatment_infobox_2-text").get() accessibility_always_show_option>
              <Cite source = move_tr!("recommenations_ch4_emissions_pre_treatment_infobox_2-cite-source").get() url = DWA_MERKBLATT_URL>
                <div inner_html={ move_tr!("recommenations_ch4_emissions_pre_treatment_infobox_2-cite-text") }></div>
              </Cite>
            </InfoBox>
          </li>
        </ul>
      </Card>
    }
}

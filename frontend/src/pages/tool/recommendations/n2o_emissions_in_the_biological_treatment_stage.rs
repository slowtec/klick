use leptos::*;
use leptos_fluent::*;

use crate::pages::tool::{Card, Cite, DWA_MERKBLATT_URL};

pub fn options(accessibility_always_show_option: Option<RwSignal<bool>>) -> impl IntoView {
    view! {
      <Card id="recommendation-n2o-biological" title = move_tr!("recommendation-n2o-biological").get() bg_color="bg-yellow" accessibility_always_show_option>
        <p>
        <div inner_html={ move_tr!("recommendation-n2o-biological_p_1") }></div>
          <Cite source = move_tr!("recommendation-n2o-biological_1-cite-source").get() url = DWA_MERKBLATT_URL>
            <ul class="list-disc ml-5 space-y-1 text-gray-800">
              <li>
                <p>
                  <div inner_html={ move_tr!("recommendation-n2o-biological_p_2") }></div>
                </p>
              </li>
              <li>
                <p>
                  <div inner_html={ move_tr!("recommendation-n2o-biological_p_3") }></div>
                </p>
              </li>
              <li>
                <p>
                  <div inner_html={ move_tr!("recommendation-n2o-biological_p_4") }></div>
                </p>
              </li>
              <li>
                <p>
                  <div inner_html={ move_tr!("recommendation-n2o-biological_p_5") }></div>
                </p>
              </li>
              <li>
                <p>
                  <div inner_html={ move_tr!("recommendation-n2o-biological_p_6") }></div>
                </p>
              </li>
            </ul>
          </Cite>
          <div inner_html={ move_tr!("recommendation-n2o-biological_p_7") }></div>
        </p>
      </Card>
    }
}

use leptos::*;
use leptos_fluent::*;

use crate::pages::tool::{Card, Cite, InfoBox, DWA_MERKBLATT_URL};

pub fn options(accessibility_always_show_option: Option<RwSignal<bool>>) -> impl IntoView {
    view! {
      <Card id="recommendation-leak-test" title=move_tr!("recommendation-leak-test").get() bg_color="bg-yellow" accessibility_always_show_option>
      <InfoBox text = move_tr!("recommendation-leak-test_1-text").get() accessibility_always_show_option>
          <Cite source = move_tr!("recommendation-leak-test_1-cite-source").get() url = DWA_MERKBLATT_URL>
            <div inner_html={ move_tr!("recommendation-leak-test_1-cite-text") }></div>
          </Cite>
        </InfoBox>
        <InfoBox text = move_tr!("recommendation-leak-test_2-text").get() accessibility_always_show_option>
        <Cite source = move_tr!("recommendation-leak-test_2-cite-source").get() url = DWA_MERKBLATT_URL>
          <div inner_html={ move_tr!("recommendation-leak-test_2-cite-text") }></div>
        </Cite>
      </InfoBox>
        <p>
        <div inner_html={ move_tr!("recommendation-leak-test-p-1") }></div>
        </p>
      </Card>
    }
}

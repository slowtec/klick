use leptos::*;
use leptos_fluent::*;

use klick_boundary::FormData;
use klick_domain::{InputValueId as Id, Value};

use crate::pages::tool::{Card, Cite, InfoBox, DWA_MERKBLATT_URL};

#[component]
pub fn CH4EmissionsOpenSludgeStorage(
    accessibility_always_show_option: Option<RwSignal<bool>>,
    form_data: RwSignal<FormData>,
) -> impl IntoView {
    let show_dialog = Signal::derive(move || {
        let digester_count = form_data.with(|d| {
            d.get(&Id::ProfileSludgeTreatmentDigesterCount)
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
        sewage_gas_produced < 0.001 || digester_count == 0
    });
    view! {
      <div class = move || { if show_dialog.get() { None } else { Some("hidden") } } >
      <Card id = "sensitivity-sludge-storage" title = move_tr!("sensitivity-sludge-storage").get() bg_color="bg-blue" accessibility_always_show_option>
      <InfoBox text = move_tr!("ch4_emissions_open_sludge_storage_1-text").get() accessibility_always_show_option>
      <Cite source = move_tr!("ch4_emissions_open_sludge_storage_1-cite-source").get() url = DWA_MERKBLATT_URL>
        { move_tr!("ch4_emissions_open_sludge_storage_1-cite-text") }
      </Cite>
    </InfoBox>
      </Card>
      </div>
    }
}

use std::{collections::HashMap, rc::Rc};

use leptos::{
    component, create_effect, ev::MouseEvent, tracing, view, IntoView, RwSignal, Show, SignalGet,
    SignalSet, SignalUpdate,
};
use strum::IntoEnumIterator;

use klick_boundary::{InputData, N2OSzenario, ValueId};
use klick_svg_charts::BarChart;

use crate::{
    forms::{self, FieldSignal},
    sankey,
};

mod example_data;
mod fields;

const CHART_ELEMENT_ID: &str = "chart";

#[component]
#[allow(clippy::too_many_lines)]
pub fn Tool() -> impl IntoView {
    let field_sets = fields::field_sets();

    let (signals, set_views) = forms::render_field_sets(field_sets);
    let signals = Rc::new(signals);

    let input_data = RwSignal::new(Option::<InputData>::None);
    let sankey_header = RwSignal::new(String::new());
    let selected_scenario = RwSignal::new(Option::<u64>::Some(0));
    let barchart_arguments: RwSignal<Vec<klick_svg_charts::BarChartArguments>> =
        RwSignal::new(vec![]);

    let s = Rc::clone(&signals);
    create_effect(move |_| {
        let data = read_input_fields(&s);
        input_data.set(data);
    });

    let s = Rc::clone(&signals);

    create_effect(move |_| {
        if let Some(input_data) = input_data.get() {
            if !input_data.custom_n2o_scenario_support && selected_scenario.get() == Some(4) {
                selected_scenario.set(Some(0));
            }
            log::debug!("Calculating with {input_data:#?}");
            let szenario_calculations = N2OSzenario::iter()
            .enumerate()
            .filter_map(|(i, szenario)| {
                let output_data =
                    klick_application::calc(&input_data.clone().into(), szenario.into());
                if selected_scenario.get() == Some(i as u64) {
                    let name_ka: String = s
                        .get(&ValueId::Name)
                        .and_then(FieldSignal::get_text)
                        .unwrap_or_else(|| "Kläranlage".to_string());

                    let ew = s
                        .get(&ValueId::Ew)
                        .and_then(FieldSignal::get_float)
                        .unwrap_or_default();

                    let einheit = "t CO₂-eq/Jahr";
                    let szenario_name = match szenario {
                        N2OSzenario::ExtrapolatedParravicini => "Extrapoliert",
                        N2OSzenario::Optimistic => "Optimistisch",
                        N2OSzenario::Pesimistic => "Pessimistisch",
                        N2OSzenario::Ipcc2019 => "IPCC 2019",
                        N2OSzenario::Custom => "Benutzerdefiniert",
                    };
                    let title = format!(
                        "{name_ka} ({ew} EW) / Treibhausgasemissionen [{einheit}] - Szenario {szenario_name}"
                    );
                    sankey_header.set(title);
                    sankey::render(output_data.clone(), CHART_ELEMENT_ID);
                }
                if szenario == N2OSzenario::Custom
                    && !input_data.custom_n2o_scenario_support
                {
                    None
                } else {
                    Some((szenario, output_data))
                }
            })
            .collect::<Vec<_>>();

            barchart_arguments.set(
                szenario_calculations
                    .iter()
                    .map(|(szenario, d)| klick_svg_charts::BarChartArguments {
                        label: Some(match szenario {
                            N2OSzenario::ExtrapolatedParravicini => "Extrapoliert",
                            N2OSzenario::Optimistic => "Optimistisch",
                            N2OSzenario::Pesimistic => "Pessimistisch",
                            N2OSzenario::Ipcc2019 => "IPCC 2019",
                            N2OSzenario::Custom => "Benutzerdefiniert",
                        }),
                        co2_data: d.emissionen_co2_eq,
                        n2o_factor: d.ef_n2o_anlage,
                    })
                    .collect(),
            );
        } else {
            sankey_header.set(String::new());
            barchart_arguments.update(std::vec::Vec::clear);
            sankey::clear(CHART_ELEMENT_ID);
        }
    });

    view! {
      <div class="space-y-12">
        <div class="flex items-center justify-end gap-x-6">
          <Button
            label = "alle Werte löschen"
            on_click = {
              let signals = Rc::clone(&signals);
              move |_| {
                for s in signals.values() { s.clear(); }
              }
            }
          />
          <Button
            label = "Beispielwerte laden"
            on_click = {
              let signals = Rc::clone(&signals);
              move |_| {
                example_data::load_example_field_signal_values(&signals);
              }
            }
          />
        </div>
        { set_views }
      </div>
      // bar diagram
      { move ||
        {
          if barchart_arguments.get().is_empty() {
            None
          } else {
            Some(view! {
              <h3 class="my-8 text-xl font-bold">"Szenarien im Vergleich - Treibhausgasemissionen [t CO₂-eq/Jahr]"</h3>
              <div class="">
                <BarChart
                  width = 1200.0
                  height = 400.0
                  data  = barchart_arguments.into()
                  selected_bar = selected_scenario
                />
              </div>
            })
          }
        }
      }
      // sankey diagram
      <Show when= move || { sankey_header.get() != ""}>
      <h3 class="my-8 text-xl font-bold">
      { move ||
         sankey_header.get().to_string()
      }
      </h3>

      </Show>
      <div id={ CHART_ELEMENT_ID } class="mt-8"></div>
    }
}

#[component]
fn Button<F>(label: &'static str, on_click: F) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    view! {
      <button
        type="button"
        on:click = on_click
        class="rounded bg-primary px-2 py-1 text-sm font-semibold text-black shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">
        { label }
      </button>
    }
}

#[allow(clippy::too_many_lines)]
fn read_input_fields(s: &HashMap<ValueId, FieldSignal>) -> Option<InputData> {
    let Some(ew) = s.get(&ValueId::Ew).and_then(FieldSignal::get_float) else {
        return None;
    };
    let Some(abwasser) = s.get(&ValueId::Flow).and_then(FieldSignal::get_float) else {
        return None;
    };
    let Some(n_ges_zu) = s.get(&ValueId::TknZu).and_then(FieldSignal::get_float) else {
        return None;
    };
    let Some(csb_ab) = s.get(&ValueId::CsbAb).and_then(FieldSignal::get_float) else {
        return None;
    };
    let Some(n_ges_ab) = s.get(&ValueId::TknAb).and_then(FieldSignal::get_float) else {
        return None;
    };
    let Some(klaergas_gesamt) = s.get(&ValueId::Klaergas).and_then(FieldSignal::get_float) else {
        return None;
    };
    let Some(methangehalt) = s
        .get(&ValueId::Methangehalt)
        .and_then(FieldSignal::get_float)
    else {
        return None;
    };
    let Some(strombedarf) = s
        .get(&ValueId::Strombedarf)
        .and_then(FieldSignal::get_float)
    else {
        return None;
    };
    let Some(energie_eigen) = s.get(&ValueId::Eigenstrom).and_then(FieldSignal::get_float) else {
        return None;
    };
    let Some(ef_co2_strommix) = s.get(&ValueId::EfStrommix).and_then(FieldSignal::get_float) else {
        return None;
    };
    let Some(schlammtaschen) = s
        .get(&ValueId::Schlammtaschen)
        .and_then(FieldSignal::get_bool)
    else {
        return None;
    };
    let Some(schlammstapel) = s
        .get(&ValueId::Schlammstapel)
        .and_then(FieldSignal::get_bool)
    else {
        return None;
    };
    let Some(klaerschlamm_transport_km) = s
        .get(&ValueId::KlaerschlammTransport)
        .and_then(FieldSignal::get_float)
    else {
        return None;
    };
    let Some(klaerschlamm_entsorgung_m) = s
        .get(&ValueId::KlaerschlammEnstorgung)
        .and_then(FieldSignal::get_float)
    else {
        return None;
    };
    let Some(betriebsstoffe_fe3) = s
        .get(&ValueId::BetriebsstoffeFe3)
        .and_then(FieldSignal::get_float)
    else {
        return None;
    };
    let Some(betriebsstoffe_feso4) = s
        .get(&ValueId::BetriebsstoffeFeso4)
        .and_then(FieldSignal::get_float)
    else {
        return None;
    };
    let Some(betriebsstoffe_kalk) = s
        .get(&ValueId::BetriebsstoffeKalk)
        .and_then(FieldSignal::get_float)
    else {
        return None;
    };
    let Some(betriebsstoffe_poly) = s
        .get(&ValueId::BetriebsstoffePoly)
        .and_then(FieldSignal::get_float)
    else {
        return None;
    };
    let Some(custom_n2o_scenario_support) = s
        .get(&ValueId::CustomN2oScenarioSupport)
        .and_then(FieldSignal::get_bool)
    else {
        return None;
    };
    let Some(custom_n2o_scenario_value) = s
        .get(&ValueId::CustomN2oSzenarioValue)
        .and_then(FieldSignal::get_float)
    else {
        return None;
    };

    Some(InputData {
        ew,
        abwasser,
        n_ges_zu,
        csb_ab,
        n_ges_ab,
        klaergas_gesamt,
        methangehalt,
        strombedarf,
        energie_eigen,
        ef_co2_strommix,
        schlammtaschen,
        schlammstapel,
        klaerschlamm_transport_km,
        klaerschlamm_entsorgung_m,
        betriebsstoffe_fe3,
        betriebsstoffe_feso4,
        betriebsstoffe_kalk,
        betriebsstoffe_poly,
        custom_n2o_scenario_support,
        custom_n2o_scenario_value,
    })
}

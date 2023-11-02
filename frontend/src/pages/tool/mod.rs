use std::{collections::HashMap, rc::Rc};

use leptos::{ev::MouseEvent, *};
use strum::IntoEnumIterator;

use klick_boundary::{InputData, N2OSzenario, ValueId};
use klick_svg_charts::Barchart;

use crate::{
    forms::{self, FieldSignal},
    sankey,
};

mod example_data;
mod fields;
mod util;

const CHART_ELEMENT_ID: &str = "chart";

#[component]
pub fn Tool() -> impl IntoView {
    let field_sets = fields::field_sets();

    let (signals, set_views) = forms::render_field_sets(field_sets);
    let signals = Rc::new(signals);

    let input_data = RwSignal::new(Option::<InputData>::None);
    let szenario_comparison = RwSignal::new(vec![]);

    let s = Rc::clone(&signals);
    create_effect(move |_| {
        let data = read_input_fields(&s);
        input_data.set(data);
    });

    let s = Rc::clone(&signals);
    create_effect(move |_| {
        match input_data.get() {
            Some(mut input_data) => {
                log::debug!("Calculating with {input_data:#?}");
                let output_data = klick_application::calc(&input_data.clone().into());
                log::debug!("Result is {output_data:#?}");

                let name_ka: String = s
                    .get(&ValueId::Name)
                    .and_then(FieldSignal::get_text)
                    .unwrap_or_else(|| "Kläranlage".to_string());

                let ew = s
                    .get(&ValueId::Ew)
                    .and_then(FieldSignal::get_float)
                    .unwrap_or_default();

                sankey::render(&name_ka, ew, output_data, CHART_ELEMENT_ID);

                // Also calculate the other szenarios
                let szenario_calculations = N2OSzenario::iter()
                    .map(|szenario| {
                        input_data.n2o_szenario = szenario;
                        let output_data = klick_application::calc(&input_data.clone().into());
                        (szenario, output_data)
                    })
                    .collect::<Vec<_>>();

                let data = szenario_calculations
                    .iter()
                    .map(|(_, d)| d.emissionen_co2_eq)
                    .collect();
                szenario_comparison.set(data);
            }
            None => {
                szenario_comparison.update(|data| data.clear());
                sankey::clear(CHART_ELEMENT_ID);
            }
        }
    });

    let barchart_labels = N2OSzenario::iter()
        .map(|s| match s {
            N2OSzenario::ExtrapolatedParravicini => "Extrapoliert",
            N2OSzenario::Optimistic => "Optimistisch",
            N2OSzenario::Pesimistic => "Pessimistisch",
            N2OSzenario::Ipcc2019 => "IPCC 2019",
        })
        .collect::<Vec<_>>();

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
      <div id={ CHART_ELEMENT_ID } class="mt-8"></div>
      { move ||
        {
          let data = szenario_comparison.get();
          if !data.is_empty() {
            Some(view! {
              <h3 class="my-8 text-xl font-bold">"Szenarien im Vergleich"</h3>
              <div class="">
                <Barchart
                  width = 1200.0
                  height = 400.0
                  labels = barchart_labels.clone()
                  data = szenario_comparison.into()
                />
              </div>
            })
          } else {
            None
          }
        }
      }
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
    let Some(n2o_szenario) = s
        .get(&ValueId::N2oSzenario)
        .and_then(FieldSignal::get_selection)
    else {
        return None;
    };

    let n2o_szenario = util::try_n2o_szenario_from_usize(n2o_szenario).unwrap();

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
        n2o_szenario,
    })
}

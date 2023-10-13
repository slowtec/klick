use std::rc::Rc;

use leptos::{ev::MouseEvent, *};
use strum::IntoEnumIterator;

use klick_application::{N2OSzenario, ValueId};

use crate::{
    forms::{self, FieldSignal},
    sankey,
};

mod example_data;
mod fields;
mod util;

#[component]
pub fn Tool() -> impl IntoView {
    let render = create_action(
        |(output_data, ew, name_ka): &(klick_application::OutputData, f64, String)| {
            let output_data = output_data.clone();
            let name_ka = name_ka.clone();
            let ew = *ew;
            async move {
                sankey::render(&name_ka, ew, output_data, "chart");
            }
        },
    );

    let field_sets = fields::field_sets();

    let (signals, set_views) = forms::render_field_sets(field_sets);
    let signals = Rc::new(signals);

    let s = Rc::clone(&signals);
    create_effect(move |_| {
        let Some(ew) = s.get(&ValueId::Ew).and_then(FieldSignal::get_float) else {
            return;
        };
        let Some(abwasser) = s.get(&ValueId::Flow).and_then(FieldSignal::get_float) else {
            return;
        };
        let Some(n_ges_zu) = s.get(&ValueId::TknZu).and_then(FieldSignal::get_float) else {
            return;
        };
        let Some(csb_ab) = s.get(&ValueId::CsbAb).and_then(FieldSignal::get_float) else {
            return;
        };
        let Some(n_ges_ab) = s.get(&ValueId::TknAb).and_then(FieldSignal::get_float) else {
            return;
        };
        let Some(klaergas_gesamt) = s.get(&ValueId::Klaergas).and_then(FieldSignal::get_float)
        else {
            return;
        };
        let Some(methangehalt) = s
            .get(&ValueId::Methangehalt)
            .and_then(FieldSignal::get_float)
        else {
            return;
        };
        let Some(strombedarf) = s
            .get(&ValueId::Strombedarf)
            .and_then(FieldSignal::get_float)
        else {
            return;
        };
        let Some(energie_eigen) = s.get(&ValueId::Eigenstrom).and_then(FieldSignal::get_float)
        else {
            return;
        };
        let Some(ef_co2_strommix) = s.get(&ValueId::EfStrommix).and_then(FieldSignal::get_float)
        else {
            return;
        };
        let Some(schlammtaschen) = s
            .get(&ValueId::Schlammtaschen)
            .and_then(FieldSignal::get_bool)
        else {
            return;
        };
        let Some(schlammstapel) = s
            .get(&ValueId::Schlammstapel)
            .and_then(FieldSignal::get_bool)
        else {
            return;
        };
        let Some(klaerschlamm_transport_km) = s
            .get(&ValueId::KlaerschlammTransport)
            .and_then(FieldSignal::get_float)
        else {
            return;
        };
        let Some(klaerschlamm_entsorgung_m) = s
            .get(&ValueId::KlaerschlammEnstorgung)
            .and_then(FieldSignal::get_float)
        else {
            return;
        };
        let Some(betriebsstoffe_fe3) = s
            .get(&ValueId::BetriebsstoffeFe3)
            .and_then(FieldSignal::get_float)
        else {
            return;
        };
        let Some(betriebsstoffe_feso4) = s
            .get(&ValueId::BetriebsstoffeFeso4)
            .and_then(FieldSignal::get_float)
        else {
            return;
        };
        let Some(betriebsstoffe_kalk) = s
            .get(&ValueId::BetriebsstoffeKalk)
            .and_then(FieldSignal::get_float)
        else {
            return;
        };
        let Some(betriebsstoffe_poly) = s
            .get(&ValueId::BetriebsstoffePoly)
            .and_then(FieldSignal::get_float)
        else {
            return;
        };
        let Some(n2o_szenario) = s
            .get(&ValueId::N2oSzenario)
            .and_then(FieldSignal::get_selection)
        else {
            return;
        };

        let n2o_szenario = util::try_n2o_szenario_from_usize(n2o_szenario).unwrap();

        let mut input_data = klick_application::InputData {
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
        };

        log::debug!("Calculating with {input_data:#?}");
        let output_data = klick_application::calc(&input_data);
        log::debug!("Result is {output_data:#?}");

        let name_ka: String = s
            .get(&ValueId::Name)
            .and_then(FieldSignal::get_text)
            .unwrap_or_else(|| "Kläranlage".to_string());

        let ew = s
            .get(&ValueId::Ew)
            .and_then(FieldSignal::get_float)
            .unwrap_or_default();

        render.dispatch((output_data, ew, name_ka));

        // Also calculate the other szenarios
        let _szenario_calculations = N2OSzenario::iter()
            .map(|szenario| {
                input_data.n2o_szenario = szenario;
                let output_data = klick_application::calc(&input_data);
                (szenario, output_data)
            })
            .collect::<Vec<_>>();

        // TODO: visualize
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
      <div id="chart" class="mt-8"></div>
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

#[allow(clippy::wildcard_imports)]
use leptos::*;

use crate::{
    BarChart, BarChartArguments, BarChartRadioInput, BarChartRadioInputArguments, SankeyChart,
    SankeyData,
};

use klick_presenter::Lng;

const SVG_HEADER: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>"#;

#[must_use]
pub fn bar_chart_radio_input(
    data: Vec<BarChartRadioInputArguments>,
    width: f64,
    height: f64,
    selected: Option<u64>,
    emission_factor_label: Option<&'static str>,
    lang: Lng,
) -> String {
    render_view_as_svg(move || {
        let selected_bar = RwSignal::new(selected);
        let data = data;
        view! {
          <BarChartRadioInput
            width
            height
            data
            selected_bar = selected_bar.into()
            emission_factor_label
            aria_label = None
            lang
            on_change = |_|{}
          />
        }
    })
}

#[must_use]
pub fn bar_chart(data: Vec<BarChartArguments>, width: f64, height: f64) -> String {
    render_view_as_svg(move || {
        let data = data;
        view! {
          <BarChart
            width
            height
            data
            aria_label = None
          />
        }
    })
}

#[must_use]
pub fn sankey_chart<F>(
    sankey_data: SankeyData,
    width: f64,
    height: f64,
    number_format: F,
    font_size: f64,
    aria_label: Option<String>,
) -> String
where
    F: Fn(f64) -> String + 'static,
{
    render_view_as_svg(move || {
        view! {
          <SankeyChart
            sankey_data
            width
            height
            number_format
            font_size
            aria_label
          />
        }
    })
}

fn render_view_as_svg<F, V>(view: F) -> String
where
    F: FnOnce() -> V + 'static,
    V: IntoView,
{
    let rt = create_runtime();
    let svg_string = view().into_view().render_to_string().into_owned();
    let svg_string = format!("{SVG_HEADER}{svg_string}");
    rt.dispose();
    svg_string
}

#[allow(clippy::wildcard_imports)]
use leptos::*;

use crate::{BarChart, BarChartArguments};

const SVG_HEADER: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>"#;

#[must_use]
pub fn bar_chart(
    data: Vec<BarChartArguments>,
    width: f64,
    height: f64,
    selected: Option<u64>,`
) -> String {
    let rt = create_runtime();
    let data = RwSignal::new(data);
    let selected_bar = RwSignal::new(selected);
    let view = view! {
      <BarChart
        width
        height
        data = data.into()
        selected_bar
      />
    }
    .into_view();
    let svg_string = view.render_to_string().into_owned();
    rt.dispose();
    format!("{SVG_HEADER}{svg_string}")
}

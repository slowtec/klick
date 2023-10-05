use leptos::*;

use crate::Barchart;

const SVG_HEADER: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>"#;

pub fn barchart(data: Vec<f64>, width: f64, height: f64) -> String {
    let rt = create_runtime();
    let data = create_rw_signal(data);
    let view = view! { <Barchart width height data = data.into() /> }.into_view();
    let svg_string = view.render_to_string().into_owned();
    rt.dispose();
    format!("{SVG_HEADER}{svg_string}")
}

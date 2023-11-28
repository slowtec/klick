use klick_svg_charts::{SankeyChart, SankeyData};
use leptos::*;

#[component]
fn MySankeyChart() -> impl IntoView {
    let mut sankey = SankeyData::new();

    let c = sankey.node(1.0, Some("c"));
    let a = sankey.node(5.0, Some("a"));
    let b = sankey.node(2.0, Some("b"));

    let e = sankey.node(5.0, Some("e"));
    let d = sankey.node(5.0, Some("d"));
    let g = sankey.node(7.0, Some("g"));
    let h = sankey.node(8.0, Some("h"));
    let i = sankey.node(10.0, Some("i"));
    let j = sankey.node(5.0, Some("j"));
    let k = sankey.node(23.0, Some("k"));
    let f = sankey.node(5.0, Some("f"));

    sankey.edge(a, g);
    sankey.edge(b, g);

    sankey.edge(c, h);
    sankey.edge(g, h);

    sankey.edge(d, i);
    sankey.edge(e, i);

    sankey.edge(f, j);

    sankey.edge(h, k);
    sankey.edge(i, k);
    sankey.edge(j, k);

    view! {
      <h1>"Sankey Chart Example"</h1>
      <SankeyChart
        sankey = { sankey.clone() }
        width = 800.0
        height = 300.0
      />
      <SankeyChart
        sankey
        width = 800.0
        height = 300.0
      />
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    log::info!("Start web application");
    mount_to_body(|| {
        view! { <MySankeyChart /> }
    });
}

use leptos::*;

use klick_app_charts::BarChart;

#[component]
fn MyBarChart() -> impl IntoView {
    let b1: RwSignal<Vec<klick_app_charts::BarChartArguments>> = RwSignal::new(vec![
        klick_app_charts::BarChartArguments {
            label: "Emissionen",
            value: 156.0,
            percentage: None,
        },
        klick_app_charts::BarChartArguments {
            label: "CH₄ Schlupf Schlammtasche",
            value: 6.0,
            percentage: None,
        },
        klick_app_charts::BarChartArguments {
            label: "CH₄ Schlupf Schlammlagerung",
            value: -86.0,
            percentage: None,
        },
        klick_app_charts::BarChartArguments {
            label: "CH₄ BHKW",
            value: 56.0,
            percentage: None,
        },
    ]);
    let b2: RwSignal<Vec<klick_app_charts::BarChartArguments>> = RwSignal::new(vec![
        klick_app_charts::BarChartArguments {
            label: "Emissionen",
            value: -156.0,
            percentage: None,
        },
        klick_app_charts::BarChartArguments {
            label: "CH₄ BHKW",
            value: 10.0,
            percentage: None,
        },
    ]);
    let b3: RwSignal<Vec<klick_app_charts::BarChartArguments>> = RwSignal::new(vec![
        klick_app_charts::BarChartArguments {
            label: "Emissionen",
            value: 156.0,
            percentage: None,
        },
        klick_app_charts::BarChartArguments {
            label: "CH₄ BHKW",
            value: -10.0,
            percentage: None,
        },
    ]);
    let b4: RwSignal<Vec<klick_app_charts::BarChartArguments>> = RwSignal::new(vec![]);
    let b5: RwSignal<Vec<klick_app_charts::BarChartArguments>> = RwSignal::new(vec![
        klick_app_charts::BarChartArguments {
            label: "Emissionen",
            value: 10.0,
            percentage: None,
        },
        klick_app_charts::BarChartArguments {
            label: "CH₄ Schlupf Schlammtasche",
            value: 20.0,
            percentage: None,
        },
        klick_app_charts::BarChartArguments {
            label: "CH₄ Schlupf Schlammlagerung",
            value: 30.0,
            percentage: None,
        },
        klick_app_charts::BarChartArguments {
            label: "CH₄ BHKW",
            value: 40.0,
            percentage: None,
        },
    ]);
    let b6: RwSignal<Vec<klick_app_charts::BarChartArguments>> = RwSignal::new(vec![
        klick_app_charts::BarChartArguments {
            label: "Emissionen",
            value: -10.0,
            percentage: None,
        },
        klick_app_charts::BarChartArguments {
            label: "CH₄ Schlupf Schlammtasche",
            value: -20.0,
            percentage: None,
        },
        klick_app_charts::BarChartArguments {
            label: "CH₄ Schlupf Schlammlagerung",
            value: -30.0,
            percentage: None,
        },
        klick_app_charts::BarChartArguments {
            label: "CH₄ BHKW",
            value: -40.0,
            percentage: None,
        },
    ]);
    let b7: RwSignal<Vec<klick_app_charts::BarChartArguments>> = RwSignal::new(vec![
        klick_app_charts::BarChartArguments {
            label: "Emissionen",
            value: -10.0,
            percentage: None,
        },
        klick_app_charts::BarChartArguments {
            label: "CH₄ Schlupf Schlammtasche",
            value: -20.0,
            percentage: None,
        },
        klick_app_charts::BarChartArguments {
            label: "CH₄ Schlupf Schlammlagerung",
            value: -30.0,
            percentage: None,
        },
        klick_app_charts::BarChartArguments {
            label: "CH₄ BHKW",
            value: -40.0,
            percentage: None,
        },
        // Additional entries with funny names
        klick_app_charts::BarChartArguments {
            label: "Giggly Gases",
            value: -50.0,
            percentage: None,
        },
        klick_app_charts::BarChartArguments {
            label: "Silly Slime",
            value: -60.0,
            percentage: None,
        },
        klick_app_charts::BarChartArguments {
            label: "Whimsical Wastes",
            value: -70.0,
            percentage: None,
        },
        klick_app_charts::BarChartArguments {
            label: "Laughable Leakage",
            value: -80.0,
            percentage: None,
        },
        klick_app_charts::BarChartArguments {
            label: "Chuckling Carbon",
            value: -90.0,
            percentage: None,
        },
        klick_app_charts::BarChartArguments {
            label: "Mirthful Methane",
            value: -100.0,
            percentage: None,
        },
    ]);
    view! {
      <h1>"BarChart Example b1"</h1>
      <BarChart
        width = 1200.0
        height = 400.0
        data=b1.get()
        aria_label = None
      />
      <h1>"BarChart Example b2"</h1>
      <BarChart
        width = 1200.0
        height = 400.0
        data=b2.get()
        aria_label = None
      />
      <h1>"BarChart Example b3"</h1>
      <BarChart
        width = 1200.0
        height = 400.0
        data=b3.get()
        aria_label = None
      />
      <h1>"BarChart Example b4"</h1>
      <BarChart
        width = 1200.0
        height = 400.0
        data=b4.get()
        aria_label = None
      />
      <h1>"BarChart Example b5"</h1>
      <BarChart
        width = 1200.0
        height = 400.0
        data=b5.get()
        aria_label = None
      />
      <h1>"BarChart Example b6"</h1>
      <BarChart
        width = 1200.0
        height = 400.0
        data=b6.get()
        aria_label = None
      />
      <h1>"BarChart Example b71"</h1>
      <BarChart
        width = 1200.0
        height = 400.0
        data=b7.get()
        aria_label = None
      />
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    log::info!("Start web application");
    mount_to_body(|| {
        view! { <MyBarChart /> }
    });
}

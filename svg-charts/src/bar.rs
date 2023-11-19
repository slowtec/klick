use leptos::*;

#[derive(Debug, Clone)]
pub struct Arguments {
    pub label: Option<&'static str>,
    pub co2_data: f64,
    pub n2o_factor: f64,
}

#[component]
#[allow(clippy::module_name_repetitions)]
pub fn Chart(
    width: f64,
    height: f64,
    data: Signal<Vec<Arguments>>,
    selected_bar: RwSignal<Option<u64>>,
) -> impl IntoView {
    let margin = 10.0;

    let inner_width = width - 2.0 * margin;
    let inner_height = height - 2.0 * margin;

    view! {
      <svg
        with=format!("{width}px")
        height=format!("{height}px")
        viewBox=format!("0 0 {width} {height}")
        xmlns="http://www.w3.org/2000/svg"
      >
        <g transform=format!("translate({margin},{margin})")>
          <g transform=format!("translate(0,{inner_height})")>
            <XAxis width={ inner_width } />
          </g>
          <YAxis height={ inner_height } />
          <Bars width={ inner_width } height={ inner_height } data=data selected_bar=selected_bar/>
        </g>
      </svg>
    }
}

#[component]
fn XAxis(width: f64) -> impl IntoView {
    view! {
      <line x1=0 y1=0 x2={width} y2=0 stroke-width=1 stroke="#bbb" />
    }
}

#[component]
fn YAxis(height: f64) -> impl IntoView {
    view! {
      <line x1=0 y1=0 x2=0 y2={ height } stroke-width=1 stroke="#bbb" />
    }
}

#[component]
#[allow(clippy::cast_precision_loss)]
fn Bars(
    width: f64,
    height: f64,
    data: Signal<Vec<Arguments>>,
    selected_bar: RwSignal<Option<u64>>,
) -> impl IntoView {
    let count: usize = data.get().len();
    let co2_value_max = data
        .get()
        .iter()
        .fold(0.0, |max_co2, item| f64::max(max_co2, item.co2_data));
    let gap = width * 0.01;
    let bar_width = (width - ((count + 1) as f64 * gap)) / (count as f64);

    view! {
      <For
        each = move || {
          data.get().into_iter().enumerate().map(|(i,v)|
            (i, v.label, v.co2_data, v.n2o_factor * 100.0)
          ).collect::<Vec<_>>()
        }
        key=|(i,_,_,_)| *i
        children = move |(i,label,co2_value, n2o_factor)| {
          let bar_height = (height - 4.0 * gap) * co2_value/co2_value_max;
          let dx = gap + (bar_width + gap) * i as f64;
          let dy = (height - gap) - bar_height;

          let selected_rect_dx = (gap / 2.0) + ((bar_width + gap) * i as f64);
          view! {
            // background for selected bar
            <Show when= move || { selected_bar.get() == Some(i as u64)}>
            <g transform=format!("translate({selected_rect_dx},0)")>
            <rect
              width={ bar_width + gap }
              height={ height }
              fill="#9FE2BF"
              rx=3
              ry=3
            />
            </g>
            </Show>
            // bar
            <Bar label co2_value n2o_factor dx={dx} dy={dy} bar_width={ bar_width } bar_height={ bar_height } width={ width } height={ height } i=i selected_bar/>
          }
        }
      />
    }
}

#[component]
#[allow(clippy::cast_precision_loss)]
fn Bar(
    label: Option<&'static str>,
    co2_value: f64,
    n2o_factor: f64,
    dx: f64,
    dy: f64,
    bar_width: f64,
    bar_height: f64,
    width: f64,
    height: f64,
    i: usize,
    selected_bar: RwSignal<Option<u64>>,
) -> impl IntoView {
    let hovered = create_rw_signal(false);
    let fill = RwSignal::new("#0af");
    let font_weight = RwSignal::new("normal");
    let font_size = RwSignal::new(0.0);
    let on_mouse_enter = move |_| {
        hovered.set(true);
        fill.set("#5cf");
        font_weight.set("bold");
        font_size.set(2.0);
    };
    let on_mouse_leave = move |_| {
        hovered.set(false);
        let selected_fill = if selected_bar.get() == Some(i as u64) {
            "#0076b2" // #0088cc
        } else {
            "#0af"
        };
        fill.set(selected_fill);
        font_weight.set("normal");
        font_size.set(0.0);
    };
    let co2_value_label = format_with_thousands_seperator(co2_value, ".");
    let gap = width * 0.01;
    let transparent_dx = (gap / 2.0) + ((bar_width + gap) * i as f64);
    let hovered_color = move || if hovered.get() { "grey" } else { "" };
    view! {
      <g class="bar"
        on:mouseenter = on_mouse_enter
        on:mouseleave = on_mouse_leave
        on:mousedown = move |_| {
            //info!("Bar {} clicked", i);
            selected_bar.set(Some(i as u64));
        }
        cursor="pointer"
      >
      // transparent background for mouse events
      <g transform=format!("translate({transparent_dx},0)")>
        <rect
          width={ bar_width + gap }
          height={ height }
          fill="transparent"
          stroke={ hovered_color }
          stroke-width="3"
          stroke-dasharray="0 5"
          stroke-linecap="round"
        />
      </g>
      // bar with 6.038 label above
      <g transform=format!("translate({dx},{dy})")>
      <rect
        width={ bar_width }
        height={ bar_height }
        fill= move || if selected_bar.get() == Some(i as u64)  {
          "#0076b2" // #0088cc
        } else {
          "#0af"
        }
      />
      // co2_value
      <text
        x = { bar_width/2.0 }
        y = { -10.0 }
        text-anchor = "middle"
        font-size = move || 20.0 + font_size.get()
        font-weight = move || font_weight.get()
      >
        { co2_value_label }
      </text>
      // label, i.e.: Extrapoliert, Optimistisch, IPCC 2019, Pessimistisch, ...
      {
        label.and_then(|_| {
          view! {
            <text
              x = { bar_width/2.0 }
              y = { bar_height - 25.0 }
              text-anchor = "middle"
              font-size = move || 20.0 + font_size.get()
              font-weight = "bold"
            >
              { label }
            </text>
          }.into()
        })
      }
      // n2o_factor
      {
        label.and_then(|_| {
          let n2o_factor_label = format!("{n2o_factor:.2} % N₂O").replace('.', ",");
          view! {
            <text
              x = { bar_width/2.0 }
              y = { bar_height - 5.0 }
              text-anchor = "middle"
              font-size = move || 16.0 + font_size.get()
            >
              { n2o_factor_label }
            </text>
          }.into()
        })
      }
      </g>
      </g>
    }
}

fn format_with_thousands_seperator(value: f64, seperator: &str) -> String {
    format!("{value:.0}")
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join(seperator)
}

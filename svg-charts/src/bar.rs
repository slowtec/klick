use leptos::*;
#[derive(Debug, Clone)]
pub struct BarchartArguments {
    pub label: Option<&'static str>,
    pub co2_data: f64,
    pub n2o_factor: f64,
}

#[component]
pub fn Barchart(
    width: f64,
    height: f64,
    data: Signal<Vec<BarchartArguments>>,
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
          <Bars width={ inner_width } height={ inner_height } data=data />
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
fn Bars(
    width: f64,
    height: f64,
    data: Signal<Vec<BarchartArguments>>,
) -> impl IntoView {
    let count: usize = data.get().len();
    let co2_value_max = data.get().iter().fold(0.0, |max_co2, item| f64::max(max_co2, item.co2_data));
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
          view! {
            <g transform=format!("translate({dx},{dy})")>
              <Bar label co2_value n2o_factor width={ bar_width } height={ bar_height } />
            </g>
          }
        }
      />
    }
}

#[component]
fn Bar(label: Option<&'static str>, co2_value: f64, n2o_factor: f64, width: f64, height: f64) -> impl IntoView {
    let fill = create_rw_signal("#0af");
    let font_weight = create_rw_signal("normal");
    let font_size = create_rw_signal(0.0);

    let on_mouse_enter = move |_| {
        fill.set("#5cf");
        font_weight.set("bold");
        font_size.set(2.0);
    };

    let on_mouse_leave = move |_| {
        fill.set("#0af");
        font_weight.set("normal");
        font_size.set(0.0);
    };

    let co2_value_label = format_with_thousands_seperator(co2_value, ".");

    view! {
      <rect
        width={ width }
        height={ height }
        fill= move || fill.get()
        on:mouseenter = on_mouse_enter
        on:mouseleave = on_mouse_leave
      />
      // co2_value
      <text
        x = { width/2.0 }
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
              x = { width/2.0 }
              y = { height - 25.0 }
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
          let n2o_factor_label = format!("{:.2} % N₂O", n2o_factor).replace(".", ",");
          view! {
            <text
              x = { width/2.0 }
              y = { height - 5.0 }
              text-anchor = "middle"
              font-size = move || 16.0 + font_size.get()
            >
              { n2o_factor_label }
            </text>
          }.into()
        })
      }
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

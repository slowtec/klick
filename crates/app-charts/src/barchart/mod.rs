use leptos::*;

use klick_presenter::Lng;

#[derive(Debug, Clone)]
pub struct BarChartArguments {
    pub label: &'static str,
    pub value: f64,
}

#[component]
#[allow(clippy::module_name_repetitions)]
pub fn BarChart(width: f64, height: f64, data: Vec<BarChartArguments>) -> impl IntoView {
    let margin = 10.0;

    let inner_width = width - 2.0 * margin;
    let inner_height = height - 2.0 * margin;

    let x_axis_position = height / 2.0;

    view! {
      <svg
        with=format!("{width}px")
        height=format!("{height}px")
        viewBox=format!("0 0 {width} {height}")
        xmlns="http://www.w3.org/2000/svg"
      >
        <g transform=format!("translate({margin},{margin})")>
          <g transform=format!("translate(0,{x_axis_position})")>
            <XAxis width={ inner_width } height={ inner_height } />
          </g>
            <YAxis height={ inner_height } />
          <Bars
            width = { inner_width }
            height = { inner_height }
            x_axis_position
            data
          />
        </g>
      </svg>
    }
}

#[component]
fn XAxis(width: f64, height: f64) -> impl IntoView {
    view! {
      <line x1=0 y1={0} x2={width} y2={0} stroke-width=1 stroke="#bbb" />
    }
}

#[component]
fn YAxis(height: f64) -> impl IntoView {
    view! {
      <line x1=10.0 y1=0 x2=10.0 y2={ height } stroke-width=1 stroke="#bbb" />
    }
}

#[component]
#[allow(clippy::cast_precision_loss)]
fn Bars(
    width: f64,
    height: f64,
    data: Vec<BarChartArguments>,
    x_axis_position: f64,
) -> impl IntoView {
    let count: usize = data.len();
    let value_max = data
        .iter()
        .fold(10.0, |current_max, item| f64::max(current_max, item.value));
    let value_min = data
        .iter()
        .fold(-10.0, |current_min, item| f64::min(current_min, item.value));
    let value_range = value_max - value_min;
    let gap = width * 0.01;
    let bar_width = (width - ((count + 1) as f64 * gap)) / (count as f64);

    view! {
      <For
        each = move || {
          data.iter().enumerate().map(|(i,v)|
            (i, v.label, v.value)
          ).collect::<Vec<_>>()
        }
        key=|(i,_,_)| *i
        children = move |(i, label, value)| {
          let bar_height = (height - 4.0 * gap) * (value/value_max).abs() * 0.5;
          let dx = gap + (bar_width + gap) * i as f64;


          let (dy, label_position) = if value < 0.0 {
            (x_axis_position, LabelPosition::Bottom)
          } else {
            (x_axis_position - bar_height , LabelPosition::Top)
          };

          // let selected_rect_dx = (gap / 2.0) + ((bar_width + gap) * i as f64);
          view! {
            // background for selected barchart
            // <Show when= move || { selected_bar.get() == Some(i as u64)}>
            //   <g transform=format!("translate({selected_rect_dx},0)")>
            //     <rect
            //       width={ bar_width + gap }
            //       height={ height }
            //       fill="#9FE2BF"
            //       rx=3
            //       ry=3
            //     />
            //   </g>
            // </Show>
            <Bar
              i
              label
              value
              dx={dx}
              dy={dy}
              bar_width={ bar_width }
              bar_height={ bar_height }
              width={ width }
              height={ height }
              label_position
            />
          }
        }
      />
    }
}

enum LabelPosition {
    Top,
    Bottom,
}

#[component]
#[allow(clippy::cast_precision_loss)]
fn Bar(
    label: &'static str,
    value: f64,
    dx: f64,
    dy: f64,
    bar_width: f64,
    bar_height: f64,
    width: f64,
    height: f64,
    i: usize,
    label_position: LabelPosition,
) -> impl IntoView {
    let hovered = create_rw_signal(false);
    let fill = RwSignal::new("#0af");
    let font_weight = RwSignal::new("bold");
    let font_size = RwSignal::new(0.0);
    let gap = width * 0.01;
    let transparent_dx = (gap / 2.0) + ((bar_width + gap) * i as f64);
    let hovered_color = move || if hovered.get() { "grey" } else { "" };

    let label_dx = dx;
    let label_dy = match label_position {
        LabelPosition::Top => dy - 10.0,
        LabelPosition::Bottom => dy + bar_height + 16.0 + 20.0,
    };

    view! {
      <g class="barchart">
        // barchart with 6.038 label above
        <g transform=format!("translate({dx},{dy})")>
          <rect
            width={ bar_width }
            height={ bar_height }
            fill= { if value > 0.0 { "red" } else { "green" }}
          />
        </g>
        <g transform=format!("translate({label_dx},{label_dy})")>
          // label, i.e.: CH₄ BHKW
          <text
            x = { bar_width/2.0 }
            y = { -10.0 }
            text-anchor = "middle"
            font-size = move || 16.0 + font_size.get()
            font-weight = move || font_weight.get()
          >
            { label }
          </text>
        </g>
      </g>
        <g transform=format!("translate({label_dx},0)")>
          // value 23.2
          {
              let value_as_label = format!("{value:.2}").replace('.', ",");
              view! {
                <text
                  x = { bar_width/2.0 }
                  y = { bar_height - 5.0 }
                  text-anchor = "middle"
                  font-size = move || 16.0 + font_size.get()
                >
                  { value_as_label }
                </text>
              }
          }
        </g>
    }
}

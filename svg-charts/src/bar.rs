use leptos::*;

#[component]
pub fn Barchart(width: f64, height: f64, data: Signal<Vec<f64>>) -> impl IntoView {
    let margin = 10.0;

    let inner_width = width - 2.0 * margin;
    let inner_height = height - 2.0 * margin;

    view! {
      <svg with=format!("{width}px") height=format!("{height}px") viewBox = format!("0 0 {width} {height}") xmlns="http://www.w3.org/2000/svg">
        <g transform=format!("translate({margin},{margin})")>
          <g transform=format!("translate(0,{inner_height})")>
            <XAxis width={ inner_width } />
          </g>
          <YAxis height={ inner_height } />
         <Bars width={ inner_width } height={ inner_height } data />
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
fn Bars(width: f64, height: f64, data: Signal<Vec<f64>>) -> impl IntoView {
    let data_count: usize = data.with(|d| d.len());
    let value_max = data.with(|d| d.iter().cloned().fold(0.0, f64::max));
    let gap = width * 0.01;
    let bar_width = (width - ((data_count + 1) as f64 * gap)) / (data_count as f64);

    view! {
      <For
        each = move || { data.get().into_iter().enumerate().collect::<Vec<_>>() }
        key=|(i,_)| *i
        children = move |(i,value)| {
          let bar_height = (height - 4.0 * gap) * value/value_max;
          let dx = gap + (bar_width + gap) * i as f64;
          let dy = (height - gap) - bar_height;
          view! {
            <g transform=format!("translate({dx},{dy})")>
              <Bar value width={ bar_width } height={ bar_height } />
            </g>
          }
        }
      />
    }
}

#[component]
fn Bar(value: f64, width: f64, height: f64) -> impl IntoView {
    let fill = create_rw_signal("#0af");
    let font_weight = create_rw_signal("normal");
    let font_size = create_rw_signal(20.0);

    let on_mouse_enter = move |_| {
        fill.set("#5cf");
        font_weight.set("bold");
        font_size.set(23.0);
    };

    let on_mouse_leave = move |_| {
        fill.set("#0af");
        font_weight.set("normal");
        font_size.set(20.0);
    };

    view! {
      <text
        x = { width/2.0 }
        y = { -10.0 }
        text-anchor = "middle"
        font-size = move || font_size.get()
        font-weight = move || font_weight.get()
      >
        { value }
      </text>
      <rect
        width={ width }
        height={ height }
        fill= move || fill.get()
        on:mouseenter = on_mouse_enter
        on:mouseleave = on_mouse_leave
      />
    }
}

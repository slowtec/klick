use leptos::*;

use leptos_codemirror::{CodeMirror, ErrorMarker};

#[component]
fn MyCodemirror() -> impl IntoView {
    let input = RwSignal::new(None);

    let on_change = Callback::new(|txt| {
        log::debug!("CodeMirror editor value changed: {txt:?}");
    });

    let errors = RwSignal::new(Vec::new());

    view! {
      <h1>"Codemirror Example"</h1>
      <CodeMirror
        input = input.read_only()
        on_change
        errors = errors.read_only()
      />
      <button
        on:click = move |_| {
          errors.update(|errors|errors.push(ErrorMarker { line: 0.into() }));
        }
      >
        "set error marker"
      </button>
      <button
        on:click = move |_| {
          errors.update(|errors|{errors.clear(); });
        }
      >
        "clear error marker"
      </button>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    log::info!("Start web application");
    mount_to_body(|| {
        view! { <MyCodemirror /> }
    });
}

use gloo_storage::{SessionStorage, Storage};
use leptos::*;

use klick_app_components::icons;

pub const DWA_MERKBLATT_URL: &str =
    "https://shop.dwa.de/DWA-M-230-1-Treibhausgasemissionen-10-2022/M-230-T1-22";

#[component]
pub fn InfoBox(
    text: &'static str,
    children: Children,
    accessibility_always_show_option: Option<RwSignal<bool>>,
) -> impl IntoView {
    let show = RwSignal::<bool>::new(false);
    let accessibility_always_show =
        Signal::derive(move || match accessibility_always_show_option {
            Some(sig) => sig.get(),
            None => false,
        });
    let combined_show_signal = Signal::derive(move || match accessibility_always_show_option {
        Some(sig) => show.get() || sig.get(),
        None => show.get(),
    });
    let children = children();
    view! {
      <p>{ text }
        <div
          class = move || if accessibility_always_show.get() { Some("hidden") } else { Some("mx-1 cursor-pointer inline-block") }
          on:click = move |_| show.update(|x|*x = !*x)
        >
          <icons::InformationCircle />
        </div>
      </p>
      <div class = move || if combined_show_signal.get() { None } else { Some("hidden") } >
        { children }
      </div>
    }
}

#[component]
pub fn Card(
    id: &'static str,
    title: &'static str,
    children: Children,
    bg_color: &'static str,
    accessibility_always_show_option: Option<RwSignal<bool>>,
) -> impl IntoView {
    let hide = RwSignal::<bool>::new(false);
    let combined_hide_signal = Signal::derive(move || match accessibility_always_show_option {
        Some(sig) => !sig.get() && hide.get(),
        None => hide.get(),
    });

    // TODO:
    // This is a bit of a hack, but it's ok for prototyping now.
    // In the long term we need a dedicated model for the "view state"
    // that is not the same as the data model.
    let hidden_state_ss_id = card_id_to_session_store_hidden_state_id(id);

    if let Ok(Some(state)) = SessionStorage::get(&hidden_state_ss_id) {
        hide.set(state);
    }

    Effect::new(move |_| {
        if let Err(err) = SessionStorage::set(&hidden_state_ss_id, hide.get()) {
            log::warn!("Unable to store card hidden state: {err}");
        }
    });

    let children = children();

    view! {
      <div
        class="mt-8 divide-y divide-gray-200 overflow-hidden rounded-lg bg-white shadow-md"
      >
        <div
          class = {format!("px-4 py-3 {bg_color} cursor-pointer flex items-center justify-between") }
          on:click = move |_| hide.update(|h| *h = !*h)
        >
          <a href="#" tabindex="0"><h3 class="font-bold text-lg">{ title }</h3></a>
          <svg
            class = move || if combined_hide_signal.get() { "w-3 h-3 rotate-180 shrink-0" } else { "w-3 h-3 shrink-0" }
            aria-hidden="true"
            fill="none"
            viewBox="0 0 10 6"
          >
            <path
              stroke="currentColor"
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M9 5 5 1 1 5"
            />
          </svg>
        </div>
        <div
          class = move || if combined_hide_signal.get() { "hidden" } else { "px-4 py-4 sm:px-6 text-md" }
        >
          { children }
        </div>
      </div>
    }
}

fn card_id_to_session_store_hidden_state_id(id: &str) -> String {
    format!("card-hidden-state-{id}")
}

#[component]
pub fn Cite(source: &'static str, url: &'static str, children: Children) -> impl IntoView {
    view! {
      <p class="mt-4 mb-2 mx-3 px-3 border-solid border-l-8 border-slate-50 bg-slate-50 italic">
        { children() }
        <span class="block mt-2 mb-3 not-italic text-right text-sm font-mono">
          <a target="_blank" href = {url} >{ source }</a>
        </span>
      </p>
    }
}

use leptos::{ev, *};

use klick_boundary::json_api;

pub const DEFAULT_INPUT_CLASS: &str = "form-control block w-full px-3 py-1.5 text-base font-normal text-gray-700 bg-white bg-clip-padding border border-solid border-gray-300 rounded transition ease-in-out m-0 focus:text-gray-700 focus:bg-white focus:outline-none";

pub const DISABLED_INPUT_CLASS: &str = "form-control block w-full px-3 py-1.5 text-base font-normal text-gray-400 bg-gray-50 bg-clip-padding border border-solid border-gray-300 rounded transition ease-in-out m-0 focus:outline-none";

pub const DEFAULT_BUTTON_CLASS: &str = "cursor-pointer inline-block px-6 py-2.5 font-medium text-xs leading-tight uppercase rounded shadow-md hover:shadow-lg focus:shadow-lg focus:outline-none focus:ring-0 active:shadow-lg transition duration-150 ease-in-out w-full mb-3 bg-highlight";

pub const DISABLED_BUTTON_CLASS: &str = "inline-block px-6 py-2.5 font-medium text-xs leading-tight uppercase rounded shadow-md focus:outline-none focus:ring-0 transition duration-150 ease-in-out w-full mb-3 bg-gray-200";
#[component]

pub fn CredentialsForm(
    title: &'static str,
    description: &'static str,
    action_label: &'static str,
    initial_credentials: json_api::Credentials,
    action: Action<(String, String), ()>,
    error: Signal<Option<String>>,
    disabled: Signal<bool>,
) -> impl IntoView {
    let json_api::Credentials { email, password } = initial_credentials;
    let (email, set_email) = create_signal(email);
    let (password, set_password) = create_signal(password);

    let dispatch_action = move || action.dispatch((email.get(), password.get()));

    let button_is_disabled = Signal::derive(move || {
        disabled.get() || password.get().is_empty() || email.get().is_empty()
    });

    view! {
        <form on:submit=|ev| ev.prevent_default()>
            <div class="text-center">
              <h4 class="text-xl font-semibold mt-1 mb-12 pb-1">{ title }</h4>
            </div>
            <p class="mb-4 text-gray-600">{ description }</p>
            {move || error.get().map(|err| view! {
              <p class="mb-4 text-red-700">{ err }</p>
            })}
            <div class="mb-4">
              <input
                  type="email"
                  class=move || if disabled.get() { DISABLED_INPUT_CLASS } else { DEFAULT_INPUT_CLASS }
                  required
                  placeholder="E-Mail Adresse"
                  prop:disabled=move || disabled.get()
                  on:keyup=move |ev: ev::KeyboardEvent| {
                      let val = event_target_value(&ev);
                      set_email.update(|v| *v = val);
                  }
                  on:change=move |ev| {
                      let val = event_target_value(&ev);
                      set_email.update(|v| *v = val);
                  }
              />
            </div>
            <div class="mb-4">
              <input
                  type="password"
                  class=move || if disabled.get() { DISABLED_INPUT_CLASS } else { DEFAULT_INPUT_CLASS }
                  required
                  placeholder="Passwort"
                  prop:disabled=move || disabled.get()
                  on:keyup=move |ev: ev::KeyboardEvent| {
                      match &*ev.key() {
                          "Enter" => {
                              dispatch_action();
                          }
                          _ => {
                              let val = event_target_value(&ev);
                              set_password.update(|p| *p = val);
                          }
                      }
                  }
                  on:change=move |ev| {
                      let val = event_target_value(&ev);
                      set_password.update(|p| *p = val);
                  }
              />
            </div>
            <div class="text-center pt-1 mb-12 pb-1">
              <button
                prop:disabled=move || button_is_disabled.get()
                on:click=move |_| dispatch_action()
                class=move || if disabled.get() { DISABLED_BUTTON_CLASS } else { DEFAULT_BUTTON_CLASS }
              >
                { action_label }
              </button>
            </div>
        </form>
    }
}

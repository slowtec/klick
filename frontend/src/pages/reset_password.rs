use leptos::*;
use leptos_router::*;

use klick_components::message::*;

use crate::{
    api::{self, UnauthorizedApi},
    credentials::{
        DEFAULT_BUTTON_CLASS, DEFAULT_INPUT_CLASS, DISABLED_BUTTON_CLASS, DISABLED_INPUT_CLASS,
    },
};

#[derive(Params, PartialEq, Debug, Clone)]
struct Query {
    token: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
struct Token(String);

#[derive(Debug, Clone, PartialEq)]
enum State {
    Initial,
    InvalidToken,
    ValidToken(Token),
    WaitForResponse,
    Success,
    Failed(Token),
}

#[component]
pub fn ResetPassword(api: UnauthorizedApi) -> impl IntoView {
    let error = RwSignal::new(None::<String>);
    let state = RwSignal::new(State::Initial);
    let disabled = Signal::derive(move || state.get() == State::WaitForResponse);

    let query = use_query::<Query>();

    let reset_action = create_action(move |(token, password): &(Token, String)| {
        let token = token.clone();
        let pwd = password.clone();

        async move {
            state.set(State::WaitForResponse);
            error.set(None);
            let result = api.reset_password(token.0.clone(), pwd).await;

            match result {
                Ok(_) => {
                    error.set(None);
                    state.set(State::Success);
                }
                Err(err) => {
                    let msg = match err {
                        api::Error::Fetch(js_err) => {
                            log::error!("{js_err:?}");
                            "Ein Kommunikationsfehler ist aufgetreten".to_string()
                        }
                        api::Error::Api(err) => err
                            .message
                            .unwrap_or_else(|| "Unbekannter fehler".to_string()),
                    };
                    log::error!("Unable to reset password: {msg}");
                    error.set(Some(msg));
                    state.set(State::Failed(token));
                }
            }
        }
    });

    create_effect(move |_| {
        let Ok(query) = query.get() else {
            state.set(State::InvalidToken);
            return;
        };
        let Some(t) = query.token else {
            state.set(State::InvalidToken);
            return;
        };
        // TODO: decode nounce here
        state.set(State::ValidToken(Token(t)));
    });

    view! {
      <section>
        <div class="container py-12 px-6 mx-auto">
          <div class="flex justify-center items-center flex-wrap h-full g-6 text-gray-800">
            <div class="xl:w-6/12">
              <div class="block bg-white shadow-lg rounded-lg">
                <div class="lg:flex lg:flex-wrap g-0">
                  <div class="px-4 md:px-0 mx-auto">
                    <div class="md:p-12 md:mx-6">
                    { move || match state.get() {
                        State::Initial | State::WaitForResponse => view!{
                            <InfoMessage message = "Einen moment bitte..." />
                        }.into_view(),
                        State::InvalidToken => view! {
                            <ErrorMessage message = "Ungültiger link" />
                        }.into_view(),
                        State::ValidToken(token) |
                        State::Failed(token) => view! {
                            <ResetForm
                              token
                              error = error.into()
                              disabled
                              action = reset_action
                            />
                        }.into_view(),
                        State::Success => view! {
                            <SuccessMessage message = "Ihr Passwort wurde erfolgreich zurück gesetzt." />
                        }.into_view(),
                      }
                    }
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </section>
    }
}

#[component]
fn ResetForm(
    token: Token,
    action: Action<(Token, String), ()>,
    error: Signal<Option<String>>,
    disabled: Signal<bool>,
) -> impl IntoView {
    let new_password = RwSignal::new(String::new());
    let button_is_disabled =
        Signal::derive(move || disabled.get() || new_password.get().is_empty());

    let dispatch_action = move || {
        action.dispatch((token.clone(), new_password.get()));
    };

    let on_button_click = dispatch_action.clone();

    view! {
        <form on:submit=|ev| ev.prevent_default()>
            <div class="text-center">
              <h4 class="text-xl font-semibold mt-1 mb-12 pb-1">"Neues Passwort"</h4>
            </div>
            <p class="mb-4 text-gray-600">"Bitte geben Sie ihr neues Passwort ein"</p>
            { move || error.get().map(|err| view! {
              <p class="mb-4 text-red-700">{ err }</p>
            })}
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
                              new_password.update(|p| *p = val);
                          }
                      }
                  }
                  on:change=move |ev| {
                      let val = event_target_value(&ev);
                      new_password.update(|p| *p = val);
                  }
              />
            </div>
            <div class="text-center pt-1 mb-12 pb-1">
              <button
                prop:disabled=move || button_is_disabled.get()
                on:click=move |_| on_button_click()
                class=move || if disabled.get() { DISABLED_BUTTON_CLASS } else { DEFAULT_BUTTON_CLASS }
              >
               "Neues Passwort setzen"
              </button>
            </div>
        </form>
    }
}

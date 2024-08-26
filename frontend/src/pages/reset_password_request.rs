use leptos::*;
use leptos_fluent::*;

use crate::{
    api::{self, UnauthorizedApi},
    pages::InfoBox,
};

#[component]
pub fn ResetPasswordRequest(api: UnauthorizedApi) -> impl IntoView {
    let (wait_for_response, set_wait_for_response) = create_signal(false);
    let (request_response, set_request_response) = create_signal(None::<()>);
    let (request_error, set_request_error) = create_signal(None::<String>);
    let (email, set_email) = create_signal(String::new());
    let reset_password_action = create_action(move |(): &()| {
        let email = email.get().to_string();
        log::info!("Request password reset for {}", email);
        async move {
            set_wait_for_response.update(|w| *w = true);
            let result = api.request_password_reset(email).await;
            set_wait_for_response.update(|w| *w = false);
            match result {
                Ok(res) => {
                    set_request_response.update(|v| *v = Some(res));
                    set_request_error.update(|e| *e = None);
                }
                Err(err) => {
                    let msg = match err {
                        api::Error::Fetch(js_err) => {
                            format!("{js_err:?}")
                        }
                        api::Error::Api(err) => err
                            .message
                            .unwrap_or_else(|| "Unbekannter fehler".to_string()),
                    };
                    log::warn!("Unable to request password reset for: {msg}");
                    set_request_error.update(|e| *e = Some(msg));
                }
            }
        }
    });

    let input_is_disabled = move || request_response.get().is_some() || wait_for_response.get();
    let reset_button_is_disabled = move || input_is_disabled() || email.get().is_empty();
    let success = Signal::derive(move || request_response.get().is_some());

    view! {
      <section>
        <div class="container py-12 px-6 mx-auto">
          <div class="flex justify-center items-center flex-wrap h-full g-6 text-gray-800">
            <div class="xl:w-10/12">
              <div class="block bg-white shadow-lg rounded-lg">
                <div class="lg:flex lg:flex-wrap g-0">
                  <div class="lg:w-6/12 px-4 md:px-0">
                    <div class="md:p-12 md:mx-6">
                      <div class="text-center">
                        <h4 class="text-xl font-semibold mt-1 mb-12 pb-1">
                          { move_tr!("reset-password") }
                        </h4>
                      </div>
                      <form>
                        <p class="mb-4 text-gray-600">
                          { move_tr!("reset-password-promt") }
                        </p>
                        { move || request_error.get().map(|err| view!{
                          <p class="mb-4 text-red-700">{ err }</p>
                        })}
                        <div class="mb-4">
                          <input
                            type="email"
                            class="form-control block w-full px-3 py-1.5 text-base font-normal text-gray-700 bg-white bg-clip-padding border border-solid border-gray-300 rounded transition ease-in-out m-0 focus:text-gray-700 focus:bg-white focus:outline-none"
                            placeholder=move_tr!("email-address")
                            prop:disabled= input_is_disabled
                            on:keyup = move |ev: ev::KeyboardEvent| {
                                let val = event_target_value(&ev);
                                set_email.update(|v|*v = val);
                            }
                          />
                        </div>
                        <div class="text-center pt-1 mb-12 pb-1">
                          <button
                            class="inline-block px-6 py-2.5 font-medium text-xs leading-tight uppercase rounded shadow-md hover:text-white hover:shadow-lg focus:shadow-lg focus:outline-none focus:ring-0 active:shadow-lg transition duration-150 ease-in-out w-full mb-3 bg-highlight"
                            type="button"
                            prop:disabled = reset_button_is_disabled
                            on:click = move |_| {
                              reset_password_action.dispatch(());
                            }
                          >
                            { move_tr!("reset-password") }
                          </button>
                        </div>
                      </form>
                    </div>
                  </div>
                  <InfoBox
                    success
                    info_title = move_tr!("how-does-it-work")
                    info_description = move_tr!("reset-password-description")
                    success_title = move_tr!("email-sent-to-reset-password")
                    success_description = move_tr!("reset-request-success-message")
                  />
                </div>
              </div>
            </div>
          </div>
        </div>
      </section>
    }
}

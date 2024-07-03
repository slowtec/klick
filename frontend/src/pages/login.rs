use leptos::*;
use leptos_router::*;

use klick_boundary::json_api::{self, Credentials};

use crate::{
    api::{self, AuthorizedApi, UnauthorizedApi},
    credentials::*,
    Page,
};

#[allow(clippy::too_many_lines)] // TODO
#[component]
pub fn Login(
    api: UnauthorizedApi,
    #[prop(into)] on_success: Callback<AuthorizedApi>,
) -> impl IntoView {
    let (login_error, set_login_error) = create_signal(None::<String>);
    let (wait_for_response, set_wait_for_response) = create_signal(false);
    let show_resent_confirmation_button = RwSignal::new(false);

    let last_typed_credentials = RwSignal::new(None);

    let login_action = Action::new(move |(email, password): &(String, String)| {
        log::debug!("Try to login with {email}");
        let email = email.to_string();
        let password = password.to_string();
        let credentials = json_api::Credentials { email, password };
        last_typed_credentials.set(Some(credentials.clone()));
        async move {
            set_wait_for_response.set(true);
            let result = api.login(&credentials).await;
            set_wait_for_response.set(false);

            // TODO: clean up result handling
            match result {
                Ok(res) => {
                    set_login_error.set(None);
                    on_success.call(res);
                }
                Err(err) => {
                    let msg = match err {
                        api::Error::Fetch(js_err) => {
                            log::error!("{js_err:?}");
                            show_resent_confirmation_button.set(false);
                            "Ein Kommunikationsfehler ist aufgetreten".to_string()
                        }
                        api::Error::Api(err) => match err.details {
                            Some(json_api::login::Error::Credentials) => {
                                show_resent_confirmation_button.set(false);
                                "Email-Addresse oder Passwort ungültig".to_string()
                            }
                            Some(json_api::login::Error::EmailNotConfirmed) => {
                                show_resent_confirmation_button.set(true);
                                "Sie haben Ihre Email-Addresse noch nicht bestätigt".to_string()
                            }
                            None => {
                                show_resent_confirmation_button.set(false);
                                "Tut uns leid, irgend etwas ist schief gelaufen".to_string()
                            }
                        },
                    };
                    log::error!("Unable to login with {}: {msg}", credentials.email);
                    set_login_error.update(|e| *e = Some(msg));
                }
            }
        }
    });

    let resend_confirmation_mail = Action::new(move |(): &()| {
        show_resent_confirmation_button.set(false);
        set_login_error.set(None);

        async move {
            let Some(credentials) = last_typed_credentials.get() else {
                return;
            };
            if let Err(err) = api.resend_confirmation_mail(&credentials).await {
                log::warn!("{err}");
            }
        }
    });

    let disabled = Signal::derive(move || wait_for_response.get());

    view! {
      <section>
        <div class="container py-12 px-6 mx-auto">
          <div class="flex justify-center items-center flex-wrap h-full g-6 text-gray-800">
            <div class="xl:w-6/12">
              <div class="block bg-white shadow-lg rounded-lg">
                <div class="lg:flex lg:flex-wrap g-0">
                  <div class="px-4 md:px-0 mx-auto">
                    <div class="md:p-12 md:mx-6">
                      <CredentialsForm
                          title = "Login"
                          description="Bitte loggen Sie sich in Ihr Konto ein"
                          action_label = "Log in"
                          initial_credentials = Credentials::default()
                          action = login_action
                          error = login_error.into()
                          disabled
                      />
                      <Show when = move || show_resent_confirmation_button.get() >
                        <div class="text-center pt-1 mb-6 pb-1">
                          <button
                            on:click = move |_| resend_confirmation_mail.dispatch(())
                            class="bg-red-200 text-xs px-6 py-2.5 rounded cursor-pointer">
                            "Email zur Bestätigung erneut senden"
                          </button>
                        </div>
                      </Show>
                      <div class="text-center pt-1 mb-6 pb-1">
                        <A
                          href=Page::ResetPasswordRequest.path()
                          class="text-gray-500".to_string()>
                          "Passwort vergessen?"
                        </A>
                      </div>
                      <div class="flex items-center justify-between pb-6">
                        <p class="mb-0 mr-2 text-gray-600">"Sie haben noch kein Konto?"</p>
                        <A
                          href=Page::Register.path()
                          class="inline-block px-6 py-2 border-2 font-medium text-xs leading-tight uppercase rounded hover:bg-opacity-25 focus:outline-none focus:ring-0 transition duration-150 ease-in-out"
                        >
                          "Registrieren"
                        </A>
                      </div>
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

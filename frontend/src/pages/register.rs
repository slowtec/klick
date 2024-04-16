use leptos::*;
use leptos_router::*;

use klick_boundary::json_api::{self, Credentials};

use crate::{
    api::{self, UnauthorizedApi},
    credentials::*,
    Page,
};

#[component]
pub fn Register(api: UnauthorizedApi) -> impl IntoView {
    let (register_response, set_register_response) = create_signal(None::<()>);
    let (register_error, set_register_error) = create_signal(None::<String>);
    let (wait_for_response, set_wait_for_response) = create_signal(false);

    let register_action = create_action(move |(email, password): &(String, String)| {
        let email = email.to_string();
        let password = password.to_string();
        let credentials = json_api::Credentials { email, password };
        log::debug!("Try to register new account for {}", credentials.email);
        async move {
            set_wait_for_response.update(|w| *w = true);
            let result = api.register(&credentials).await;
            set_wait_for_response.update(|w| *w = false);
            match result {
                Ok(res) => {
                    set_register_response.update(|v| *v = Some(res));
                    set_register_error.update(|e| *e = None);
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
                    log::warn!(
                        "Unable to register new account for {}: {msg}",
                        credentials.email
                    );
                    set_register_error.update(|e| *e = Some(msg));
                }
            }
        }
    });

    let disabled =
        Signal::derive(move || wait_for_response.get() || register_response.get().is_some());
    let success = Signal::derive(move || register_response.get().is_some());

    view! {
      <section>
        <div class="container py-12 px-6 mx-auto">
          <div class="flex justify-center items-center flex-wrap h-full g-6 text-gray-800">
            <div class="xl:w-10/12">
              <div class="block bg-white shadow-lg rounded-lg">
                <div class="lg:flex lg:flex-wrap g-0">
                  <div class="lg:w-6/12 px-4 md:px-0">
                    <div class="md:p-12 md:mx-6">
                      <CredentialsForm
                          title = "Registrierung"
                          description = "Bitte geben Sie die gewünschten Anmeldeinformationen ein"
                          action_label="Registrieren"
                          initial_credentials = Credentials::default()
                          action=register_action
                          error = register_error.into()
                          disabled = { disabled }
                      />
                      <div class="flex items-center justify-between pb-6">
                        <p class="mb-0 mr-2 text-gray-600">"Sie haben bereits ein Konto?"</p>
                        <A
                          href=Page::Login.path()
                          class="inline-block px-6 py-2 border-2 font-medium text-xs leading-tight uppercase rounded hover:bg-opacity-25 focus:outline-none focus:ring-0 transition duration-150 ease-in-out"
                        >
                          "Login"
                        </A>
                      </div>
                    </div>
                  </div>
                  <InfoBox
                    success
                    info_title = "Was bietet Ihnen ein Benutzer*innenkonto?"
                    info_description = "Mit einem Konto können Sie Ihre Daten online verwalten."
                    success_title = "Erfolgreich registriert"
                    // TODO: pass children like <p>
                    success_description = "Herzlichen Glückwunsch! Sie haben Ihr Konto erfolgreich registriert. Überprüfen Sie nun Ihren E-Mail-Posteingang und bestätigen Sie die Gültigkeit Ihrer E-Mail-Adresse."
                  />
                </div>
              </div>
            </div>
          </div>
        </div>
      </section>
    }
}

#[component]
pub fn InfoBox(
    success: Signal<bool>,
    info_title: &'static str,
    info_description: &'static str,
    success_title: &'static str,
    success_description: &'static str,
) -> impl IntoView {
    const DEFAULT_CLASS: &str =
        "lg:w-6/12 flex items-center lg:rounded-r-lg rounded-b-lg lg:rounded-bl-none bg-gray-100";
    const SUCCESS_CLASS: &str =
        "lg:w-6/12 flex items-center lg:rounded-r-lg rounded-b-lg lg:rounded-bl-none bg-green-100";

    view! {
      <div class = move || if success.get() { SUCCESS_CLASS } else { DEFAULT_CLASS } >
        <div class="px-4 py-6 md:p-12 md:mx-6">
          <Show
            when = move || success.get()
            fallback = move ||
              view! {
                <h4 class="text-xl font-semibold mb-6">
                  { info_title }
                </h4>
                <p class="text-sm">
                  { info_description }
                </p>
              }
          >
            <h4 class="text-xl font-semibold mb-6">
              { success_title }
            </h4>
            <p class="text-sm">
              { success_description }
            </p>
          </Show>
        </div>
      </div>
    }
}

use leptos::*;
use leptos_fluent::*;
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

    let register_action = Action::new(move |(email, password): &(String, String)| {
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
                          title = move_tr!("sign-up")
                          description = move_tr!("enter-login-information")
                          action_label= move_tr!("sign-up")
                          initial_credentials = Credentials::default()
                          action=register_action
                          error = register_error.into()
                          disabled = { disabled }
                      />
                      <div class="flex items-center justify-between pb-6">
                        <p class="mb-0 mr-2 text-gray-600">
                          { move_tr!("already-have-an-account") }
                        </p>
                        <A
                          href=Page::Login.path()
                          class="inline-block px-6 py-2 border-2 font-medium text-xs leading-tight uppercase rounded hover:bg-opacity-25 focus:outline-none focus:ring-0 transition duration-150 ease-in-out"
                        >
                          { move_tr!("log-in") }
                        </A>
                      </div>
                    </div>
                  </div>
                  <InfoBox
                    success
                    info_title = move_tr!("user-account-offer-question")
                    info_description = move_tr!("user-account-benefits")
                    success_title = move_tr!("successfully-signed-up")
                    // TODO: pass children like <p>
                    success_description = move_tr!("sign-up-success-message")
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
    info_title: Signal<String>,
    info_description: Signal<String>,
    success_title: Signal<String>,
    success_description: Signal<String>,
) -> impl IntoView {
    view! {
      <div
        class = "lg:w-6/12 flex items-center lg:rounded-r-lg rounded-b-lg lg:rounded-bl-none"
        class = ("bg-gray-100", move || !success.get())
        class = ("bg-green-100", move || success.get())
      >
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

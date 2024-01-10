use leptos::*;
use leptos_router::*;

use crate::api::UnauthorizedApi;

type ErrorMessage = String;
type ConfirmationResult = std::result::Result<(), ErrorMessage>;
type ConfirmationState = Option<ConfirmationResult>;

#[derive(Params, PartialEq, Debug, Clone)]
struct ConfirmEmailAddressParams {
    token: Option<String>,
}

#[component]
pub fn ConfirmEmailAddress(api: UnauthorizedApi) -> impl IntoView {
    let confirmation_result = RwSignal::<ConfirmationState>::new(Option::None);
    let params = use_params::<ConfirmEmailAddressParams>();

    let confirm_action = create_action(move |token: &String| {
        let token = token.clone();
        async move {
            let result = api
                .confirm_email_address(token)
                .await
                .map_err(|err| err.to_string());
            confirmation_result.set(Some(result));
        }
    });

    create_effect(move |_| {
        let ConfirmEmailAddressParams { token } = match params.get() {
            Ok(params) => params,
            Err(err) => {
                confirmation_result.set(Some(Err(err.to_string())));
                return;
            }
        };
        let Some(token) = token else {
            confirmation_result.set(Some(Err("missing token".to_string())));
            return;
        };
        confirm_action.dispatch(token);
    });

    move || match confirmation_result.get() {
        None => view! { <WaitMessage /> }.into_view(),
        Some(Ok(_)) => view! { <SuccessMessage /> }.into_view(),
        Some(Err(err)) => {
            log::warn!("{err}");
            view! { <ErrorMessage /> }.into_view()
        }
    }
}

#[component]
fn WaitMessage() -> impl IntoView {
    view! {
      <div class="rounded-md bg-blue-50 p-4">
        <div class="flex">
          <div class="flex-shrink-0">
            <svg class="h-5 w-5 text-blue-400" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
              <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a.75.75 0 000 1.5h.253a.25.25 0 01.244.304l-.459 2.066A1.75 1.75 0 0010.747 15H11a.75.75 0 000-1.5h-.253a.25.25 0 01-.244-.304l.459-2.066A1.75 1.75 0 009.253 9H9z" clip-rule="evenodd" />
            </svg>
          </div>
          <div class="ml-3 flex-1 md:flex md:justify-between">
            <p class="text-sm text-blue-700">"Einen moment bitte..."</p>
          </div>
        </div>
      </div>
    }
}

#[component]
fn SuccessMessage() -> impl IntoView {
    view! {
      <div class="rounded-md bg-green-50 p-4">
        <div class="flex">
          <div class="flex-shrink-0">
            <svg class="h-5 w-5 text-green-400" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
              <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z" clip-rule="evenodd" />
            </svg>
          </div>
          <div class="ml-3">
            <p class="text-sm font-medium text-green-800">"Ihre E-Mail-Adresse ist nun bestätigt."</p>
          </div>
          <div class="ml-auto pl-3">
            <div class="-mx-1.5 -my-1.5">
              <button type="button" class="inline-flex rounded-md bg-green-50 p-1.5 text-green-500 hover:bg-green-100 focus:outline-none focus:ring-2 focus:ring-green-600 focus:ring-offset-2 focus:ring-offset-green-50">
                <span class="sr-only">Dismiss</span>
                <svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                  <path d="M6.28 5.22a.75.75 0 00-1.06 1.06L8.94 10l-3.72 3.72a.75.75 0 101.06 1.06L10 11.06l3.72 3.72a.75.75 0 101.06-1.06L11.06 10l3.72-3.72a.75.75 0 00-1.06-1.06L10 8.94 6.28 5.22z" />
                </svg>
              </button>
            </div>
          </div>
        </div>
      </div>
    }
}

#[component]
fn ErrorMessage() -> impl IntoView {
    view! {
      <div class="rounded-md bg-red-50 p-4">
        <div class="flex">
          <div class="flex-shrink-0">
            <svg class="h-5 w-5 text-red-400" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
              <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.28 7.22a.75.75 0 00-1.06 1.06L8.94 10l-1.72 1.72a.75.75 0 101.06 1.06L10 11.06l1.72 1.72a.75.75 0 101.06-1.06L11.06 10l1.72-1.72a.75.75 0 00-1.06-1.06L10 8.94 8.28 7.22z" clip-rule="evenodd" />
            </svg>
          </div>
          <div class="ml-3">
            <p class="text-sm font-medium text-red-800">"Es tut uns leid, aber das hat nicht geklappt."</p>
          </div>
        </div>
      </div>
    }
}

use leptos::*;
use leptos_router::*;

use crate::{api::UnauthorizedApi, message::*};

type ErrorMessage = String;
type ConfirmationResult = std::result::Result<(), ErrorMessage>;
type ConfirmationState = Option<ConfirmationResult>;

#[derive(Params, PartialEq, Debug, Clone)]
struct Query {
    token: Option<String>,
}

#[component]
pub fn ConfirmEmailAddress(api: UnauthorizedApi) -> impl IntoView {
    let confirmation_result = RwSignal::<ConfirmationState>::new(Option::None);
    let query = use_query::<Query>();

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
        let Query { token } = match query.get() {
            Ok(q) => q,
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

    view! {
      <div class="mx-auto max-w-7xl sm:px-6 lg:px-8">
        <div class="py-8 px-4 sm:px-0">
          {
            move || match confirmation_result.get() {
                None => view! {
                    <InfoMessage message = "Einen moment bitte..." />
                }.into_view(),
                Some(Ok(_)) => view! {
                   <SuccessMessage message = "Ihre E-Mail-Adresse ist nun bestätigt." />
                }.into_view(),
                Some(Err(err)) => {
                    log::warn!("{err}");
                    view! {
                      <ErrorMessage message = "Es tut uns leid, aber das hat nicht geklappt." />
                    }.into_view()
                }
            }
          }
        </div>
      </div>
    }
}

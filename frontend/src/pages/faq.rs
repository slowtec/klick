use klick_app_components::icons;
use klick_app_components::links::FAQ_DE;
use leptos::*;

#[component]
pub fn Faq() -> impl IntoView {
    view! {
      <div class="bg-white">
        <div class="mx-auto max-w-7xl px-6 py-24 sm:pt-32 lg:px-8 lg:py-40">
          <div class="lg:grid lg:grid-cols-12 lg:gap-8">
            <div class="lg:col-span-5">
              <h2 class="text-2xl font-bold leading-10 tracking-tight text-gray-900">Frequently asked questions</h2>
              <p class="mt-4 text-base leading-7 text-gray-600">
                "Sie können die gesuchte Antwort nicht finden?"
              </p>
              <p class="mt-4 text-base leading-7 text-gray-600">
                <a href={FAQ_DE} class="font-semibold text-indigo-600 hover:text-indigo-500" style="text-decoration: underline;
                text-decoration-color: currentcolor; -webkit-text-decoration-color: #ffed00; text-decoration-color: #ffed00; text-underline-offset: 4px;">
                <icons::ExternalLink /> "Schauen sie in unser Wiki im Abschnitt FAQ"
                </a>
              </p>
            </div>
          </div>
        </div>
      </div>
    }
}

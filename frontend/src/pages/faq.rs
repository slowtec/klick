use leptos::{component, view, IntoView};

#[component]
pub fn Faq() -> impl IntoView {
    view! {
      <div class="bg-white">
        <div class="mx-auto max-w-7xl px-6 py-24 sm:pt-32 lg:px-8 lg:py-40">
          <div class="lg:grid lg:grid-cols-12 lg:gap-8">
            <div class="lg:col-span-5">
              <h2 class="text-2xl font-bold leading-10 tracking-tight text-gray-900">Frequently asked questions</h2>
              <p class="mt-4 text-base leading-7 text-gray-600">
                "Sie können die gesuchte Antwort nicht finden? Wenden Sie sich an unser "
                <a href="#" class="font-semibold text-indigo-600 hover:text-indigo-500">"Kundensupport"</a>"-Team."
              </p>
            </div>
            <div class="mt-10 lg:col-span-7 lg:mt-0">
              <dl class="space-y-10">
                <QnA
                  q = "Wie funktioniert es?"
                  a = "Erläuterung."
                />
                <QnA
                  q = "Was macht die Kläranlage?"
                  a = "Klären."
                />
                <QnA
                  q = "Emissionsfaktor?"
                  a = "Der ist hoch."
                />
              </dl>
            </div>
          </div>
        </div>
      </div>
    }
}

#[component]
pub fn QnA(q: &'static str, a: &'static str) -> impl IntoView {
    view! {
      <div>
        <dt class="text-base font-semibold leading-7 text-gray-900">{ q }</dt>
        <dd class="mt-2 text-base leading-7 text-gray-600">{ a }</dd>
      </div>
    }
}

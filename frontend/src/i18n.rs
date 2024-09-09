use leptos::*;
use leptos_fluent::*;
use leptos_hotkeys::use_hotkeys;

#[component]
pub fn LanguageSelector() -> impl IntoView {
    let i18n = expect_i18n();
    let current_lang = i18n.language;
    let change_language = RwSignal::new(false);

    Effect::new(move |_| {
        _ = current_lang.get();
        change_language.set(false);
    });

    use_hotkeys!(("F2") => move |()| {
      let current_lang = current_lang.get();
      let current_index = i18n.languages.iter().position(|&r| r == current_lang).unwrap_or(0);
      let next_index = (current_index + 1) % i18n.languages.len();
      let next_lang = &i18n.languages[next_index];
      next_lang.activate();
    });

    view! {
        <div class="px-2 relative inline-block text-left">
          <div>
            <button
              type="button"
              id="language-selector-button"
              class="inline-flex items-center justify-center w-full rounded-md border border-gray-300 shadow-sm px-4 py-2 bg-white text-sm font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-teal-500"
              aria-haspopup="true"
              on:click = move |_| { change_language.update(|x|*x = !*x); }
            >
            { move || current_lang.get().name }
            <svg
              class="-mr-1 ml-2 h-5 w-5"
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 20 20"
              fill="currentColor"
              aria-hidden="true"
            >
              <path
                fill-rule="evenodd"
                d="M10.293 14.707a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L10 12.586l3.293-3.293a1 1 0 011.414 1.414l-4 4z"
                clip-rule="evenodd"
              />
            </svg>
          </button>
        </div>
        <Show when = move || change_language.get()>
          <LanguageOptions />
        </Show>
      </div>
    }
}

#[component]
fn LanguageOptions() -> impl IntoView {
    let i18n = expect_i18n();

    view! {
      <div
        id="language-selector-options"
        class="origin-top-right absolute right-0 mt-2 w-48 md:w-96 rounded-md shadow-lg bg-white ring-1 ring-black ring-opacity-5"
        role="menu"
        aria-orientation="vertical"
        aria-labelledby="language-selector"
      >
        <div class="p-1 grid md:grid-cols-2 grid-cols-1 gap-2" role="none">
          {
              move || i18n.languages.iter().enumerate().map(|(i,lang)|{
                  let select_style = if *lang == i18n.language.get() {
                      "bg-gray-100 text-gray-900"
                  } else {
                      "text-gray-700"
                  };

                  let rounded_style =  if i %2 == 0 {
                    "rounded-r"
                  } else {
                    "rounded-l"
                  };

                  let flag_icon = match lang.id.language.as_str() {
                      "en" => "gb",
                       id => id,
                  };

                  view! {

                    <a
                      class=format!("{select_style} cursor-pointer block px-4 py-2 text-sm text-left items-center inline-flex hover:bg-gray-100 {rounded_style}")
                      role="menuitem" href=""
                      on:click = move |_| { lang.activate(); }
                    >
                      <span class=format!("fi fis fi-circle inline-block mr-2 fi-{flag_icon}")></span>
                      <span class="truncate">{ lang.name }</span>
                    </a>
                  }
              }).collect::<Vec<_>>()
          }
        </div>
      </div>
    }
}

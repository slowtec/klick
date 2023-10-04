use leptos::*;

use crate::Page;

const CLASS_CURRENT : &str = "border-highlight text-gray-900 inline-flex items-center border-b-2 px-1 pt-1 text-sm font-medium";
const CLASS_INACTIVE : &str = "border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 inline-flex items-center border-b-2 px-1 pt-1 text-sm font-medium";

#[component]
pub fn Nav(current_page: Signal<Page>) -> impl IntoView {
    view! {
      <nav class="border-b border-gray-200 bg-white">
        <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
          <div class="flex h-16 justify-between">
            <div class="flex">
              <div class="flex flex-shrink-0 items-center">
                <img class="h-8 w-auto" src="logo-utbw-solo.svg" alt="Umwelt Technik BW" />
              </div>
              <div class="sm:ml-6 sm:flex sm:space-x-8">
                <a
                  href=Page::Home.path()
                  // TODO: aria-current
                  class= move ||{
                    if current_page.get() == Page::Home {
                       CLASS_CURRENT
                    } else {
                       CLASS_INACTIVE
                    }
                  }>
                  "Tool"
                </a>
                <a
                  href=Page::Faq.path()
                  // TODO: aria-current
                  class= move ||{
                    if current_page.get() == Page::Faq {
                       CLASS_CURRENT
                    } else {
                       CLASS_INACTIVE
                    }
                  }>
                  "FAQ"
                </a>
              </div>
            </div>
          </div>
        </div>
      </nav>
    }
}

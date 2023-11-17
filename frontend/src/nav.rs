use leptos::{component, tracing, view, IntoView, Signal, SignalGet};

use crate::Page;

#[component]
pub fn Nav(current_page: Signal<Page>) -> impl IntoView {
    view! {
      <nav class="border-b border-gray-200 bg-white">
        <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
          <div class="flex h-16 justify-between">
            <div class="flex">
              <div class="flex flex-shrink-0 items-center">
                <img class="h-16 w-auto" src="logo-utbw-solo.svg" alt="Umwelt Technik BW" />
              </div>
              <div class="sm:ml-6 sm:flex sm:space-x-8">
                <NavLink page=Page::Home current_page label="KlicK" />
                <NavLink page=Page::Tool current_page label="Tool" />
                <NavLink page=Page::Faq current_page label="FAQs" />
              </div>
            </div>
          </div>
        </div>
      </nav>
    }
}

const CLASS_CURRENT : &str = "border-highlight text-gray-900 inline-flex items-center border-b-2 px-1 pt-1 text-sm font-medium";
const CLASS_INACTIVE : &str = "border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 inline-flex items-center border-b-2 px-1 pt-1 text-sm font-medium";

#[component]
fn NavLink(page: Page, current_page: Signal<Page>, label: &'static str) -> impl IntoView {
    view! {
      <a
        href= page.path()
        // TODO: aria-current
        class= move ||{
          if current_page.get() == page {
             CLASS_CURRENT
          } else {
             CLASS_INACTIVE
          }
        }>
        { label }
      </a>
    }
}

use leptos::*;
use leptos_router::*;

use klick_boundary::json_api::UserInfo;

use crate::Page;

#[component]
pub fn Nav(
    current_page: Signal<Page>,
    user_info: Signal<Option<UserInfo>>,
    #[prop(into)] on_logout: Callback<()>,
) -> impl IntoView {
    let mobile_menu_is_open = RwSignal::new(false);

    view! {
      <nav class="border-b border-gray-200 bg-white">
        <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
          <div class="flex h-16 justify-between">

            <div class="flex">
              <div class="flex flex-shrink-0 items-center">
                <img class="h-16 w-auto" src="logo-utbw-solo.svg" alt="Umwelt Technik BW" />
              </div>
              <div class="hidden sm:ml-6 sm:flex sm:space-x-8">
                <NavLink page=Page::Home current_page label="KlicK" />
                <NavLink page=Page::Tool current_page label="Tool" />
                <NavLink page=Page::Projects current_page label="Projekte" />
                <NavLink page=Page::Faq  current_page label="FAQs" />
              </div>
            </div>

            <div class="hidden sm:ml-6 sm:flex sm:items-center">
            { move || if let Some(user_info) = user_info.get() {
                view! {
                  <UserMenu user_info on_logout />
                }.into_view()
              } else {
                view! {
                  <div>
                    <a
                      class = "text-gray-500 hover:text-gray-700 mx-2 px-1 pt-1 text-sm font-medium"
                      href = Page::Login.path()
                    >
                      "Login"
                    </a>
                    <a
                      class = "text-gray-500 hover:text-gray-700 mx-2 px-1 pt-1 text-sm font-medium"
                      href = Page::Register.path()
                    >
                      "Registrieren"
                    </a>
                  </div>
                }.into_view()
              }
            }
            </div>

            <div class="-mr-2 flex items-center sm:hidden">
                // Mobile menu button
              <button
                type="button"
                class="relative inline-flex items-center justify-center rounded-md p-2 text-gray-400 hover:bg-gray-100 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-indigo-500"
                aria-controls="mobile-menu"
                aria-expanded="false"
                on:click = move |_| mobile_menu_is_open.update(|x|*x = !*x)
              >
                <span class="absolute -inset-0.5"></span>
                <span class="sr-only">"Open main menu"</span>
                // Icon when menu is open.
                <svg
                  class = move || if mobile_menu_is_open.get() { "block h-6 w-6" } else { "hidden" }
                  class="hidden h-6 w-6"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke-width="1.5"
                  stroke="currentColor"
                  aria-hidden="true"
                >
                  <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                </svg>
                // Icon when menu is closed.
                <svg
                  class = move || if mobile_menu_is_open.get() { "hidden" } else { "block h-6 w-6" }
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke-width="1.5"
                  stroke="currentColor"
                  aria-hidden="true"
                >
                  <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
                </svg>
              </button>
            </div>
          </div>
        </div>

        // Mobile menu
        <div class= move || if mobile_menu_is_open.get() { "sm:hidden" } else { "hidden" } >
          <div class="space-y-1 pb-3 pt-2">
            <MobileNavLink page=Page::Home current_page label="KlicK" />
            <MobileNavLink page=Page::Tool current_page label="Tool" />
            <MobileNavLink page=Page::Projects current_page label="Projekte" />
            <MobileNavLink page=Page::Faq  current_page label="FAQs" />
          </div>
          { move ||
            if let Some(user_info) = user_info.get() {
              view! { <MobileUserMenu user_info on_logout /> }.into_view()
            } else {
              view! { <PublicMobileUserMenu /> }.into_view()
            }
          }
        </div>
      </nav>
    }
}

#[component]
fn UserMenu(user_info: UserInfo, #[prop(into)] on_logout: Callback<()>) -> impl IntoView {
    let user_menu_is_active = RwSignal::new(false);
    view! {
       // Profile dropdown
       <div class="relative ml-3">
         <div>
           <button
             type="button"
             class="relative flex items-center rounded-full bg-white text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
             aria-expanded="false"
             aria-haspopup="true"
             on:click = move |_| user_menu_is_active.update(|x|*x = !*x)
           >
             <span class="absolute -inset-1.5"></span>
             <span class="sr-only">"Open user menu"</span>

             <svg viewBox="0 0 24 24" fill="currentColor" class="w-8 h-8">
               <path fill-rule="evenodd" d="M18.685 19.097A9.723 9.723 0 0 0 21.75 12c0-5.385-4.365-9.75-9.75-9.75S2.25 6.615 2.25 12a9.723 9.723 0 0 0 3.065 7.097A9.716 9.716 0 0 0 12 21.75a9.716 9.716 0 0 0 6.685-2.653Zm-12.54-1.285A7.486 7.486 0 0 1 12 15a7.486 7.486 0 0 1 5.855 2.812A8.224 8.224 0 0 1 12 20.25a8.224 8.224 0 0 1-5.855-2.438ZM15.75 9a3.75 3.75 0 1 1-7.5 0 3.75 3.75 0 0 1 7.5 0Z" clip-rule="evenodd" />
             </svg>
             <div class="ml-2">
               { user_info.email }
             </div>
           </button>
         </div>

         // TODO:
         // Entering: "transition ease-out duration-200"
         //   From: "transform opacity-0 scale-95"
         //   To: "transform opacity-100 scale-100"
         // Leaving: "transition ease-in duration-75"
         //   From: "transform opacity-100 scale-100"
         //   To: "transform opacity-0 scale-95"
         <Show
           when = move || user_menu_is_active.get()
         >
           <div
             class="absolute right-0 z-10 mt-2 w-48 origin-top-right rounded-md bg-white py-1 shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none"
             role="menu"
             aria-orientation="vertical"
             aria-labelledby="user-menu-button"
             tabindex="-1"
           >
             <a
               href="#"
               class="block px-4 py-2 text-sm text-gray-700"
               role="menuitem"
               tabindex="-1"
               id="user-menu-item-2"
               on:click = move |_| on_logout.call(())
             >
               "Logout"
             </a>
           </div>
         </Show>
       </div>
    }
}

#[component]
fn MobileUserMenu(user_info: UserInfo, #[prop(into)] on_logout: Callback<()>) -> impl IntoView {
    view! {
      <div class="border-t border-gray-200 pb-3 pt-4">
        <div class="flex items-center px-4">
          <div class="flex-shrink-0">
            <svg viewBox="0 0 24 24" fill="currentColor" class="w-10 h-10">
              <path
                fill-rule="evenodd"
                d="M18.685 19.097A9.723 9.723 0 0 0 21.75 12c0-5.385-4.365-9.75-9.75-9.75S2.25 6.615 2.25 12a9.723 9.723 0 0 0 3.065 7.097A9.716 9.716 0 0 0 12 21.75a9.716 9.716 0 0 0 6.685-2.653Zm-12.54-1.285A7.486 7.486 0 0 1 12 15a7.486 7.486 0 0 1 5.855 2.812A8.224 8.224 0 0 1 12 20.25a8.224 8.224 0 0 1-5.855-2.438ZM15.75 9a3.75 3.75 0 1 1-7.5 0 3.75 3.75 0 0 1 7.5 0Z"
                clip-rule="evenodd" />
            </svg>
          </div>
          <div class="ml-3">
            <div class="text-sm font-medium text-gray-500">
              { user_info.email }
            </div>
          </div>
        </div>
        <div class="mt-3 space-y-1">
          <a
            href="#"
            class="block px-4 py-2 text-base font-medium text-gray-500 hover:bg-gray-100 hover:text-gray-800"
            on:click = move |_| on_logout.call(())
          >
            "Logout"
          </a>
          <A
            href=Page::Projects.path()
            class="block px-4 py-2 text-base font-medium text-gray-500 hover:bg-gray-100 hover:text-gray-800"
          >
            "Projekte"
          </A>
        </div>
      </div>
    }
}

#[component]
fn PublicMobileUserMenu() -> impl IntoView {
    view! {
      <div class="border-t border-gray-200 pb-3 pt-1">
        <div class="mt-3 space-y-1">
          <a href= Page::Login.path() class="block px-4 py-2 text-base font-medium text-gray-500 hover:bg-gray-100 hover:text-gray-800">"Login"</a>
          <a href= Page::Register.path() class="block px-4 py-2 text-base font-medium text-gray-500 hover:bg-gray-100 hover:text-gray-800">"Registrieren"</a>
        </div>
      </div>
    }
}

#[component]
fn NavLink(page: Page, current_page: Signal<Page>, label: &'static str) -> impl IntoView {
    const CLASS_CURRENT : &str = "border-highlight text-gray-900 inline-flex items-center border-b-2 px-1 pt-1 text-sm font-medium";
    const CLASS_INACTIVE : &str = "border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 inline-flex items-center border-b-2 px-1 pt-1 text-sm font-medium";

    view! {
      <a
        href= page.path()
        // TODO: aria-current
        class= move ||{ if current_page.get() == page { CLASS_CURRENT } else { CLASS_INACTIVE } }
      >
        { label }
      </a>
    }
}

#[component]
fn MobileNavLink(page: Page, current_page: Signal<Page>, label: &'static str) -> impl IntoView {
    const CLASS_CURRENT: &str = "block border-l-4 border-highlight bg-indigo-50 py-2 pl-3 pr-4 text-base font-medium text-indigo-700";
    const CLASS_INACTIVE: &str = "block border-l-4 border-transparent py-2 pl-3 pr-4 text-base font-medium text-gray-500 hover:border-gray-300 hover:bg-gray-50 hover:text-gray-700";

    view! {
      <a
        href= page.path()
        // TODO: aria-current
        class= move ||{ if current_page.get() == page { CLASS_CURRENT } else { CLASS_INACTIVE } }
      >
        { label }
      </a>
    }
}

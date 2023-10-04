use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod forms;
mod pages;

use self::pages::{Documentation, Home};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let (current_page, set_current_page) = create_signal(Page::Home);

    view! {
      <Nav current_page = current_page.into() />
      <div class="py-10">
        <Router>
          <Routes>
            <Route
              path="/"
              view= move ||{
                set_current_page.update(|p|*p = Page::Home);
                view! {
                  <Main title = "Sankey-Tool">
                    <Home />
                  </Main>
                }
              }
            />
            <Route
              path="/doc"
              view= move || {
                set_current_page.update(|p|*p = Page::Docs);
                view! {
                  <Main title = "Dokumentation">
                    <Documentation />
                  </Main>
                }
              }
            />
          </Routes>
        </Router>
      </div>
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Page {
    Home,
    Docs,
}

#[component]
fn Main(title: &'static str, children: Children) -> impl IntoView {
    view! {
      <Header title />
      <main>
        <div class="mx-auto max-w-7xl sm:px-6 lg:px-8">
          <div class="px-4 py-8 sm:px-0">
            { children() }
          </div>
        </div>
      </main>
    }
}

#[component]
fn Header(title: &'static str) -> impl IntoView {
    view! {
      <header>
        <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
          <h1 class="text-3xl font-bold leading-tight tracking-tight text-gray-900">{ title }</h1>
        </div>
      </header>
    }
}

const CLASS_CURRENT : &str = "border-highlight text-gray-900 inline-flex items-center border-b-2 px-1 pt-1 text-sm font-medium";
const CLASS_INACTIVE : &str = "border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 inline-flex items-center border-b-2 px-1 pt-1 text-sm font-medium";

#[component]
fn Nav(current_page: Signal<Page>) -> impl IntoView {
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
                  href="/"
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
                  href="/doc"
                  // TODO: aria-current
                  class= move ||{
                    if current_page.get() == Page::Docs {
                       CLASS_CURRENT
                    } else {
                       CLASS_INACTIVE
                    }
                  }>
                  "Dokumentation"
                </a>
              </div>
            </div>
          </div>
        </div>
      </nav>
    }
}

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod footer;
mod forms;
mod imprint;
mod nav;
mod pages;
mod sankey;

use self::{
    footer::Footer,
    imprint::Imprint,
    nav::Nav,
    pages::{Documentation, Home, Page},
};

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
              path=Page::Faq.path()
              view= move || {
                set_current_page.update(|p|*p = Page::Faq);
                view! {
                  <Main title = "FAQ">
                    <Documentation />
                  </Main>
                }
              }
            />
            <Route
              path=Page::Imprint.path()
              view= move || {
                set_current_page.update(|p|*p = Page::Imprint);
                view! {
                  <Main title = "Impressum">
                    <Imprint />
                  </Main>
                }
              }
            />
          </Routes>
        </Router>
        <Footer />
      </div>
    }
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

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod pages;

use self::pages::{Documentation, Home};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
      <Link
        rel="stylesheet"
        href="https://cdn.jsdelivr.net/npm/semantic-ui@2.5.0/dist/semantic.min.css"
      />
      <Script
        src="https://code.jquery.com/jquery-3.1.1.min.js"
        integrity="sha256-hVVnYaiADRTO2PzUGmuLJr8BLUSjGIZsDYGmIJLv2b8="
        crossorigin="anonymous"
      />
      <Script src="https://cdn.jsdelivr.net/npm/semantic-ui@2.5.0/dist/semantic.min.js" />

      <div class="ui container">
        <h2>Sankey-Tool</h2>
        <Router>
          <Routes>
            <Route
              path="/"
              view=|| {
                view! {
                  <Home />
                  <h2>Dokumentation</h2>
                  <Documentation />
                }
              }
            />
          </Routes>
        </Router>
      </div>
    }
}

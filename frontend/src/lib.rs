use gloo_storage::{LocalStorage, Storage};
use leptos::*;
use leptos_hotkeys::{provide_hotkeys_context, scopes, use_hotkeys};
use leptos_meta::provide_meta_context;
use leptos_router::{use_navigate, NavigateOptions, Route, Router, Routes};

use klick_app_api as api;
use klick_boundary::{self as boundary, json_api::UserInfo};

mod credentials;
mod footer;
mod forms;
mod nav;
mod pages;
mod sankey;

use self::{
    footer::Footer,
    nav::Nav,
    pages::{
        ConfirmEmailAddress, Faq, Login, Page, PageSection, Projects, Register, ResetPassword,
        ResetPasswordRequest, Tool,
    },
};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const IMPRINT_MD: &str = include_str!("../content/imprint.md");
const ABOUT_MD: &str = include_str!("../content/about.md");
const OPEN_SOURCE_MD: &str = include_str!("../content/open-source.md");
const CHANGELOG_URL: &str = concat!(
    "https://codeberg.org/slowtec/klick/src/tag/v",
    env!("CARGO_PKG_VERSION"),
    "/CHANGELOG.md"
);

const DEFAULT_API_URL: &str = "/api";
const API_TOKEN_STORAGE_KEY: &str = "api-token";

const SECTION_ID_TOOL_HOME: &str = "tool-home";

#[allow(clippy::too_many_lines)] // TODO
#[component]
#[must_use]
pub fn App() -> impl IntoView {
    provide_meta_context();

    // -- hotkeys -- //

    let app_ref = create_node_ref::<html::Div>();
    provide_hotkeys_context(app_ref, false, scopes!());
    let accessibility_always_show: Option<RwSignal<bool>> = Some(RwSignal::new(false));

    use_hotkeys!(("F1") => move |()| {
      match accessibility_always_show {
        Some(o) => {
          o.set(!o.get());},
        None => {}
      }
    });

    // -- signals -- //

    let (current_page, set_current_page) = RwSignal::new(Page::Home).split();
    let authorized_api = RwSignal::new(None::<api::AuthorizedApi>);
    let user_info = RwSignal::new(None::<UserInfo>);
    let logged_in = Signal::derive(move || user_info.get().is_some());
    let current_project = RwSignal::new(None::<boundary::Project>);

    // -- actions -- //

    let fetch_user_info = Action::new(move |api: &api::AuthorizedApi| {
        let api = api.clone();
        async move {
            match api.user_info().await {
                Ok(info) => {
                    user_info.update(|i| *i = Some(info));
                }
                Err(err) => {
                    log::error!("Unable to fetch user info: {err}");
                    user_info.set(None);
                    authorized_api.set(None);
                }
            }
        }
    });

    let logout = Action::new(move |(): &()| async move {
        match authorized_api.get() {
            Some(api) => match api.logout().await {
                Ok(()) => {
                    authorized_api.update(|a| *a = None);
                    user_info.update(|i| *i = None);
                }
                Err(err) => {
                    log::error!("Unable to logout: {err}");
                }
            },
            None => {
                log::error!("Unable to logout user: not logged in");
            }
        }
    });

    // -- callbacks -- //

    let on_logout = move |()| {
        logout.dispatch(());
    };

    // -- init API -- //

    let unauthorized_api = api::UnauthorizedApi::new(DEFAULT_API_URL);
    if let Ok(token) = LocalStorage::get(API_TOKEN_STORAGE_KEY) {
        let api = api::AuthorizedApi::new(DEFAULT_API_URL, token);
        fetch_user_info.dispatch(api.clone());
        authorized_api.update(|a| *a = Some(api));
    }

    log::debug!("User is logged in: {}", logged_in.get_untracked());

    // -- effects -- //

    Effect::new(move |_| {
        if let Some(api) = authorized_api.get() {
            log::debug!("API is now authorized: save token in LocalStorage");
            LocalStorage::set(API_TOKEN_STORAGE_KEY, api.token()).expect("LocalStorage::set");
        } else {
            log::debug!("API is no longer authorized: delete token from LocalStorage");
            LocalStorage::delete(API_TOKEN_STORAGE_KEY);
        }
    });

    view! {
      <div _ref=app_ref>
        <Nav
          current_page = current_page.into()
          user_info = user_info.into()
          on_logout
        />
        <Router>
          <Routes>
            <Route
              path=Page::Home.path()
              view= move ||{
                set_current_page.update(|p|*p = Page::Home);
                view! {
                  <Main>
                    <Markdown content = ABOUT_MD />
                  </Main>
                }
              }
            />
            <Route
              path=Page::Tool.path()
              view= move ||{
                set_current_page.update(|p|*p = Page::Tool);
                let current_section = RwSignal::new(PageSection::DataCollection);
                view! {
                  <Main>
                    <header
                      class="prose"
                      id = move || current_section.get().section_id()
                    >
                      <h1 class="mb-8">
                        "KlicK-Tool "
                        <span class="font-light text-xl text-gray-600">
                          <a
                            class="font-light text-xl no-underline hover:underline"
                            href= { CHANGELOG_URL } >
                            "(v" { VERSION } ")"
                          </a>
                        </span>
                      </h1>
                      <p id="keyboard-hint" class="sr-only">
                        "Press F1 to display all hints inline."
                      </p>
                    </header>
                    <Tool
                      api = authorized_api.into()
                      current_project
                      current_section
                      accessibility_always_show
                    />
                  </Main>
                }
              }
            />
            <Route
              path=Page::Faq.path()
              view= move || {
                set_current_page.update(|p|*p = Page::Faq);
                view! {
                  <Main>
                    <header class="prose">
                      <h1>"FAQs"</h1>
                    </header>
                    <Faq />
                  </Main>
                }
              }
            />
            <Route
              path=Page::OpenSource.path()
              view= move || {
                set_current_page.update(|p|*p = Page::OpenSource);
                view! {
                  <Main>
                    <Markdown content = OPEN_SOURCE_MD />
                  </Main>
                }
              }
            />
            <Route
              path=Page::Imprint.path()
              view= move || {
                set_current_page.update(|p|*p = Page::Imprint);
                view! {
                  <Main>
                    <Markdown content = IMPRINT_MD />
                  </Main>
                }
              }
            />
            <Route
                path=Page::Login.path()
                view=move || {
                    view! {
                        <Login
                            api=unauthorized_api
                            on_success=move |api: api::AuthorizedApi| {
                                log::info!("Successfully logged in");
                                authorized_api.update(|v| *v = Some(api.clone()));
                                let navigate = use_navigate();
                                navigate(Page::Home.path(), NavigateOptions::default());
                                fetch_user_info.dispatch(api);
                            }
                        />
                    }
                }
            />
            <Route
                path=Page::Register.path()
                view=move || {
                    view! { <Register api=unauthorized_api /> }
                }
            />
            <Route
                path=Page::ResetPasswordRequest.path()
                view=move || {
                    view! { <ResetPasswordRequest api=unauthorized_api /> }
                }
            />
            <Route
                path=Page::ResetPassword.path()
                view=move || {
                    view! { <ResetPassword api=unauthorized_api /> }
                }
            />
            <Route
                path=Page::ConfirmEmailAddress.path()
                view= move || {
                    view! { <ConfirmEmailAddress api=unauthorized_api /> }
                }
            />
            <Route
                path=Page::Projects.path()
                view = move || {
                  set_current_page.update(|p|*p = Page::Projects);
                  view! {
                    <Main>
                      <header class="prose">
                        <h1 class="mb-8">"Projekte"</h1>
                      </header>
                      <Projects
                        api = authorized_api.into()
                        current_project
                      />
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
fn Main(children: Children) -> impl IntoView {
    view! {
      <div class="py-10">
        <main>
          <div class="mx-auto max-w-7xl sm:px-6 lg:px-8">
            <div class="px-4 py-8 sm:px-0">
              { children() }
            </div>
          </div>
        </main>
      </div>
    }
}

#[component]
fn Markdown(content: &'static str) -> impl IntoView {
    use pulldown_cmark::{html, Options, Parser};

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(content, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    view! {
      <div class="prose" inner_html = html_output></div>
    }
}

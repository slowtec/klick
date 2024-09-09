use fluent_templates::static_loader;
use gloo_net::http::Request;
use gloo_storage::{LocalStorage, Storage};
use leptos::*;
use leptos_fluent::*;
use leptos_hotkeys::{provide_hotkeys_context, scopes, use_hotkeys};
use leptos_meta::provide_meta_context;
use leptos_router::{use_navigate, NavigateOptions, Route, Router, Routes};

use klick_app_api as api;
use klick_boundary::{self as boundary, json_api::UserInfo};
use klick_presenter::{Lng, ValueLabel};

static_loader! {
    static TRANSLATIONS = {
        locales: "./locales",
        fallback_language: "de",
    };
}

mod credentials;
mod footer;
mod forms;
mod i18n;
mod nav;
mod pages;
mod sankey;

use self::{
    footer::Footer,
    nav::Nav,
    pages::{
        ConfirmEmailAddress, Login, Page, PageSection, Projects, Register, ResetPassword,
        ResetPasswordRequest, Tool,
    },
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

const CHANGELOG_URL: &str = concat!(
    "https://codeberg.org/slowtec/klick/src/tag/v",
    env!("CARGO_PKG_VERSION"),
    "/CHANGELOG.md"
);

const DEFAULT_API_URL: &str = "/api";
const API_TOKEN_STORAGE_KEY: &str = "api-token";

const SECTION_ID_TOOL_HOME: &str = "tool-home";

#[must_use]
pub fn current_lang() -> Signal<Lng> {
    // TODO: avoid mixing lang and lng
    let lang = leptos_fluent::i18n().language;
    Signal::derive(move || Lng::try_from_id(&lang.get().id).unwrap())
}

pub fn label_signal<ID>(id: ID) -> Signal<String>
where
    ID: ValueLabel + 'static,
{
    let lang = leptos_fluent::i18n().language;
    Signal::derive(move || {
        let lng = Lng::try_from_id(&lang.get().id).unwrap();
        id.label(lng)
    })
}

#[allow(clippy::too_many_lines)] // TODO
#[component]
#[must_use]
pub fn App() -> impl IntoView {
    provide_meta_context();

    leptos_fluent! {{
        locales: "./locales",
        translations: [TRANSLATIONS],
        #[cfg(debug_assertions)]
        check_translations: "./src/**/*.rs",
        sync_html_tag_lang: true,
        sync_html_tag_dir: true,
        set_language_to_localstorage: true,
        initial_language_from_navigator: true,
    }};

    // -- hotkeys -- //

    let app_ref = create_node_ref::<html::Div>();
    provide_hotkeys_context(app_ref, false, scopes!());
    let accessibility_always_show_option: Option<RwSignal<bool>> = Some(RwSignal::new(false));

    use_hotkeys!(("F1") => move |()| {
      if let Some(o) = accessibility_always_show_option {
      o.set(!o.get());}
    });

    // -- signals -- //

    let (current_page, set_current_page) = RwSignal::new(Page::Home).split();
    let authorized_api = RwSignal::new(None::<api::AuthorizedApi>);
    let user_info = RwSignal::new(None::<UserInfo>);
    let logged_in = Signal::derive(move || user_info.get().is_some());
    let current_project = RwSignal::new(None::<boundary::Project>);

    let lng = current_lang();

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
                    <ContentLoader
                      file = "about.html"
                      lng = lng
                    />
                    <p class="my-4">
                      <a
                        class="rounded bg-primary px-2 py-1 text-sm font-semibold text-black shadow-sm no-underline"
                        href="/tool"
                      >
                        { move_tr!("to-the-tool") }
                      </a>
                    </p>
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
                      accessibility_always_show_option
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
                    <ContentLoader
                      file = "faq.html"
                      lng = lng
                    />
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
                    <ContentLoader
                      file = "open-source.html"
                      lng = lng
                    />
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
                    <ContentLoader lng file = "imprint.html" />
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
        <Footer lng />
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
fn ContentLoader(lng: Signal<Lng>, file: &'static str) -> impl IntoView {
    let url = Signal::derive(move || format!("content/{}/{file}", lng.get().alpha_2()));
    view! { <HtmlLoader url = url /> }
}

#[component]
#[must_use]
pub fn HtmlLoader(url: Signal<String>) -> impl IntoView {
    let html_content = create_resource(
        move || url.get(),
        |url| async move {
            fetch_html(&url)
                .await
                .map_err(|err| format!("unable to loat {url}: {err}"))
        },
    );

    view! {
      <Suspense
        fallback= move || view!{ <p>"Loading..."</p> }
      >
        {
          match html_content.get() {
              Some(Ok(content)) => {
                view! { <div class="prose" inner_html = content></div> }.into_view()

              }
              Some(Err(_)) | None => {
                view! { <div>"Failed to load content."</div> }.into_view()
              }
          }
        }
      </Suspense>
    }
}

const ABOUT_DE_HTML: &str = include_str!("../target/content/de/about.html");

async fn fetch_html(url: &str) -> anyhow::Result<String, gloo_net::Error> {
    // Make the German main page immediately available .
    if url == "content/de/about.html" {
        return Ok(ABOUT_DE_HTML.to_string());
    };
    let response = Request::get(url).send().await?;
    let html = response.text().await?;
    Ok(html)
}

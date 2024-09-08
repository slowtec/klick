use leptos::*;
use leptos_router::NavigateOptions;

use klick_boundary::{Project, ProjectId, SavedProject};

use crate::{api::AuthorizedApi, Page};

mod new_project;
mod project_list;

use self::{new_project::NewProject, project_list::ProjectList};

#[component]
pub fn Projects(
    api: Signal<Option<AuthorizedApi>>,
    current_project: RwSignal<Option<Project>>,
) -> impl IntoView {
    move || match api.get() {
        None => view! { <NotAuthorized /> }.into_view(),
        Some(api) => view! { <Authorized api current_project /> }.into_view(),
    }
}

#[component]
fn NotAuthorized() -> impl IntoView {
    view! {
      <p>
        "Sie müssen sich "
        <a class="underline" href=Page::Login.path() >"anmelden"</a>
        " um Ihre Projekte verwalten zu können."
      </p>
    }
}

const ENABLED_BUTTON_CLASS: &str = "rounded bg-primary px-2 py-1 text-sm font-semibold text-black shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600";

const DISABLED_BUTTON_CLASS: &str = "rounded bg-gray-100 px-2 py-1 text-sm font-semibold text-gray-300 shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 cursor-not-allowed";

#[component]
fn Authorized(api: AuthorizedApi, current_project: RwSignal<Option<Project>>) -> impl IntoView {
    let show_new_project = RwSignal::new(false);
    let api = RwSignal::new(api);

    let on_cancel_new = move |()| {
        show_new_project.set(false);
    };

    let projects = RwSignal::<Vec<SavedProject>>::new(vec![]);

    let error = RwSignal::<Option<String>>::new(None);

    let load_projects = Action::new(move |(): &()| {
        let api = api.get();
        async move {
            let result = api.all_projects().await;
            match result {
                Ok(p) => {
                    projects.set(p);
                    error.set(None);
                }
                Err(err) => {
                    projects.update(Vec::clear);
                    log::warn!("Unable to load projects: {err}");
                    error.set(Some(
                        "Es tut uns leid, es ist ein Kommunikationsproblem aufgetreten."
                            .to_string(),
                    ));
                }
            }
        }
    });

    let download_link: NodeRef<leptos::html::A> = create_node_ref();

    let download_pdf = Action::new(move |id: &ProjectId| {
        let id = *id;
        let api = api.get();
        let link = download_link.get().expect("<a> to exist");
        async move {
            let result = api.download_pdf_report(&id.into()).await;
            match result {
                Ok(response) => {
                    log::debug!("{}", &response.download_url);
                    link.set_attribute("href", &response.download_url).unwrap();
                    link.set_attribute("download", "klimabilanz.pdf").unwrap();
                    link.click();
                    link.remove_attribute("href").unwrap();
                }
                Err(err) => {
                    log::warn!("Unable to download PDF report: {err}");
                }
            }
        }
    });

    let on_new_success = move |_| {
        show_new_project.set(false);
        load_projects.dispatch(());
    };

    let on_delete_success = move |()| {
        load_projects.dispatch(());
    };

    let navigate = leptos_router::use_navigate();

    let on_load = move |id| {
        log::debug!("Load project {id:?}");
        let Some(project) = projects.get().iter().find(|p| p.id == id).cloned() else {
            return;
        };
        current_project.set(Some(project.into()));
        let nav_options = NavigateOptions::default();
        navigate(Page::Tool.path(), nav_options);
    };

    let on_download_pdf = move |id| {
        download_pdf.dispatch(id);
    };

    load_projects.dispatch(());

    view! {
      <div class="flex items-center justify-end gap-x-6">
        <button
          on:click = move |_| show_new_project.set(true)
          type="button"
          class = move || if show_new_project.get() { DISABLED_BUTTON_CLASS } else { ENABLED_BUTTON_CLASS }
        >
          "Neues Projekt"
        </button>
      </div>
      <Show when = move || show_new_project.get()>
        <NewProject
          api = api.into()
          on_cancel = on_cancel_new.into()
          on_success = on_new_success.into()
        />
      </Show>
      <div class="mt-8">
        <ProjectList
          api = api.into()
          projects = projects.into()
          on_load
          on_delete_success
          on_download_pdf
        />
      </div>
      // Hidden download anchor
      <a style="display:none;" node_ref=download_link></a>
    }
}

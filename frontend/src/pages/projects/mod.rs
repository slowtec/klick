use std::time::Duration;

use gloo_timers::future::TimeoutFuture;
use leptos::*;
use leptos_router::NavigateOptions;
use web_time::Instant;

use klick_boundary::{
    json_api::{DownloadId, DownloadStatus},
    Project, ProjectId, SavedProject,
};

use crate::{api::AuthorizedApi, Modal, Page};

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

    let current_download = RwSignal::new(Option::<DownloadId>::None);

    let download_pdf = Action::new(move |id: &ProjectId| {
        let id = *id;
        let api = api.get();
        async move {
            let result = api.download_pdf_report(&id.into()).await;
            match result {
                Ok(response) => {
                    log::debug!("{:?}", &response.download_id);
                    current_download.set(Some(response.download_id));
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
      { move ||
        current_download.get().map(|download_id| {
          let download_status = RwSignal::new(Option::<Result<DownloadStatus, String>>::None);

          const TIMEOUT: Duration = Duration::from_secs(5);
          const INTERVAL: Duration = Duration::from_millis(500);

          spawn_local(async move {
              let start = Instant::now();
              loop {
                TimeoutFuture::new(INTERVAL.as_millis() as u32).await;
                if start.elapsed() > TIMEOUT {
                    log::warn!("Download timed out");
                    break;
                }
                let status: Result<_,String> = api.get().download_status(&download_id).await.map_err(|err|err.to_string());
                if status.is_err() {
                    download_status.set(Some(status));
                    break;
                } else {
                    download_status.set(Some(status));
                }
              }
          });
          view!{
            <DownloadDialog
              status = download_status.into()
              current_download
            />
          }
        })
      }
    }
}

#[component]
fn DownloadDialog(
    status: Signal<Option<Result<DownloadStatus, String>>>,
    current_download: RwSignal<Option<DownloadId>>,
) -> impl IntoView {
    view! {
      <Modal>
        <h3 class="my-2 text-gray-900 font-semibold">"Download is being prepared"</h3>
        <div class="my-3 text-center">
        { move || match status.get() {
            Some(Ok(DownloadStatus::Failed(err))) | Some(Err(err))  => {
              view!{
                <p class="text-red">
                  "failed to download: "
                  { err }
                </p>
              }.into_view()
            }
            Some(Ok(DownloadStatus::Completed(url))) => {
              view!{
                <p class="button rounded bg-primary px-2 py-1 text-sm font-semibold text-black shadow-sm">
                  <a
                    target="_black"
                    download
                    href={url}
                    on:click = move |_| {
                      current_download.set(None);
                    }
                  >
                    "download"
                  </a>
                </p>
              }.into_view()
            }
            Some(Ok(DownloadStatus::Pending)) | None => {
              view!{
                <p class="text-blue">"waiting..."</p>
              }.into_view()
            }
          }
        }
        </div>
      </Modal>
    }
}

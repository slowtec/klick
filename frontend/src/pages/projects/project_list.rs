use leptos::*;

use time::{
    format_description::{well_known::Rfc3339, FormatItem},
    macros::format_description,
};

use klick_app_components::icons;
use klick_boundary::{ProjectId, SavedProject};

use crate::api::AuthorizedApi;

#[component]
pub fn ProjectList(
    api: Signal<AuthorizedApi>,
    projects: Signal<Vec<SavedProject>>,
    #[prop(into)] on_load: Callback<ProjectId, ()>,
    #[prop(into)] on_download_pdf: Callback<ProjectId, ()>,
    #[prop(into)] on_delete_success: Callback<(), ()>,
) -> impl IntoView {
    move || {
        if projects.get().is_empty() {
            return view! {
              <p>"Es wurden noch keine Projekte abgespeichert."</p>
            }
            .into_view();
        }
        view! {
          <ul role="list" class="divide-y divide-gray-100">
          {
            let mut projects = projects.get().clone();
            projects.sort_by(|a,b|{
              match (a.modified_at, b.modified_at) {
                (Some(x),Some(y)) => x.cmp(&y),
                (Some(x),None) => x.cmp(&b.created_at),
                (None,Some(y)) => a.created_at.cmp(&y),
                (None, None) => a.created_at.cmp(&b.created_at)
              }
            });

            projects.into_iter().rev().map(|project|view!{
              <li class="flex items-center justify-between gap-x-6 py-5">
                <Project api project load = on_load on_delete_success on_download_pdf />
              </li>
            }).collect::<Vec<_>>()
          }
          </ul>
        }
        .into_view()
    }
}

const DATE_TIME_FORMAT: &[FormatItem<'_>] =
    format_description!("[day].[month].[year] um [hour]:[minute]");

#[component]
fn Project(
    api: Signal<AuthorizedApi>,
    project: SavedProject,
    #[prop(into)] load: Callback<ProjectId, ()>,
    #[prop(into)] on_download_pdf: Callback<ProjectId, ()>,
    #[prop(into)] on_delete_success: Callback<(), ()>,
) -> impl IntoView {
    let error = RwSignal::<Option<String>>::new(None);

    let delete_project = create_action(move |(): &()| {
        let id = project.id;
        async move {
            let result = api.get().delete_project(id).await;
            match result {
                Ok(()) => {
                    on_delete_success.call(());
                    error.set(None);
                }
                Err(err) => {
                    log::warn!("Unable to delete project: {err}");
                    error.set(Some(
                        "Das Project konnte nicht gelöscht werden.".to_string(),
                    ));
                }
            }
        }
    });

    let offset_minutes = js_sys::Date::new_0().get_timezone_offset();

    #[allow(clippy::cast_possible_truncation)]
    let offset_seconds = -(offset_minutes as i32) * 60;

    let local_offset = time::UtcOffset::from_whole_seconds(offset_seconds)
        .map_err(|_| {
            log::warn!("Unable to determine local timezone");
        })
        .unwrap_or(time::UtcOffset::UTC);

    view! {
      <div class="min-w-0">
        <div class="flex items-start gap-x-3">
          <p class="text-sm font-semibold leading-6 text-gray-900">{ project.data.project_title }</p>
        </div>
        <div class="mt-1 flex items-center gap-x-2 text-xs leading-5 text-gray-500">
          <p class="whitespace-nowrap">
            "Erstellt am "
            <time datetime= { project.created_at.format(&Rfc3339).ok() } >
              { project.created_at.to_offset(local_offset).format(DATE_TIME_FORMAT).ok() }
            </time>
          </p>
          { project.modified_at.map(|modified|view!
            {
              <svg viewBox="0 0 2 2" class="h-0.5 w-0.5 fill-current">
                <circle cx="1" cy="1" r="1" />
              </svg>
              <p class="whitespace-nowrap">
                "Verändert am "
                <time datetime= { modified.format(&Rfc3339).ok() } >
                  { modified.to_offset(local_offset).format(DATE_TIME_FORMAT).ok() }
                </time>
              </p>
            })
          }
        </div>
      </div>
      <Menu
        load = move |()| load.call(project.id)
        delete = move |()| delete_project.dispatch(())
        download_pdf = move |()| on_download_pdf.call(project.id)
      />
    }
}

#[component]
fn Menu(
    #[prop(into)] load: Callback<(), ()>,
    #[prop(into)] delete: Callback<(), ()>,
    #[allow(unused)]
    #[prop(into)]
    download_pdf: Callback<(), ()>,
) -> impl IntoView {
    let menu_is_open = RwSignal::new(false);

    view! {
      <div class="flex flex-none items-center gap-x-4">
        <div class="relative flex-none">
          <button
            type="button"
            class="-m-2.5 block p-2.5 text-gray-500 hover:text-gray-900"
            aria-expanded="false"
            aria-haspopup="true"
            on:click = move |_| menu_is_open.update(|s|*s = !*s)
          >
            <span class="sr-only">Open options</span>
            <svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
              <path d="M10 3a1.5 1.5 0 110 3 1.5 1.5 0 010-3zM10 8.5a1.5 1.5 0 110 3 1.5 1.5 0 010-3zM11.5 15.5a1.5 1.5 0 10-3 0 1.5 1.5 0 003 0z" />
            </svg>
          </button>

          // Dropdown menu, show/hide based on menu state.
          //
          // Entering: "transition ease-out duration-100"
          //   From: "transform opacity-0 scale-95"
          //   To: "transform opacity-100 scale-100"
          // Leaving: "transition ease-in duration-75"
          //   From: "transform opacity-100 scale-100"
          //   To: "transform opacity-0 scale-95"
          <Show when = move || menu_is_open.get() >
            <div
              class="absolute right-0 z-10 mt-2 w-56 origin-top-right rounded-md bg-white py-2 shadow-lg ring-1 ring-gray-900/5 focus:outline-none"
              role="menu"
              aria-orientation="vertical"
              tabindex="-1"
            >
              <Entry
                on:click = move |_| delete.call(())
                label = "löschen"
                icon = icons::Trash()
              />
              <Entry
                on:click = move |_| download_pdf.call(())
                label = "Bericht (PDF) erzeugen"
                icon = icons::DocumentArrowDown()
              />
            </div>
          </Show>
        </div>
        <a
          href="#"
          on:click = move |_| load.call(())
          class="rounded-md bg-gray-300 px-2 py-1 text-sm font-semibold text-gray-900 shadow-sm hover:bg-gray-50 sm:block"
        >
          "Projekt laden"
        </a>
      </div>
    }
}

#[component]
fn Entry<V>(
    label: &'static str,
    icon: V,
    #[prop(optional)] href: Option<&'static str>,
) -> impl IntoView
where
    V: IntoView,
{
    view! {
      <a
        href={ href.unwrap_or("#") }
        class="flex group items-center px-3 py-1 text-sm leading-6 text-gray-700 hover:text-gray-900"
        role="menuitem"
        tabindex="-1"
      >
       { icon }
       { label }
      </a>
    }
}

use leptos::*;

use time::{
    format_description::{well_known::Rfc3339, FormatItem},
    macros::format_description,
};

use klick_boundary::{ProjectId, SavedProject};

use crate::api::AuthorizedApi;

#[component]
pub fn ProjectList(
    api: Signal<AuthorizedApi>,
    projects: Signal<Vec<SavedProject>>,
    #[prop(into)] on_load: Callback<ProjectId, ()>,
    #[prop(into)] on_delete_success: Callback<(), ()>,
) -> impl IntoView {
    move || {
        if projects.get().is_empty() {
            view! {
              <p>"Es wurden noch keine Projekte abgespeichert."</p>
            }
            .into_view()
        } else {
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
                    <Project api project load = on_load on_delete_success />
                  </li>
                }).collect::<Vec<_>>()
              }
              </ul>
            }
            .into_view()
        }
    }
}

const DATE_TIME_FORMAT: &[FormatItem<'_>] =
    format_description!("[day].[month].[year] um [hour]:[minute]");

#[component]
fn Project(
    api: Signal<AuthorizedApi>,
    project: SavedProject,
    #[prop(into)] load: Callback<ProjectId, ()>,
    #[prop(into)] on_delete_success: Callback<(), ()>,
) -> impl IntoView {
    let error = RwSignal::<Option<String>>::new(None);

    let delete_project = create_action(move |_: &()| {
        let api = api.clone();
        let id = project.id;
        async move {
            let result = api.get().delete_project(id).await;
            match result {
                Ok(_) => {
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
    let offset_seconds = -(offset_minutes as i32) * 60;
    let local_offset = time::UtcOffset::from_whole_seconds(offset_seconds)
        .map_err(|_| {
            log::warn!("Unable to determine local timezone");
        })
        .unwrap_or(time::UtcOffset::UTC);

    view! {
      <div class="min-w-0">
        <div class="flex items-start gap-x-3">
          <p class="text-sm font-semibold leading-6 text-gray-900">{ project.data.title }</p>
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
        load = move |_| load.call(project.id)
        delete = move |_| delete_project.dispatch(())
      />
    }
}

#[component]
fn Menu(
    #[prop(into)] load: Callback<(), ()>,
    #[prop(into)] delete: Callback<(), ()>,
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
              class="absolute right-0 z-10 mt-2 w-32 origin-top-right rounded-md bg-white py-2 shadow-lg ring-1 ring-gray-900/5 focus:outline-none"
              role="menu"
              aria-orientation="vertical"
              tabindex="-1"
            >
              <Entry
                on:click = move |_| delete.call(())
                label = "löschen"
                icon = view! {
                  <svg
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke-width="1.5"
                    stroke="currentColor"
                    class="mr-3 w-6 h-6 text-gray-400 group-hover:text-gray-500"
                  >
                    <path stroke-linecap="round" stroke-linejoin="round" d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0" />
                  </svg>
                }
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

use gloo_file::{File, ObjectUrl};
use leptos::*;

use crate::Page;

const EXPORT_FILE_NAME: &str = "klimabilanzklaeranlage.json";

#[component]
pub fn ProjectMenu(
    logged_in: Signal<bool>,
    #[prop(into)] save: Callback<()>,
    #[prop(into)] clear: Callback<()>,
    #[prop(into)] load: Callback<()>,
    #[prop(into)] download: Callback<(), ObjectUrl>,
    upload_action: Action<File, ()>,
) -> impl IntoView {
    let is_open = RwSignal::new(false);

    let download_link: NodeRef<leptos::html::A> = create_node_ref();
    let upload_input: NodeRef<leptos::html::Input> = create_node_ref();
    let show_upload_input = RwSignal::new(false);

    view! {
          <div class="flex items-center justify-end gap-x-6">
            <div class="relative inline-block text-left">
              <div>
                <button
                  on:click = move |_| is_open.update(|s| *s =! *s)
    type="button" class="inline-flex w-full justify-center gap-x-1.5 rounded-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50"
                  aria-expanded="true"
                  aria-haspopup="true"
                >
                  "Projekt"
                  <svg class="-mr-1 h-5 w-5 text-gray-400" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                    <path fill-rule="evenodd" d="M5.23 7.21a.75.75 0 011.06.02L10 11.168l3.71-3.938a.75.75 0 111.08 1.04l-4.25 4.5a.75.75 0 01-1.08 0l-4.25-4.5a.75.75 0 01.02-1.06z" clip-rule="evenodd" />
                  </svg>
                </button>
              </div>

              // <!--
              //   Dropdown menu, show/hide based on menu state.

              //   Entering: "transition ease-out duration-100"
              //     From: "transform opacity-0 scale-95"
              //     To: "transform opacity-100 scale-100"
              //   Leaving: "transition ease-in duration-75"
              //     From: "transform opacity-100 scale-100"
              //     To: "transform opacity-0 scale-95"
              // -->
              <Show when = move || is_open.get() >
                <div class="absolute right-0 z-10 mt-2 w-64 origin-top-right divide-y divide-gray-100 rounded-md bg-white shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none" role="menu" aria-orientation="vertical" aria-labelledby="menu-button" tabindex="-1">
                  <div class="py-1" role="none">
                    <Entry
                      on:click = move |_|{
                        load.call(());
                        is_open.set(false);
                      }
                      label = "Beispielwerte laden"
                      icon = view! {
                        <svg
                          class="mr-3 h-6 w-6"
                          fill="none"
                          viewBox="0 0 24 24"
                          stroke-width="1.5"
                          stroke="currentColor"
                        >
                          <path stroke-linecap="round" stroke-linejoin="round" d="M12 18v-5.25m0 0a6.01 6.01 0 0 0 1.5-.189m-1.5.189a6.01 6.01 0 0 1-1.5-.189m3.75 7.478a12.06 12.06 0 0 1-4.5 0m3.75 2.383a14.406 14.406 0 0 1-3 0M14.25 18v-.192c0-.983.658-1.823 1.508-2.316a7.5 7.5 0 1 0-7.517 0c.85.493 1.509 1.333 1.509 2.316V18" />
                        </svg>
                      }
                    />
                    <Entry
                      label = "Werte zurücksetzen"
                      on:click = move |_| {
                        clear.call(());
                        is_open.set(false);
                      }
                      icon = view! {
                        <svg
                          class="mr-3 h-6 w-6"
                          fill="none"
                          viewBox="0 0 24 24"
                          stroke-width="1.5"
                          stroke="currentColor"
                        >
                          <path stroke-linecap="round" stroke-linejoin="round" d="M12 9.75 14.25 12m0 0 2.25 2.25M14.25 12l2.25-2.25M14.25 12 12 14.25m-2.58 4.92-6.374-6.375a1.125 1.125 0 0 1 0-1.59L9.42 4.83c.21-.211.497-.33.795-.33H19.5a2.25 2.25 0 0 1 2.25 2.25v10.5a2.25 2.25 0 0 1-2.25 2.25h-9.284c-.298 0-.585-.119-.795-.33Z" />
                        </svg>
                      }
                    />
                  </div>
                  <Section>
                    <Entry
                      label = "hochladen"
                      on:click = move |ev| {
                          ev.prevent_default();
                          show_upload_input.set(true);
                          is_open.set(false);
                      }
                      icon = view! {
                        <svg
                          class="mr-3 h-6 w-6"
                          fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor"
                        >
                          <path stroke-linecap="round" stroke-linejoin="round" d="M19.5 14.25v-2.625a3.375 3.375 0 0 0-3.375-3.375h-1.5A1.125 1.125 0 0 1 13.5 7.125v-1.5a3.375 3.375 0 0 0-3.375-3.375H8.25m6.75 12-3-3m0 0-3 3m3-3v6m-1.5-15H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 0 0-9-9Z" />
                        </svg>
                      }
                    />
                    <Entry
                      label = "herunterladen"
                      on:click = move |ev| {
                        ev.prevent_default();
                        let object_url = download.call(());
                        let link = download_link.get().expect("<a> to exist");
                        link.set_attribute("href", &object_url).unwrap();
                        link.set_attribute("download", EXPORT_FILE_NAME).unwrap();
                        link.click();
                        link.remove_attribute("href").unwrap();
                        is_open.set(false);
                      }
                      icon = view! {
                        <svg
                          class="mr-3 h-6 w-6"
                          fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M19.5 14.25v-2.625a3.375 3.375 0 0 0-3.375-3.375h-1.5A1.125 1.125 0 0 1 13.5 7.125v-1.5a3.375 3.375 0 0 0-3.375-3.375H8.25m.75 12 3 3m0 0 3-3m-3 3v-6m-1.5-9H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 0 0-9-9Z" />
                        </svg>
                      }
                    />
                  </Section>
                  <Section>
                    <Entry
                      label = "laden"
                      href = Page::Projects.path()
                      disabled = Signal::derive(move|| !logged_in.get())
                      icon = view! {
                        <svg
                          class="mr-3 h-6 w-6"
                          fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor"
                        >
                          <path stroke-linecap="round" stroke-linejoin="round" d="M12 16.5V9.75m0 0 3 3m-3-3-3 3M6.75 19.5a4.5 4.5 0 0 1-1.41-8.775 5.25 5.25 0 0 1 10.233-2.33 3 3 0 0 1 3.758 3.848A3.752 3.752 0 0 1 18 19.5H6.75Z" />
                        </svg>
                      }
                    />
                    <Entry
                      label = "speichern"
                      on:click = move |_| {
                        save.call(());
                        is_open.set(false);
                      }
                      disabled = Signal::derive(move|| !logged_in.get())
                      icon = view! {
                        <svg
                          class="mr-3 h-6 w-6"
                          fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor"
                        >
                          <path stroke-linecap="round" stroke-linejoin="round" d="M12 9.75v6.75m0 0-3-3m3 3 3-3m-8.25 6a4.5 4.5 0 0 1-1.41-8.775 5.25 5.25 0 0 1 10.233-2.33 3 3 0 0 1 3.758 3.848A3.752 3.752 0 0 1 18 19.5H6.75Z" />
                        </svg>
                      }
                    />
                  </Section>
                </div>
              </Show>
            </div>
            <input
                class = "block text-sm bg-gray-50 rounded-md shadow-sm file:bg-primary file:rounded-md file:border-0 file:mr-4 file:py-1 file:px-2 file:font-semibold"
                type="file"
                style = move || if show_upload_input.get() { None } else { Some("display:none;") }
                accept="application/json"
                node_ref=upload_input
                on:change = move |ev| {
                  ev.prevent_default();
                  let Some(file_list) = get_file_list(upload_input) else {
                      log::debug!("No file list");
                      return;
                  };
                  let Some(file) = file_list.item(0) else {
                      log::debug!("No file selected");
                      return;
                  };
                  let gloo_file = File::from(file);
                  upload_action.dispatch(gloo_file);
                  show_upload_input.set(false);
                }
            />
            // Hidden download anchor
            <a style="display:none;" node_ref=download_link></a>
          </div>
        }
}

#[component]
fn Section(children: Children) -> impl IntoView {
    view! {
      <div class="py-1" role="none">
      { children() }
      </div>
    }
}

#[component]
fn Entry<V>(
    label: &'static str,
    icon: V,
    #[prop(optional)] href: Option<&'static str>,
    #[prop(optional)] disabled: Signal<bool>,
) -> impl IntoView
where
    V: IntoView,
{
    view! {
      <a
       href= move || if disabled.get() { None } else { Some(href.unwrap_or("#")) }
       class = move || if disabled.get() { "cursor-not-allowed text-gray-400 group flex items-center px-4 py-2 text-sm" } else { "text-gray-700 group flex items-center px-4 py-2 text-sm" }
       role="menuitem"
       tabindex="-1"
      >
        <div
          class= move || if disabled.get() { "text-gray-400" } else { "text-gray-400 group-hover:text-gray-500" }
        >
        { icon }
        </div>
        { label }
        { move ||
          if disabled.get() {
            Some(view!{
              <span class="ml-2 text-gray-300">"(nur mit Login)"</span>
            })
          } else {
            None
          }
        }
      </a>
    }
}

fn get_file_list(upload_input: NodeRef<leptos::html::Input>) -> Option<web_sys::FileList> {
    upload_input.get().expect("<input /> to exist").files()
}

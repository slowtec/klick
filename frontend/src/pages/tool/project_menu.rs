use gloo_file::{File, ObjectUrl};
use leptos::*;

use klick_app_components::icons;

use crate::Page;

const EXPORT_FILE_NAME_JSON: &str = "klimabilanzklaeranlage.json";
const EXPORT_FILE_NAME_CSV: &str = "klimabilanzklaeranlage.csv";

#[component]
pub fn ProjectMenu(
    logged_in: Signal<bool>,
    #[prop(into)] save: Callback<()>,
    #[prop(into)] clear: Callback<()>,
    #[prop(into)] load: Callback<()>,
    #[prop(into)] download: Callback<(), ObjectUrl>,
    #[prop(into)] export_csv: Callback<(), ObjectUrl>,
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
                  <icons::Bars3 />
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
                      icon = icons::LightBulb
                    />
                    <Entry
                      label = "Werte zurücksetzen"
                      on:click = move |_| {
                        clear.call(());
                        is_open.set(false);
                      }
                      icon = icons::Backspace
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
                      icon = icons::DocumentArrowUp
                    />
                    <Entry
                      label = "herunterladen"
                      on:click = move |ev| {
                        ev.prevent_default();
                        let object_url = download.call(());
                        let link = download_link.get().expect("<a> to exist");
                        link.set_attribute("href", &object_url).unwrap();
                        link.set_attribute("download", EXPORT_FILE_NAME_JSON).unwrap();
                        link.click();
                        link.remove_attribute("href").unwrap();
                        is_open.set(false);
                      }
                      icon = icons::DocumentArrowDown
                    />
                  </Section>
                  <Section>
                  <Entry
                      label = "CSV export"
                      on:click = move |ev| {
                        ev.prevent_default();
                        let object_url = export_csv.call(());
                        let link = download_link.get().expect("<a> to exist");
                        link.set_attribute("href", &object_url).unwrap();
                        link.set_attribute("download", EXPORT_FILE_NAME_CSV).unwrap();
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
                      icon = icons::CloudArrowUp
                    />
                    <Entry
                      label = "speichern"
                      on:click = move |_| {
                        save.call(());
                        is_open.set(false);
                      }
                      disabled = Signal::derive(move|| !logged_in.get())
                      icon = icons::CloudArrowDown
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

use gloo_file::{File, ObjectUrl};
use leptos::{ev::MouseEvent, *};

const EXPORT_FILE_NAME: &str = "klimabilanzklaeranlage.json";

#[component]
pub fn ActionPanel<C, LS>(
    is_logged_in: Signal<bool>,
    clear: C,
    load: LS,
    #[prop(into)] download: Callback<(), ObjectUrl>,
    #[prop(into)] save: Callback<(), ()>,
    upload_action: Action<File, ()>,
) -> impl IntoView
where
    C: Fn() + 'static,
    LS: Fn() + 'static,
{
    let download_link: NodeRef<leptos::html::A> = create_node_ref();
    let upload_input: NodeRef<leptos::html::Input> = create_node_ref();
    let show_upload_input = RwSignal::new(false);

    view! {
      <div class="flex items-center justify-end gap-x-6">
        <Button
          label = "Werte zurücksetzen"
          on_click = move |ev| {
              ev.prevent_default();
              clear();
          }
        />
        <Button
          label = "Beispielwerte laden"
          on_click = move |ev| {
              ev.prevent_default();
              load();
          }
        />
        <Button
          label = "Projekt herunterladen"
          on_click = move |ev| {
              ev.prevent_default();
              let object_url = download.call(());
              let link = download_link.get().expect("<a> to exist");
              link.set_attribute("href", &object_url).unwrap();
              link.set_attribute("download", EXPORT_FILE_NAME).unwrap();
              link.click();
              link.remove_attribute("href").unwrap();
          }
        />
        <Button
          label = "Projekt hochladen"
          on_click = move |ev| {
              ev.prevent_default();
              show_upload_input.set(true);
          }
        />
        <Button
          label = "Projekt speichern"
          on_click = move |_| save.call(())
          is_disabled = Signal::derive(move || !is_logged_in.get())
        />
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

const DISABLED_CLASS: &str = "cursor-not-allowed rounded bg-gray-100 px-2 py-1 text-sm font-semibold text-gray-400 shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600";

const ENABLED_CLASS: &str = "rounded bg-primary px-2 py-1 text-sm font-semibold text-black shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600";

#[component]
fn Button<F>(
    label: &'static str,
    on_click: F,
    #[prop(optional)] is_disabled: Signal<bool>,
) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    view! {
      <button
        type="button"
        on:click = on_click
        class = move || if is_disabled.get() { DISABLED_CLASS } else { ENABLED_CLASS }
      >
        { label }
      </button>
    }
}

fn get_file_list(upload_input: NodeRef<leptos::html::Input>) -> Option<web_sys::FileList> {
    upload_input.get().expect("<input /> to exist").files()
}

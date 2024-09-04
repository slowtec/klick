use std::rc::Rc;

use codemirror::{DocApi, Editor, EditorOptions, GutterId, Line};
use leptos::{
    html::Textarea,
    web_sys::{Document, Element},
    *,
};

const GUTTER_ERROR: GutterId = GutterId::new("gutter-error");

#[derive(Debug, Clone, PartialEq)]
pub struct ErrorMarker {
    pub line: Line, // TODO: add error message
}

#[component]
#[must_use]
pub fn CodeMirror(
    input: Signal<Option<String>>,
    errors: Signal<Vec<ErrorMarker>>,
    #[prop(into)] on_change: Callback<Option<String>, ()>,
) -> impl IntoView {
    let textarea_ref = NodeRef::<Textarea>::new();
    textarea_ref.on_load(move |el| {
        let _ = el.on_mount(move |el| {
            log::debug!("Initialize codemirror editor");

            let options = EditorOptions::default()
                .line_numbers(true)
                .gutters(&[GUTTER_ERROR]);
            let editor = Editor::from_text_area(&el, &options);
            editor.on_change(move |editor, _| {
                let value = editor.value();
                on_change.call(value);
            });

            let editor = Rc::new(editor);

            Effect::new({
                let editor = Rc::clone(&editor);
                move |_| {
                    input.try_with(|x| {
                        let txt = match x {
                            Some(v) => v,
                            None => "",
                        };
                        if editor.value() != *x {
                            editor.set_value(txt);
                        }
                    });
                }
            });

            Effect::new(move |_| {
                errors.with(|errors| {
                    editor.clear_gutter(GUTTER_ERROR);
                    let doc = document();
                    for ErrorMarker { line } in errors {
                        let marker = create_error_marker(&doc);
                        editor.set_gutter_marker(*line, GUTTER_ERROR, &marker);
                    }
                });
            });
        });
    });

    view! { <textarea _ref=textarea_ref /> }
}

const ERROR_MARKER_CLASS: &str = "CodeMirror-lint-marker-error CodeMirror-lint-marker";

#[must_use]
fn create_error_marker(document: &Document) -> Element {
    let marker = document.create_element("div").unwrap();
    marker.set_attribute("class", ERROR_MARKER_CLASS).unwrap();
    marker
}

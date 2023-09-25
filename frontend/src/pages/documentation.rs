use leptos::*;

#[component]
pub fn Documentation() -> impl IntoView {
    view! {
      <div class="ui styled fluid accordion">
        <div class="active title">
          <i class="dropdown icon"></i>
          Wie funktioniert es?
        </div>
        <div class="active content">
          <p>Erläuterung.</p>
        </div>
        <div class="title">
          <i class="dropdown icon"></i>
          Was macht die Kläranlage?
        </div>
        <div class="content">
          <p>Klären.</p>
        </div>
        <div class="title">
          <i class="dropdown icon"></i>
          Emissionsfaktor?
        </div>
        <div class="content">
          <p>Der ist hoch.</p>
        </div>
      </div>
    }
}

use leptos::*;

#[component]
pub fn Imprint() -> impl IntoView {
    view! {
      <h2 class="text-xl font-bold mb-3 mt-4">"Angaben gemäß § 5 TMG:"</h2>
      <p>
        slowtec GmbH
        <br />
        Friedrichsberg 55
        <br />
        70567 Stuttgart
      </p>
      <h3 class="text-lg font-bold mb-2 mt-3">Vertreten durch:</h3>
      <p>Dipl.-Ing. Markus Kohlhase</p>
      <h2 class="text-xl font-bold mb-3 mt-4">Kontakt</h2>
      <p>
        "eMail: post@slowtec.de"
        <br />
        "Tel.: +49 179 768 45 72"
      </p>
    }
}

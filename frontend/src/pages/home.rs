use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
      <div class="mt-10 grid max-w-xl grid-cols-1 gap-8 text-base leading-7 text-gray-700 lg:max-w-none lg:grid-cols-2">
        <div>
          <p>
            "Dies ist die erste Beta-Version des KlicK-Tools."
            <br />
            "Vielen Dank für Ihr Interesse und dafür, dass Sie zu den ersten Anwendern/-innen gehören die das Tool testen möchten."
          </p>
          <p class="mt-8">
            "Das KlicK-Tool unterstützt Städte/Gemeinden und Anlagenbetreiber dabei,
            eine erste einfache Klimabilanz ihrer Kläranlage zu erstellen.
            Das Tool befindet sich im Aufbau und steht hier als Prototyp und Validierungszwecken zur Verfügung.
            Diese Version kann noch Fehler/Bugs enthalten.
            Das Tool darf aber gerne schon in dieser Version genutzt und getestet werden.
            Zur steten Verbesserung und Erweiterung des Tool, und gerne für allgemeines Feedback bitte 
            Herrn Dr. Aennes Abbas und/oder Herrn Dipl.-Ing. Jürgen Schmidtke über die "
            <a href="https://www.umwelttechnik-bw.de/de/klimabilanz-klaeranlagen-klick">"KlicK-Projektseite"</a>"
            kontaktieren"
          </p>
        </div>
      </div>
    }
}

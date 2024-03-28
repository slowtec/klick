use leptos::*;

use crate::pages::tool::{Card, Cite, DWA_MERKBLATT_URL};

pub fn options() -> impl IntoView {
    view! {
      <Card id="recommendation-n2o-biological" title = "Lachgasemissionen bei der biologischen Reinigungsstufe" bg_color="bg-yellow">
        <p>
          "Lachgasemissionen tragen erheblich zum Gesamt-Treibhausgas-Potenzial von Kläranlagen bei.
          Das Auftreten von N₂O-Emissionen ist Anlagen-spezifisch,
          so dass zum jetzigen Stand der Forschung und des Monitorings folgende Maßnahmen
          mit Fokus auf den Betriebseinstellungen zusammengefasst werden können:"
          <Cite source = "Auszug aus dem DWA-Merkblatt 230-1 (2022, S. 23/24)" url = DWA_MERKBLATT_URL>
            <ul class="list-disc ml-5 space-y-1 text-gray-800">
              <li>
                <p>
                "Sicherstellung eines ausreichenden Schlammalters für die Nitrifikation"
                </p>
              </li>
              <li>
                <p>
                "Vergleichmäßigung der Zulauffracht bei der Einleitung von hochkonzentrierten Teilströmen
                wie zum Beispiel Industrieeinleitungen, Schlammwasser aus der Entwässerung oder leicht abbaubaren
                C-Quellen zur Stützung der Denitrifikation"
                </p>
              </li>
              <li>
                <p>
                "Vermeidung des Auftretens von Nitritkonzentrationen"
                </p>
              </li>
              <li>
                <p>
                 "ausreichendes Denitrifikationsvolumen"
                </p>
              </li>
              <li>
                <p>
                  "klares Belüftungsregime mit eindeutigen aeroben und anoxischen Zonen/Zeiten sowie Variabilität der Belüftung
                  zur Bereitstellung an die Belastung angepasster Volumen"
                </p>
              </li>
            </ul>
          </Cite>
          "Bisherige Untersuchungen der Lachgasemissionen von Kläranlagen haben gezeigt,
          dass diese häufig saisonalen Schwankungen unterliegen und in der Regel
          anlagenspezifisch sind. Unter diesem Gesichtspunkt bieten sich Mess- und
          Monitoringkampagnen an, die eine Datenbasis (idealerweise über ein Jahr)
          schaffen, um den anlagenspezifischen Emissionsfaktor zu ermitteln. Dieser
          kann für die kommunale Klimaberichterstattung genutzt werden. Darüber hinaus
          können so potenzielle Minderungsmaßnahmen besser bewertet werden."
        </p>
      </Card>
    }
}

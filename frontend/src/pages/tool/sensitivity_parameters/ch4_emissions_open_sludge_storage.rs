use leptos::*;

use crate::pages::tool::{Card, Cite, InfoBox, DWA_MERKBLATT_URL};

#[component]
pub fn CH4EmissionsOpenSludgeStorage() -> impl IntoView {
    view! {
      <Card title = "Methanemissionen aus der Schlammlagerung" bg_color="bg-blue">
        <InfoBox text = " Emissionen aus der Schlammlagerung aerob-stabilisierter Schlämme weisen ein deutliches Emissionspotenzial auf">
          <Cite source = "Auszug aus dem DWA-Merkblatt 230-1 (2022, S. 24-25)" url = DWA_MERKBLATT_URL>
            "Auch bei ordnungsgemäßem Betrieb enthalten gemeinsam aerob stabilisierte Schlämme mit ca.
            11 g oTM/(E·d) mehr leicht abbaubare Stoffe im Vergleich zu Faulschlämmen (ca. 4 g oTM/(E·d) im
            Faulschlamm), es sei denn, das aerobe Schlammalter beträgt weit über 30 d (DWA 2020). Werden
            die Schlämme über einen längeren Zeitraum gelagert bzw. gespeichert, so kann sich ein anaerobes
            Milieu einstellen, welches Methanbildung begünstigt. Bei der Lagerung bzw. Speicherung von
            aerob stabilisierten Schlämmen kann so Methan entstehen und emittieren. Das Emissionspotenzial
            liegt daher deutlich über den aus dem Betrieb einer ordnungsgemäß betriebenen Faulungsanlage
            zu erwartenden Methan-Emissionen. Aus der Lagerung nur ungenügend stabilisierter
            Schlämme können entsprechend dem höheren Anteil an Organik, höhere Methan-Emissionen entstehen.
            Zur Reduzierung dieser Emissionen ist die Bildung eines für die Methanbildung notwendigen
            Milieus zu vermeiden."
          </Cite>
        </InfoBox>
      </Card>
    }
}

use leptos::*;

use super::{Card, Cite, InfoBox, DWA_MERKBLATT_URL};

pub fn options() -> impl IntoView {
    view! {
      <Card title ="Methanemissionen aus der Vorklärung">
        <p>
          "Eine gute Bewirtschaftung Ihrer Kläranlage ist ein zentraler Faktor für eine Verbesserung der Klimabilanz.
          Bezüglich der mechanischen Reinigungsstufe und Vorklärung kann sich dies für Methanemissionen wie folgt äußern:"
        </p>
        <ul class="ml-5 mt-4 list-disc">
          <li class="my-2">
            <InfoBox text = "Vermeidung von langen Schlammaufenthaltszeiten zur Reduktion des Methanbildungspotenzials">
              <Cite source = "Auszug aus dem DWA-Merkblatt 230-1 (S.28)" url = DWA_MERKBLATT_URL>
                "In Vorklärbecken ermittelten ISGREN & MARTENSSEN (2013)
                eine durchschnittliche CH4-Konzentration (lediglich aus drei Proben)
                im Vorklärbecken der Kläranlage Sjölunda von 0,83 mg CH4/l.
                Die CH,-Konzentration lag damit höher als im Einlaufbereich der KA,
                sodass hier durch die Autoren eine Methanbildung in der Vorklärung vermutet wird.
                Die geringe Probenzahl ist hier jedoch zu beachten.
                Aufgrund der geringen Schlammaufenthaltszeiten ist die Ausbildung
                der für die Methanbildung benötigten Biozönose eher unwahrscheinlich."
              </Cite>
            </InfoBox>
          </li>
          <li class="my-2">
            <InfoBox text =
              "Kontinuierlicher Primärschlammabzug so regulieren, dass einerseits ausreichend Kohlenstoff
              für die Denitrifikation und andererseits für eine Faulung (falls gegeben) zur Verfügung steht"
            >
              <Cite source = "Auszug aus dem DWA-Merkblatt 230-1 (S.18)" url = DWA_MERKBLATT_URL>
                "In einer vergleichbaren Studie aus Dänemark wurde gezeigt,
                dass auch große Kläranlagen mit hoher Stickstoffreinigungsleistung
                erhöhte Emissionsfaktoren von mehr als 0,5% aufweisen können.
                Als möglicher Grund konnte hier unter anderem der Abzug von Schlamm
                aus der Vorklärung zur Erhöhung der Biogasausbeute in der Faulung identifiziert werden.
                Das aus der Ausfrachtung von Kohlenstoff resultierende verringerte Verhältnis CSB/N
                ist dann nicht ausreichend für eine vollständige De-nitrifikation."
              </Cite>
            </InfoBox>
          </li>
        </ul>
        <p class="mt-2">
        "THG treten an mehreren Prozessschritten auf.
        Die Minderungsmaßnahmen fokussieren sich auf Methan- und Lachgasemissionen sowie energiebedingte Emissionen.
        Für bestimmte Maßnahmen kann ein konkretes Minderungspotenzial (aus der Literatur) abgeleitet werden,
        für andere kann vorerst nur der Hinweis ausgegeben werden,
        ob sich die Klimabilanz dadurch qualitativ verbessert.
        Unter nachfolgenden Abschnitten erhalten Sie weiterführende Informationen zu einzelnen Maßnahmen
        und können gegebenenfalls Ihre Klimabilanz neu berechnen/verbessern."
        </p>
      </Card>
    }
}

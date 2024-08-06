use leptos::*;

use crate::pages::tool::{Card, Cite, InfoBox, DWA_MERKBLATT_URL};

pub fn options(accessibility_always_show_option: Option<RwSignal<bool>>) -> impl IntoView {
    view! {
      <Card id="recommendation-leak-test" title="Dichtigkeitsprüfung" bg_color="bg-yellow" accessibility_always_show_option>
        <InfoBox
          text = "Die (jährliche) Überprüfung möglicher Leckagen und deren Behebung, kann signifikant zum THG-Minderungspotenzial an Kläranlagen beitragen."
          accessibility_always_show_option
        >
          <Cite source ="Auszug aus dem DWA-Merkblatt 230-1 (S. 23 und 43)" url = DWA_MERKBLATT_URL>
            "Methan, das an verschiedenen Behältern und Leitungen durch Undichtigkeiten und/oder Schlupfverluste austreten kann.
            Die Roboter erkunden flächendeckend und identifizieren Leckagen auch an Orten,
            wo sich aufgrund der Lage zuvor kein Monitoring realisieren ließ, und visualisieren die Ergebnisse entsprechend."
          </Cite>
        </InfoBox>
        <InfoBox text = "Potentielle Undichtigkeiten können u.a. an Kläranlagenbauteilen wie Mannlöchern auftreten." accessibility_always_show_option>
          <Cite source ="Auszug aus dem DWA-Merkblatt 230-1 (S. 23 und 43)" url = DWA_MERKBLATT_URL>
            "Weitere Emissionen aus dem Faulprozess können beim Ablassen von Schwimmschlamm
             und aus Undichtigkeiten des bei leichtem Überdruck betriebenen Gassystems entstehen."
          </Cite>
        </InfoBox>
        <p>
          "Zur genauen Bestimmung wo und wieviel Methan austritt werden Messungen empfohlen,
          welche dazu genutzt werden können die THG-Minderungspotenziale genau zu bestimmen."
        </p>
      </Card>
    }
}

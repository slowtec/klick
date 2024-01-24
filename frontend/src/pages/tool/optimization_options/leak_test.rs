use leptos::*;

use super::{Card, Cite, InfoBox, DWA_MERKBLATT_URL};

pub fn options() -> impl IntoView {
    view! {
      <Card title="Dichtigkeitsprüfung">
        <InfoBox
          text = "Die (jährliche) Überprüfung möglicher Leckagen und deren Behebung, kann signifikant zum THG-Minderungspotenzial an Kläranlagen beitragen."
        >
          <Cite source ="Auszug aus dem DWA-Merkblatt 230-1 (S. 23 und 43)" url = DWA_MERKBLATT_URL>
            "Methan, das an verschiedenen Behältern und Leitungen durch Undichtigkeiten und/oder Schlupfver-luste austreten kann.
            Die Roboter erkunden flächendeckend und identifizieren Leckagen auch an Or-ten,
            wo sich aufgrund der Lage zuvor kein Monitoring realisieren ließ, und visualisieren die Ergebnisse entsprechend (Bild 7)."
          </Cite>
        </InfoBox>
        <InfoBox text = "Potentielle Undichtigkeiten können u.a. an Kläranlagenbauteilen wie Mannlöchern auftreten.">
          <Cite source ="Auszug aus dem DWA-Merkblatt 230-1 (S. 23 und 43)" url = DWA_MERKBLATT_URL>
            "Weitere Emissionen aus dem Faulprozess können beim Ablassen von Schwimmschlamm
             und aus Undichtigkeiten des bei leichtem Überdruck betriebenen Gassystems entstehen."
          </Cite>
        </InfoBox>
        <p>
          "Zur genauen Bestimmung wo und wieviel Methan austritt werden Messungen empfohlenen,
          welche dazu genutzt werden können die THG-Minderungspotenziale genau zu bestimmen."
        </p>
      </Card>
    }
}

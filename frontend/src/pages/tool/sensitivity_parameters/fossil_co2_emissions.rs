use leptos::*;

use klick_app_components::forms::*;
use klick_boundary::FormData;
use klick_domain::InputValueId as Id;

use crate::pages::tool::{
    fields::create_field, CalculationOutcome, Card, Cite, InfoBox, DWA_MERKBLATT_URL,
};

#[component]
pub fn FossilCO2Emissions(
    form_data: RwSignal<FormData>,
    input_data: ReadSignal<FormData>,
    outcome: Signal<CalculationOutcome>,
    accessibility_always_show_option: Option<RwSignal<bool>>,
) -> impl IntoView {
    let field_set = field_set(form_data.write_only(), input_data);
    let (form1, _, _) = render_field_sets(vec![field_set], accessibility_always_show_option);

    view! {
       <Card id="sensitivity-fossil-co2" title = "Fossile CO₂-Emissionen aus Abwasser" bg_color="bg-blue" accessibility_always_show_option>
         <p class="my-2">
           "Der überwiegende Teil des aus dem Abwasser freigesetzten CO₂ ist biogenen Ursprungs. Dieses CO₂ gilt daher als
                klimaneutral und wird in der Treibhausgasbilanz nicht berücksichtigt. Ein kleinerer Teil des CO₂ führt auf
                fossile und synthetische Verbindungen zurück. Schätzungen gehen davon aus, dass dies im kommunalen Abwasser
                anteilig bis zu 5–10% der organischen Fracht ausmachen kann (Law et al. 2013). Für Abwässer mit hohen Anteilen
                an gewerblichen/industriellen Einleitern (> 45 %) kann die fossile CO₂-Fracht sogar höher liegen (UBA 2022)."
         </p>
         <p class="my-2">
           "Im Folgenden können Sie auf Basis des gesamten organischer Kohlenstoffs (total organic carbon, TOC"<sub>"Zulauf"</sub>")
                des Kläranlagenzulaufs abgeschätzt, wie hoch/niedrig der Anteil an fossilem CO₂ ist. Das fossile CO₂ emittiert aus
                der biologischen Reinigung, aus der Klärgas- und der Klärschlamm-Verwertung. Über einen wählbaren CO₂-EF (fossil)
                können sie berechnen wie sich dies auf die Klimabilanz Ihrer Kläranlage auswirkt."
         </p>
         <p class="my-2">
           "Wenn Sie in der Datenerfassung keinen Wert im „TOC"<sub>"Zulauf"</sub>"“-Eingabefeld eingetragen haben schätzt das Tool die
                fossilen CO₂-Emissionen aus der biologischen Reinigung über den angegebenen CSB"<sub>"Zulauf"</sub>" mittels des theoretischen
                Umrechnungsfaktors von CSB:TOC von 1:2,6¯) abgeleitet aus der chemischen Gleichung C + O₂ → CO₂). Wenn Sie das
                untenstehende „CO₂-EF (fossil)“-Eingabefeld freilassen wird ein gemittelter CO₂-EF (fossil) von
                3,85 (nach Law et al. 2013) angenommen."
         </p>
         { form1 }
         <InfoBox text = "Zusätzlich zu den fossilen CO₂-Emissionen aus der biologischen Reinigung, wurde ein
           erheblicher Anteil dieser Emissionen in Klärschlämmen und im Klärgas gemessen" accessibility_always_show_option>
             <Cite source = "Auszug aus dem UBA Text 149/2022 (S. 5)" url = DWA_MERKBLATT_URL>
             "Die Untersuchungsergebnisse zeigen, dass Klärschlämme aus kommunalen Anlagen mit untergeordneten gewerblichen
                  Abwässern (< 45 %, berechnet als mittlere Auslastung der Einwohnerwerte abzüglich der angeschlossenen Einwohnerzahl)
                  ca. 80 % biogene Kohlenstoffanteile und Faulgase ca. 85 % biogene Kohlenstoffanteile aufweisen. Der fossile
                  Kohlenstoff ist hierbei wahrscheinlich auf schwer abbaubare synthetische Produkte bzw. fossile Rohstoffe
                  zurückzuführen. […] Bestimmt wurden Anteile von ca. 28 bis 71 % im Klärschlamm und ca. 11 bis 88 % im Faulgas."
             </Cite>
           </InfoBox>
         <p class="my-2">
           "Diese Anteile an fossilem CO₂ könnte z.B. aus dem Klärgas abgetrennt und einer technischen Nutzung
           zugeführt werden, um das THG-Emissionspotenzial der Kläranlage weiter zu reduzieren."
         </p>
         <div class="border-t pt-3 mt-4 border-gray-900/10">
         { move || {
             outcome.with(|out|out.sensitivity.output.as_ref().map(|out|{
               let out = &out.co2_equivalents;
               view! {
                 <dl class="mx-3 my-2 grid grid-cols-2 text-sm">
                   <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">"Fossile CO₂-Emissionen"</dt>
                   <dd class="text-lg py-1 px-3">
                     { format!("{:.1}", f64::from(out.fossil_emissions)).replace('.',",") }
                     <span class="ml-2 text-gray-400">{ "t CO₂-Äq./a" }</span>
                   </dd>
                   <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">"Gesamtemissionen"</dt>
                   <dd class="text-lg py-1 px-3">
                     { format!("{:.1}", f64::from(out.total_emissions)).replace('.',",") }
                     <span class="ml-2 text-gray-400">{ "t CO₂-Äq./a" }</span>
                   </dd>
                 </dl>
               }
             }))
           }
         }
         </div>
       </Card>
    }
}

fn field_set(form_data: WriteSignal<FormData>, input_data: ReadSignal<FormData>) -> FieldSet {
    let id = Id::SensitivityCO2FossilCustomFactor;

    let custom_factor_field = create_field(form_data, input_data, id);

    let fields = vec![custom_factor_field];
    FieldSet {
        title: None,
        fields,
    }
}

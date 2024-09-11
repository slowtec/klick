klick                         = KlicK
to-the-tool                   = Zum Tool
to-the-datacollection         = Zur Datenerfassung
to-the-sensitivity            = Zur Sensitivität
to-the-recommendations        = Zu den Handlungsempfehlungen
page-datacollection           = Datenerfassung
page-sensitivity              = Sensitivität
page-recommendations          = Handlungsempfehlungen
email-address                 = E-Mail Adresse
password                      = Passwort
communication-error           = Ein Kommunikationsfehler ist aufgetreten
email-or-password-invalid     = Email-Addresse oder Passwort ungültig
email-not-confirmed           = Sie haben Ihre Email-Addresse noch nicht bestätigt
something-went-wrong          = Tut uns leid, irgend etwas ist schief gelaufen

log-in-to-your-account        =  Bitte loggen Sie sich in Ihr Konto ein
resend-email-for-confirmation =  Email zur Bestätigung erneut senden
forgot-your-password          =  Passwort vergessen?
log-in = Log in

enter-login-information       = Bitte geben Sie die gewünschten Anmeldeinformationen ein
dont-have-an-account-yet      = Sie haben noch kein Konto?
already-have-an-account       = Sie haben bereits ein Konto?
user-account-offer-question   = Was bietet Ihnen ein Benutzer*innenkonto?
user-account-benefits         = Mit einem Konto können Sie Ihre Daten online verwalten.
successfully-signed-up        = Erfolgreich registriert
sign-up-success-message       = Herzlichen Glückwunsch! Sie haben Ihr Konto erfolgreich registriert. Überprüfen Sie nun Ihren E-Mail-Posteingang und bestätigen Sie die Gültigkeit Ihrer E-Mail-Adresse.

reset-password-promt          = Bitte geben Sie Ihre E-Mail-Adresse ein, um Ihr Passwort zurückzusetzen
reset-password                = Passwort zurücksetzen
how-does-it-work              = Wie funktioniert es?
reset-password-description    = Sie erhalten eine E-Mail mit einem Link, über den Sie Ihr neues Passwort festlegen können.
email-sent-to-reset-password  = E-mail zum Zurücksetzen des Passworts versandt.
reset-request-success-message = Prüfen Sie nun Ihren E-Mail-Posteingang und öffnen Sie die entsprechende E-Mail. Klicken Sie dann auf den darin enthaltenen Link, um Ihr neues Passwort einzugeben.

form_data_table_overview      = Übersicht über Eingabewerte (Datenerfassung und Sensitivität)
co2-savings                   = CO₂ Einsparung bei

# aria_label

aria_label_barchart           = Ein Balkendiagramm innerhalb der Sensitivität, welches nur angezeigt wird, wenn eine Verbesserung / Verschlechterung durch eine Auswahl eingetreten ist.

########################################### profile ###########################################

datacollection_enforcement_helper = Bitte ergänzen Sie im Eingabeformular die fehlenden Werte, damit die Emissionen berechnet und visualisiert werden können.
datacollection_missing_fields = Bitte ergänzen Sie folgende Werte, damit die Gesamtemissionen Ihrer Kläranlage, anhand verschiedener Szenarien, berechnet werden können:
sludge-bags-are-closed        = Schlammtaschen sind geschlossen
sludge-bags-are-closed-info   = Falls die Schlammtaschen des Faulturms / der Faultürme Ihrer Kläranlage geschlossen sind und nicht zur Umgebungsluft offen sind, dann dieses Feld bitte anklicken.
sludge-storage-is-closed      = Schlammlagerung ist geschlossen
sludge-storage-is-closed-info = Falls die Schlammstapelbehälter Ihrer Kläranlage dicht abgedeckt sind, dann dieses Feld bitte anklicken.

########################################### sensitivity ###########################################

# ?

sensitivity-barchart-title    = Änderungen durch Angaben der Sensitivität
sensitivity-barchart-description = Das folgende Diagramm zeigt die Änderungen der Treibhausgasemissionen [t CO₂ Äquivalente/Jahr] bzw. die [%]-Änderung der Gesamtemissionen durch die ausgewählten Emissionsfaktoren.

sensitivity-of-emission-factors = Sensitivität von Emissionsfaktoren
sensitivity-of-emission-factors-info = Unter nachfolgenden „aufklappbaren“ Abschnitten haben Sie die Möglichkeit verschiedene Emissionsfaktoren (EF) genauer zu definieren. Dabei können Sie berechnen, wie sich die jeweilige Anpassung der EF von Anlagenkomponenten bzw. der Gesamtkläranlage auf die Klimabilanz auswirkt. Sie können die Sensibilisierung/Verfeinerung auch überspringen und direkt zu den Handlungsempfehlungen übergehen (in diesem Fall rechnet das KlicK-Tool auf Basis der genannten Standardfaktoren/-parameter).

# additional_custom_emissions.rs

sensitivity-custom-emissions = Weitere benutzerdefinierte Emissionen
sensitivity-custom-emissions-description = Erweitern Sie Ihre Simulation durch die Verwendung von benutzerdefinierten Emissionstypen und -werten.

# ?
sensitivity-ch4-chp              = Methanemissionen aus Blockheizkraftwerken (BHKW)
sensitivity-sludge-storage       = Methanemissionen aus der Schlammlagerung
sensitivity-fossil-co2           = Fossile CO₂-Emissionen aus Abwasser

# fossil_co2_emissions.rs
sensitivity-fossil-co2-1         = Der überwiegende Teil des aus dem Abwasser freigesetzten CO₂ ist biogenen Ursprungs. Dieses CO₂ gilt daher als klimaneutral und wird in der Treibhausgasbilanz nicht berücksichtigt. Ein kleinerer Teil des CO₂ führt auf fossile und synthetische Verbindungen zurück. Schätzungen gehen davon aus, dass dies im kommunalen Abwasser anteilig bis zu 5–10% der organischen Fracht ausmachen kann (Law et al. 2013). Für Abwässer mit hohen Anteilen an gewerblichen/industriellen Einleitern (> 45 %) kann die fossile CO₂-Fracht sogar höher liegen (UBA 2022).
sensitivity-fossil-co2-2         = Im Folgenden können Sie auf Basis des gesamten organischer Kohlenstoffs (total organic carbon, TOC<sub>Zulauf</sub>) des Kläranlagenzulaufs abgeschätzt, wie hoch/niedrig der Anteil an fossilem CO₂ ist. Das fossile CO₂ emittiert aus der biologischen Reinigung, aus der Klärgas- und der Klärschlamm-Verwertung. Über einen wählbaren CO₂-EF (fossil) können sie berechnen wie sich dies auf die Klimabilanz Ihrer Kläranlage auswirkt.
sensitivity-fossil-co2-3         = Wenn Sie in der Datenerfassung keinen Wert im TOC<sub>Zulauf</sub>-Eingabefeld eingetragen haben schätzt das Tool die fossilen CO₂-Emissionen aus der biologischen Reinigung über den angegebenen CSB<sub>Zulauf</sub> mittels des theoretischen Umrechnungsfaktors von CSB:TOC von 1:2,6 abgeleitet aus der chemischen Gleichung C + O₂ → CO₂. Wenn Sie das untenstehende „CO₂-EF (fossil)“-Eingabefeld freilassen wird ein gemittelter CO₂-EF (fossil) von 3,85 (nach Law et al. 2013) angenommen.
sensitivity-fossil-co2-infobox-text = Zusätzlich zu den fossilen CO₂-Emissionen aus der biologischen Reinigung, wurde ein erheblicher Anteil dieser Emissionen in Klärschlämmen und im Klärgas gemessen 
sensitivity-fossil-co2-infobox-cite-source = Auszug aus dem UBA Text 149/2022 (S. 5)
sensitivity-fossil-co2-infobox-cite-text =  Die Untersuchungsergebnisse zeigen, dass Klärschlämme aus kommunalen Anlagen mit untergeordneten gewerblichen Abwässern (< 45 %, berechnet als mittlere Auslastung der Einwohnerwerte abzüglich der angeschlossenen Einwohnerzahl) ca. 80 % biogene Kohlenstoffanteile und Faulgase ca. 85 % biogene Kohlenstoffanteile aufweisen. Der fossile Kohlenstoff ist hierbei wahrscheinlich auf schwer abbaubare synthetische Produkte bzw. fossile Rohstoffe zurückzuführen. […] Bestimmt wurden Anteile von ca. 28 bis 71 % im Klärschlamm und ca. 11 bis 88 % im Faulgas.
sensitivity-fossil-co2-4         = Diese Anteile an fossilem CO₂ könnte z.B. aus dem Klärgas abgetrennt und einer technischen Nutzung zugeführt werden, um das THG-Emissionspotenzial der Kläranlage weiter zu reduzieren.

# ch4_emissions_chp.rs
sensitivity-ch4-chp-aria = Ein Balkendiagramm welches verschiedene Szenarien zur Berechnung von Methanemissionen grafisch aufzeigt und gleichzeitig zur Auswahl eines dieser Szenarien verwendet wird.
sensitivity-ch4-chp-infobox-1-text = BHKW weisen je nach Modell und Alter unterschiedliche Methanschlupfe auf
sensitivity-ch4-chp-infobox-1-cite-source = Auszug aus dem DWA-Merkblatt 230-1 (2022, S. 25)
sensitivity-ch4-chp-infobox-1-cite = Auch bei der Gasverwertung entstehen prozessbedingte Methan-Emissionen: BHKW-Motoren arbeiten nach dem Vier-Takt-Prinzip. Dabei sind zum Gasaustausch für eine kurze Zeit beim Übergang vom vierten (Ausstoßen) in den ersten (Ansaugen) Takt sowohl das Einlass- als auch das Auslassventil (teilweise) geöffnet. Durch diese Überschneidung können unter Umständen geringe Mengen unverbrannten Faulgases in den Abgasstrom gelangen. Ottomotoren haben dabei einen Methanschlupf im Bereich von 1 % bis 2 % Zündstrahlmotoren (sind für Faulgas nicht relevant) liegen höher in der Größenordnung von 2 % bis 3 %. Mikrogasturbinen (typische Leistungsklasse von 30 kW bis 65 kW) können dagegen einen Methanschlupf < 1 % erreichen (STMWI 2016).

sensitivity-ch4-chp-p-1 = Mit der folgenden Auswahl bzw. Eingabe eines eigenen Emissionsfaktors (EF) für das BHKW Ihrer Kläranlage kann Ihre Klimabilanz bezüglich der Methanemissionen verfeinert abgeschätzt werden:
sensitivity-ch4-chp-scenario = Es ist das Szenario
sensitivity-ch4-chp-scenario-2 = ist ausgewählt [in t CO₂ Äquivalente/Jahr]. Durch Anklicken kann ein anderes Szenario ausgewählt werden.

sensitivity-ch4-chp-infobox-2-text = Zusatzinformation zum Methanschlupf:
sensitivity-ch4-chp-infobox-2-cite-source = Auszug aus dem DWA-Merkblatt 230-1 (2022, S. 25)
sensitivity-ch4-chp-infobox-2-cite = Die Gaszusammensetzung, Brennraumtemperatur (Gasfeuchte), Brennraumgestaltung und Betriebsweise beeinflussen die Verbrennungsvorgänge. Bei hohen Sauerstoffkonzentrationen (Magerbetrieb), welche für die Reduktion der NOₓ,-Bildung bei hohen Temperaturen notwendig sind, steigt der Methanschlupf. Neben der Betriebsweise hat auch die Aggregateleistung einen Einfluss auf den Methan-schlupf. So hat sich bei Messungen im Betrieb gezeigt, dass unter Volllast in der Regel weniger Methan über das Abgas emittiert wird als bei Teillastbetrieb. Bei Mikrogasturbinen ist dieser Effekt sehr stark ausgeprägt und kann zu einem Anstieg bis auf > 5 % im 60-%-Teillastbetrieb führen (STMWI 2016).

# ch4_emissions_open_digesters.rs

sensitivity-open-digesters     = Methanemissionen aus offenen Faultürmen und bei der Schlammlagerung
sensitivity-open-digesters-p-1 = Durch <b>offene Schlammtaschen an Faultürmen</b> kann Methan entweichen. Nachfolgend kann für den Methanschlupf (z.B. aus einer Messkampagne oder als Schätzwert) ein Emissionsfaktor CH₄-EF [in m³/h] bilanziert werden.
sensitivity-open-digesters-p-2 = Wenn Sie das Feld leer lassen, wird mit einem gemittelten EF von 1,25 m³/h nach Li (2020) gerechnet. In beiden Fällen wird die Anzahl der Faultürme anteilig berücksichtigt (siehe Eingabe „Anzahl der Faultürme“ in der Datenerfassung).
sensitivity-open-digesters-p-3 = Wenn Sie (in der Datenerfassung) 'offene Schlammlager' ausgewählt haben, können Sie die Auswirkungen des Methanschlupfes auf die Klimabilanz Ihrer Kläranlage abschätzen. Das folgende Eingabefeld ermöglicht Ihnen dazu die Bilanzierung eines CH₄-EF [%] für Ihren Schlammstapelbehälter (z.B. auf Basis einer Restgaspotentialanalyse). Wenn Sie das Feld leer lassen, wird der Referenzwert von Parravicini et al. (2016) CH₄-EF = 2 % der gesamten Klärgasmenge verwendet.

sensitivity-open-digesters_1-text = Die Schlammlagerung trägt maßgeblich zu Methanemissionen bei
sensitivity-open-digesters_1-cite-source = Auszug aus dem DWA-Merkblatt 230-1 (S. 24)
sensitivity-open-digesters_1-cite-text = In Abhängigkeit vom technischen Ausfaulgrad der Schlammfaulung und der Lagerzeit können bei der Faulschlammlagerung noch bis zu 15 kg CO<sub>2</sub>-Äquivalente/(E·a) emittiert werden (Quelle: DWA 2020). Das entspricht einem Methanbildungspotenzial von 576 g CH<sub>4</sub>/(E·a). Für die Methan-Emissionen aus der Lagerung und Entwässerung von ausgefaultem Schlamm wird von PARRAVICINI et al. (2016) ein Bereich von 2 % bis 4,5 % der Methanproduktion angegeben.

# ch4_emissions_open_sludge_storage.rs

ch4_emissions_open_sludge_storage_1-text = Emissionen aus der Schlammlagerung aerob-stabilisierter Schlämme weisen ein deutliches Emissionspotenzial auf
ch4_emissions_open_sludge_storage_1-cite-source = Auszug aus dem DWA-Merkblatt 230-1 (2022, S. 24-25) 
ch4_emissions_open_sludge_storage_1-cite-text = Auch bei ordnungsgemäßem Betrieb enthalten gemeinsam aerob stabilisierte Schlämme mit ca. 11 g oTM/(E·d) mehr leicht abbaubare Stoffe im Vergleich zu Faulschlämmen (ca. 4 g oTM/(E·d) im Faulschlamm), es sei denn, das aerobe Schlammalter beträgt weit über 30 d (DWA 2020). Werden die Schlämme über einen längeren Zeitraum gelagert bzw. gespeichert, so kann sich ein anaerobes Milieu einstellen, welches Methanbildung begünstigt. Bei der Lagerung bzw. Speicherung von aerob stabilisierten Schlämmen kann so Methan entstehen und emittieren. Das Emissionspotenzial liegt daher deutlich über den aus dem Betrieb einer ordnungsgemäß betriebenen Faulungsanlage zu erwartenden Methan-Emissionen. Aus der Lagerung nur ungenügend stabilisierter Schlämme können entsprechend dem höheren Anteil an Organik, höhere Methan-Emissionen entstehen. Zur Reduzierung dieser Emissionen ist die Bildung eines für die Methanbildung notwendigen Milieus zu vermeiden.

# n2o_emissions.rs

sensitivity-n2o = Lachgasemissionen
n2o_emissions-h3-1 = Lachgasemissionen bei der biologischen Reinigungsstufe
n2o_emissions-p-1 = Lachgasemissionen tragen wesentlich zum gesamten Treibhausgaspotenzial von Kläranlagen bei. Die erste Abschätzung dieses Potenzials bei der Datenerhebung erfolgt mit einem Emissionsfaktor für Lachgas (N₂O-EF) nach Parravicini et al. (2016, TU Wien), Wert siehe erster Balken im untenstehenden Diagramm.
n2o_emissions-p-2 = Da das Auftreten von N₂O-Emissionen in der Regel anlagenspezifisch ist  <b> [N₂O Anlage] </b>, bietet das KlicK-Tool weitere Auswertungsszenarien für Lachgasemissionen an. Diese sind im folgenden Balkendiagramm dargestellt, einschließlich der daraus resultierenden Lachgasemissionen [als CO₂-Äquivalente].
n2o_emissions-p-3 = Durch Anklicken der einzelnen Balken im Diagramm wird das jeweilige Szenario für die untenstehende Gesamtbilanz (im Sankey-Diagramm) verwendet.

n2o_emissions-p-4-1 = Es ist das Szenario
n2o_emissions-p-4-2 = ausgewählt [in t CO₂ Äquivalente/Jahr]. Durch Anklicken kann ein anderes Szenario ausgewählt werden.

n2o_emissions-h3-2 = Lachgasemissionen bei der Prozesswasserbehandlung
n2o_emissions-p-5 = Zusätzlich können Sie (z.B. aufgrund einer eigenen Abschätzung oder einer Messkampagne) einen
            benutzerdefinierten Wert für den N₂O-EF eingeben und bilanzieren. Der EF-Faktor erscheint im
            Balkendiagramm und kann anschließend ebenfalls ausgewählt werden.
n2o_emissions-p-6 = Die Prozesswasserbehandlung in Kläranlagen kann mit erheblichen zusätzlichen Lachgasemissionen verbunden sein.
              Vasilaki et al. (2019) geben in ihrer Metastudie einen Lachgas-EF von 1,7-5,1% des Gesamtstickstoffs im Prozesswasser an.
n2o_emissions-p-7 = Durch die Eingabe der jährlich behandelten Stickstoffmenge des Prozesswassers [t/a] können
              Sie den resultierenden Anteil an den Treibhausgasemissionen [CO₂-Äq./a] abschätzen.
n2o_emissions-p-8 = Den hierfür verwendeten N₂O-EF können Sie über das Eingabefeld „N₂O-EF Prozesswasser“ unten frei
              wählen oder leer lassen, um mit einem mittleren EF von 2% (nach Vasilaki et al. 2019) zu rechnen.
########################################### recommendation ###########################################

recommendation-barchart-title = Änderungen durch Optionen der Handlungsmaßnahmen
recommendation-barchart-description  = Die folgende Grafik zeigt die Änderungen der Treibhausgasemissionen [t CO₂ Äquivalente/Jahr] bzw. % der Gesamtemissionen durch die ausgewählten Handlungsmaßnahmen.

# ch4_emissions_open_digesters.rs
recommendation-methan-emissions  = Methanemissionen aus offenen Faultürmen und bei der Schlammlagerung
recommendation-ch4-open-digesters-p-1 = Das Schließen von Schlammtaschen an Faultürmen und der Schlammlager wirkt sich durch die Eindämmung von Methanschlupfen positiv auf die Klimabilanz von Kläranlagen aus. Dies können Sie über die nachfolgenden Checkboxen bilanzieren.
recommendation-ch4-open-closing-sludge-bags = Schließen der Schlammtaschen
recommendation-ch4-open-closing-sludge-storage = Schließen der Schlammlagerung

# excess_energy_co2_equivalent.rs
recommenation-excess-energy      = Energiebedingte Emissionen
recommenation-excess-energy-p-1 = <b>Energiesparmaßnahmen</b> und <b>Erneuerbare Energien</b> können maßgeblich zur Minderung indirekter Emissionen und zur Energieautarkie beitragen. Um die positiven Auswirkungen eines Zubaus der erneuerbaren Energien: Photovoltaik (PV), Wind-, Wasserkraft und/oder Abwärmenutzung aufzuzeigen, können nachfolgend verschiedene Szenarien bilanziert werden. Wenn Sie die jeweilige Technologie nicht bilanzieren wollen können Sie das jeweilige Feld freilassen.
recommenation-excess-energy-p-2-1 = Ihre Kläranlage ist energieneutral. Die Kläranlage spart
recommenation-excess-energy-p-2-2 = t CO2-Äq./a ein.
recommenation-excess-energy-p-3-1 = Ihre Kläranlage benötigt weiterhin externen Strom (Versorger), wodurch 
recommenation-excess-energy-p-3-2 = t CO₂-Äq./a energiebedingte Emissionen entstehen.

# leak_test.rs
recommendation-leak-test         = Dichtigkeitsprüfung
recommendation-leak-test_1-text = Die (jährliche) Überprüfung möglicher Leckagen und deren Behebung, kann signifikant zum THG-Minderungspotenzial an Kläranlagen beitragen. 
recommendation-leak-test_1-cite-source = Auszug aus dem DWA-Merkblatt 230-1 (S. 23 und 43)
recommendation-leak-test_1-cite-text = Methan, das an verschiedenen Behältern und Leitungen durch Undichtigkeiten und/oder Schlupfverluste austreten kann. Die Roboter erkunden flächendeckend und identifizieren Leckagen auch an Orten, wo sich aufgrund der Lage zuvor kein Monitoring realisieren ließ, und visualisieren die Ergebnisse entsprechend.
recommendation-leak-test_2-text = Potentielle Undichtigkeiten können u.a. an Kläranlagenbauteilen wie Mannlöchern auftreten.
recommendation-leak-test_2-cite-source = Auszug aus dem DWA-Merkblatt 230-1 (S. 23 und 43)
recommendation-leak-test_2-cite-text = Weitere Emissionen aus dem Faulprozess können beim Ablassen von Schwimmschlamm und aus Undichtigkeiten des bei leichtem Überdruck betriebenen Gassystems entstehen.
recommendation-leak-test-p-1 = Zur genauen Bestimmung wo und wieviel Methan austritt werden Messungen empfohlen, welche dazu genutzt werden können die THG-Minderungspotenziale genau zu bestimmen.

# ch4_emissions_pre_treatment.rs
recommendation-ch4-pre-treatment = Methanemissionen aus der Vorklärung
recommendation-ch4-pre-treatment-1 = Eine gute Bewirtschaftung Ihrer Kläranlage ist ein zentraler Faktor für eine Verbesserung der Klimabilanz. Bezüglich der mechanischen Reinigungsstufe und Vorklärung kann sich dies für Methanemissionen wie folgt äußern:
recommenations_ch4_emissions_pre_treatment_infobox_1-text = Vermeidung von langen Schlammaufenthaltszeiten zur Reduktion des Methanbildungspotenzials
recommenations_ch4_emissions_pre_treatment_infobox_1-cite-source = Auszug aus dem DWA-Merkblatt 230-1 (S.28)
recommenations_ch4_emissions_pre_treatment_infobox_1-cite-text = In Vorklärbecken ermittelten ISGREN & MARTENSSEN (2013) eine durchschnittliche CH₄-Konzentration (lediglich aus drei Proben) im Vorklärbecken der Kläranlage Sjölunda von 0,83 mg CH₄/L. Die CH₄-Konzentration lag damit höher als im Einlaufbereich der KA, sodass hier durch die Autoren eine Methanbildung in der Vorklärung vermutet wird. Die geringe Probenzahl ist hier jedoch zu beachten. Aufgrund der geringen Schlammaufenthaltszeiten ist die Ausbildung der für die Methanbildung benötigten Biozönose eher unwahrscheinlich.
recommenations_ch4_emissions_pre_treatment_infobox_2-text = Kontinuierlicher Primärschlammabzug so regulieren, dass einerseits ausreichend Kohlenstoff für die Denitrifikation und andererseits für eine Faulung (falls gegeben) zur Verfügung steht
recommenations_ch4_emissions_pre_treatment_infobox_2-cite-source = Auszug aus dem DWA-Merkblatt 230-1 (S.18)
recommenations_ch4_emissions_pre_treatment_infobox_2-cite-text = In einer vergleichbaren Studie aus Dänemark wurde gezeigt, dass auch große Kläranlagen mit hoher Stickstoffreinigungsleistung erhöhte Emissionsfaktoren von mehr als 0,5% aufweisen können. Als möglicher Grund konnte hier unter anderem der Abzug von Schlamm aus der Vorklärung zur Erhöhung der Biogasausbeute in der Faulung identifiziert werden. Das aus der Ausfrachtung von Kohlenstoff resultierende verringerte Verhältnis CSB/N ist dann nicht ausreichend für eine vollständige Denitrifikation.

# n2o_emissions_in_the_biological_treatment_stage.rs
recommendation-n2o-biological    = Lachgasemissionen bei der biologischen Reinigungsstufe
recommendation-n2o-biological_p_1 = Lachgasemissionen tragen erheblich zum Gesamt-Treibhausgas-Potenzial von Kläranlagen bei. Das Auftreten von N₂O-Emissionen ist Anlagen-spezifisch, so dass zum jetzigen Stand der Forschung und des Monitorings folgende Maßnahmen mit Fokus auf den Betriebseinstellungen zusammengefasst werden können:
recommendation-n2o-biological_1-cite-source = Auszug aus dem DWA-Merkblatt 230-1 (2022, S. 23/24)
recommendation-n2o-biological_p_2 = Sicherstellung eines ausreichenden Schlammalters für die Nitrifikation
recommendation-n2o-biological_p_3 = Vergleichmäßigung der Zulauffracht bei der Einleitung von hochkonzentrierten Teilströmen wie zum Beispiel Industrieeinleitungen, Schlammwasser aus der Entwässerung oder leicht abbaubaren C-Quellen zur Stützung der Denitrifikation
recommendation-n2o-biological_p_4 = Vermeidung des Auftretens von Nitritkonzentrationen
recommendation-n2o-biological_p_5 = ausreichendes Denitrifikationsvolumen
recommendation-n2o-biological_p_6 = klares Belüftungsregime mit eindeutigen aeroben und anoxischen Zonen/Zeiten sowie Variabilität der Belüftung zur Bereitstellung an die Belastung angepasster Volumen
recommendation-n2o-biological_p_7 = Bisherige Untersuchungen der Lachgasemissionen von Kläranlagen haben gezeigt, dass diese häufig saisonalen Schwankungen unterliegen und in der Regelanlagenspezifisch sind. Unter diesem Gesichtspunkt bieten sich Mess- und Monitoringkampagnen an, die eine Datenbasis (idealerweise über ein Jahr) schaffen, um den anlagenspezifischen Emissionsfaktor zu ermitteln. Dieser kann für die kommunale Klimaberichterstattung genutzt werden. Darüber hinaus können so potenzielle Minderungsmaßnahmen besser bewertet werden.

# n2o_emissions_side_stream_system.rs
recommendation-n2o-side-stream   = Lachgasemissionen bei der Prozesswasserbehandlung
recommendation-n2o-side-stream_p_1 = Da es sich bei Prozesswasserbehandlungsanlagen um relativ kleine Becken handelt, können die Lachgasemissionen hier durch Abdeckung und Abluftbehandlung (Oxidation) beseitigt werden.
recommendation-n2o-side-stream_p_2 = Im Sinne der Nachhaltigkeit und der Kreislaufschließung kann eine Stickstoffrückgewinnungsanlage integriert werden.

########################################### project menu ###########################################

project-label = Projekt
project-reset-values = Werte zurücksetzen
project-load-example-values = Beispielwerte laden
project-load-from-file = Datei laden
project-save-to-file = Datei speichern
project-export-csv = CSV-Datei exportieren
project-load-from-online = Projekt aus Online-Speicher laden
project-save-to-online = Projekt online speichern

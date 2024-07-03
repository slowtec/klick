use klick_domain::InputValueId as Id;

#[must_use]
pub fn metadata(id: &Id) -> FieldMetaData {
    metadata_de()
        .into_iter()
        .find(|(x, _)| x == id)
        .map(|(_, m)| m)
        .unwrap()
}

pub enum Placeholder {
    /// Use the label of the value
    Label,

    /// Custom text
    Text(&'static str),

    /// If there is a default value, render it as string
    DefaultValue,

    // Nothing
    None,
}

impl Placeholder {
    #[must_use]
    pub const fn label() -> Self {
        Self::Label
    }

    #[must_use]
    pub const fn text(txt: &'static str) -> Self {
        Self::Text(txt)
    }

    #[must_use]
    pub const fn default_value() -> Self {
        Self::DefaultValue
    }

    #[must_use]
    pub const fn none() -> Self {
        Self::None
    }
}

fn metadata_de() -> [(Id, FieldMetaData); 42] {
    use FieldMetaData as M;
    use Placeholder as P;
    [
        (
            Id::ProjectName,
            M {
                placeholder: P::text("Projektname"),
                description : "In diesem Feld können Sie einen Namen für Ihr Projekt hinterlegen. In der <b>angemeldeten</b> Version,
                dient der Projektname der Speicherung Ihrer Eingaben/Ergebnisse unter dem Reiter „Projekte“.

                Wenn Sie sich <b>nicht angemeldet</b> haben, wird der Projektname ausschließlich nur auf Ihrer Festplatte
                gespeichert und in Ihrem lokalen Browser verarbeitet. Weitere Informationen zur Datenverarbeitung
                finden Sie in den <b>FAQ</b>."
            }
        ),
        (
            Id::PlantName,
            M {
                placeholder: P::text("Name der Kläranlage"),
                description: "Die Angabe des Namens und/oder Orts sind freiwillig. Alternativ kann für das Feld ein Platzhalter eingetragen werden. Sämtliche Eintragungen können nur von Ihnen (nicht der UTBW) eingesehen oder gespeichert werden.",
            }
        ),
        (
            Id::PopulationEquivalent,
            M {
                placeholder: P::text("Angeschlossene Einwohner"),
                description: "Ausbaugröße Ihrer Kläranlage in Einwohnerwerten (EW) als Summe der angeschlossenen Einwohner (E) und der gewerblichen Einwohnergleichwerte (EGW).",

            }
        ),
        (
            Id::Wastewater,
            M {
                placeholder: P::text("Abwassermenge"),
                description: "Die jährliche (a) Abwassermenge in Kubikmeter (m³) im Zulauf Ihrer Kläranlage.",
            }
        ),
        (
            Id::InfluentChemicalOxygenDemand,
            M {
                placeholder: P::text("CSB"),
                description: "Der Jahresmittelwert des chemischen Sauerstoffbedarf (CSB) des Abwassers im Zulauf Ihrer Kläranlage in Milligramm (mg) pro Liter (L).",
            }
        ),
        (
            Id::InfluentNitrogen,
            M {
                placeholder: P::text("Gesamtstickstoff"),
                description: "Der Gesamtstickstoff-Gehalt des Abwassers (TN) im Zulauf Ihrer Kläranlage in Milligramm (mg) pro Liter (L) als Jahresmittelwert.",
            }
        ),
        (
            Id::InfluentTotalOrganicCarbohydrates,
            M {
                placeholder: P::text("TOC"),
                description: "Der Jahresmittelwert des Gesamten organischen Kohlenstoffs (Total Organic Carbon, TOC)
                        des Abwassers im Zulauf Ihrer Kläranlage in Milligramm (mg) pro Liter (L).<br>
                        Wenn Sie keinen Wert für den TOC haben dann dieses Feld bitte freilassen
                        (Anm.: für die Berechnung der fossilen CO₂-Emissionen wird in diesem Fall der CSB verwendet).",
            }
        ),
        (
            Id::EffluentNitrogen,
            M {
                placeholder: P::text("Gesamtstickstoff"),
                description: "Der Gesamtstickstoff-Gehalt des Abwassers (TN) im Ablauf Ihrer Kläranlage in Milligramm (mg) pro Liter (L) als Jahresmittelwert.",
            }
        ),
        (
            Id::EffluentChemicalOxygenDemand,
            M {
                placeholder: P::text("CSB"),
                description: "Der Jahresmittelwert des chemischen Sauerstoffbedarf (CSB) des Abwassers im Ablauf Ihrer Kläranlage in Milligramm (mg) pro Liter (L).",
            }
        ),
        (
            Id::TotalPowerConsumption,
            M {
                placeholder: P::text("Gesamtstrombedarf"),
                description: "Der Gesamt-Strombedarf Ihrer Kläranlage in Kilowattstunden (kWh) pro Jahr (a).",
            }
        ),
        (
            Id::OnSitePowerGeneration,
            M {
                placeholder: P::text("Eigenstrom"),
                description: "Anteil der Eigenstromerzeugung in Kilowattstunden (kWh) pro Jahr (a). Falls kein Eigenstrom erzeugt wird, dieses Feld bitte freilassen.",
            }
        ),
        (
            Id::EmissionFactorElectricityMix,
            M {
                placeholder : P::text("485"),
                description : "Angabe des Emissionsfaktors des von extern bezogenen Strommixes in Gramm (g) CO₂ pro Kilowattstunde (kWh). Falls dieser Wert nicht verfügbar ist, bitte den Referenzwert stehen lassen.",
            }
        ),
        (
            Id::GasSupply,
            M {
                placeholder: P::text("Gasbezug"),
                description: "Menge an Gas (Erdgas/Biogas) in Kubikmeter (m³) pro Jahr (a) die von einem externen Versorger bezogen werden. Falls an Ihrer Kläranlage kein Gas von extern bezogen wird, dieses Feld bitte freilassen.",
            }
        ),
        (
            Id::PurchaseOfBiogas,
            M {
                placeholder: P::text("Biogas Bezug"),
                description: "Falls Ihre Kläranlage Biogas von extern bezieht, dieses Feld bitte anklicken.",
            }
        ),
        (
            Id::HeatingOil,
            M {
                placeholder: P::text("Heizölbezug"),
                description: "Menge an Heizöl (z.B. für die Beheizung von Gebäuden) in Litern (L) pro Jahr (a) die von einem externen Versorger bezogen werden. Falls an Ihrer Kläranlage kein Heizöl von extern bezogen wird, dieses Feld bitte freilassen."
            }
        ),
        (
            Id::SewageGasProduced,
            M {
                placeholder: P::text("Klärgas"),
                description: "Das an Ihrer Kläranlage erzeugte Klärgas in Kubikmeter (m³) pro Jahr (a). Falls an Ihrer Kläranlage kein Klärgas erzeugt wird, dieses Feld bitte freilassen.",
            }
        ),
        (
            Id::MethaneFraction,
            M {
                placeholder: P::text("62"),
                description:"Der Methangehalt des an Ihrer Kläranlage erzeugten Klärgases in Prozent (%). Falls an Ihrer Kläranlage kein Klärgas erzeugt wird, dieses Feld bitte freilassen.",
            }
        ),
        (
            Id::SludgeTreatmentDigesterCount,
            M {
                placeholder: P::text("Anzahl Faultürme"),
                description: "Falls auf Ihrer Kläranlage eine Faulung vorhanden ist, dann geben Sie bitte die Anzahl der Faultürme ein. Falls nicht lassen Sie das Feld bitte offen oder tragen eine 0 ein.",
            }
        ),
        (
            Id::SludgeTreatmentBagsAreOpen,
            M {
                placeholder: P::none(),
                description: "Falls die Schlammtaschen des Faulturms / der Faultürme Ihrer Kläranlage geschlossen sind und nicht zur Umgebungsluft offen sind, dann dieses Feld bitte anklicken.",
            }
        ),
        (
            Id::SludgeTreatmentStorageContainersAreOpen,
            M {
                placeholder: P::none(),
                description: "Falls die Schlammstapelbehälter Ihrer Kläranlage dicht abgedeckt sind, dann dieses Feld bitte anklicken.",
            }
        ),
        (
            Id::SludgeTreatmentDisposal,
            M {
                placeholder: P::text("Masse entwässert"),
                description: "Angabe der Menge an Klärschlamm in Tonnen (t) die zur Entsorgung anfallen.",
            }
        ),
        (
            Id::SludgeTreatmentTransportDistance,
            M {
                placeholder: P::text("Entfernung"),
                description: "Entfernung von Ihrer Kläranlage zum Entsorgungsort des Klärschlamms in Kilometer (km). Die Angabe ist unabhängig von der Entsorgungsart (z.B. Verbrennung) oder der Transportform (z.B. entwässert/trocken). Falls der Klärschlamm auf Ihrer Kläranlage entsorgt wird, dieses Feld bitte freilassen.",
            }
        ),
        (
            Id::SideStreamTreatmentTotalNitrogen,
            M {
                placeholder: P::text("Gesamtstickstoff"),
                description: "Falls auf Ihrer Kläranlage eine Prozesswasserbehandlung vorhanden ist, dann geben Sie bitte deren jährliche Gesamtsticksoffmenge in Tonnen [t/a] ein. Falls nicht lassen Sie das Feld bitte offen oder tragen eine 0 ein. ",
            }
        ),
        (
            Id::OperatingMaterialFeCl3,
            M {
               placeholder: P::text("Lösung"),
               description: "Angabe der pro Jahr (a) eingesetzten Menge an Eisen(III)-chlorid (FeCl3) in Tonnen (t).",
            }
        ),
        (
            Id::OperatingMaterialFeClSO4,
            M {
               placeholder: P::text("Lösung"),
               description: "Angabe der pro Jahr (a) eingesetzten Menge an Eisenchloridsulfat (FeClSO4) in Tonnen (t).",
            }
        ),
        (
            Id::OperatingMaterialCaOH2,
            M {
                placeholder: P::text("Branntkalk"),
                description: "Angabe der pro Jahr (a) eingesetzten Menge an Kalkhydrat (Ca(OH)2) in Tonnen (t).",
            }
        ),
        (
            Id::OperatingMaterialSyntheticPolymers,
            M {
                placeholder: P::text("Polymere"),
                description: "Angabe der pro Jahr (a) eingesetzten Menge an synthetischen Polymeren in Tonnen (t).",
            }
        ),
        (
            Id::SensitivitySludgeBagsCustomFactor,
            M {
                placeholder: P::default_value(),
                description: "Über dieses Eingabefeld können Sie (z.B. basierend auf einer eigenen Abschätzung oder einer Messkampagne) einen Wert für den EF CH₄ eintragen.",
            }
        ),
        (
            Id::SensitivitySludgeStorageCustomFactor,
            M {
                placeholder: P::default_value(),
                description: "Über dieses Eingabefeld können Sie (z.B. basierend auf einer eigenen Abschätzung oder einer Messkampagne) einen Wert für den EF CH₄ eintragen.",
            }
        ),
        (
            Id::SensitivityCH4ChpCustomFactor,
            M {
                placeholder: P::default_value(),
                description: "Über dieses Eingabefeld können Sie (z.B. basierend auf einer eigenen Abschätzung oder einer Messkampagne) einen Wert für den EF CH₄ eintragen.",
            }
        ),
        (
            Id::SensitivityCO2FossilCustomFactor,
            M {
                placeholder: P::default_value(),
                description: "Über dieses Eingabefeld können Sie (z.B. basierend auf einer eigenen Abschätzung oder einer Messkampagne) einen Wert für den EF CO₂ eintragen.",
            }
        ),
        (
            Id::SensitivityN2OCustomFactor,
            M {
                placeholder: P::default_value(),
                description: "Über dieses Eingabefeld können Sie (z.B. anhand einer eigenen Abschätzung
                  oder einer Messkampagne) einen Wert für den EF N₂O eintragen.
                  <br/>Weiter muss die Auswahlmöglichkeit (Benutzerdefiniert) manuell ausgewählt werden, um den eingegebenen Wert zu verwenden.",
            }
        ),
        (
            Id::SensitivityN2OSideStreamFactor,
            M {
                placeholder: P::default_value(),
                description: "Über dieses Eingabefeld können Sie (z.B. anhand einer eigenen Abschätzung oder
                    einer Messkampagne) einen Wert für den EF der Prozesswasserbehandlung eintragen.",
            }
        ),
        (
            Id::ScenarioDistrictHeating,
            M {
                placeholder: P::text("Jahresleistung"),
                description: "Angabe der Abgabeleistung an Fern-/Nahwärme in Kilowattstunden (kWh) pro Jahr (a).",

            }
        ),
        (
            Id::ScenarioEstimatedSelfWaterEnergyUsage,
            M {
                placeholder: P::default_value(),
                description: "Geschätzte Eigennutzung der Wasserkraftleistung in Prozent (%).",
            }
        ),
        (
            Id::ScenarioWaterEnergyExpansion,
            M {
                placeholder: P::text("Jahresleistung"),
                description: "Angabe des Zubaus an Wasserkraftleistung in Kilowattstunden (kWh) pro Jahr (a)."
            }
        ),
        (
            Id::ScenarioEstimatedSelfWindEnergyUsage,
            M {
                placeholder: P::default_value(),
                description: "Geschätzte Eigennutzung der Windkraftleistung in Prozent (%).",
            }
        ),
        (
            Id::ScenarioWindEnergyExpansion,
            M {
                placeholder: P::text("Jahresleistung"),
                description: "Angabe des Zubaus an Windkraftleistung in Kilowattstunden (kWh) pro Jahr (a).",
            }
        ),
        (
            Id::ScenarioEstimatedSelfPhotovolaticUsage,
            M {
                placeholder: P::default_value(),
                description: "Geschätzte Eigennutzung der Photovoltaikleistung in Prozent (%).",
            }
        ),
        (
           Id::ScenarioPhotovoltaicEnergyExpansion,
            M {
                    placeholder: P::text("Jahresleistung"),
                    description: "Angabe des Zubaus an Photovoltaikleistung in Kilowattstunden (kWh) pro Jahr (a).",
            }
        ),
        (
            Id::ScenarioFossilEnergySaving,
            M {
                    placeholder: P::text("Jahreseinsparung"),
                    description: "Angabe der geschätzten Energieeinsparung bei fossilen Energieträgern (z.B. Heizöl/Erdgas) in Prozent (%).",
            }
        ),
        (
            Id::ScenarioProcessEnergySaving,
            M {
               placeholder: P::text("Jahreseinsparung"),
               description: "Angabe der geschätzten Energieeinsparung bei Kläranlagen-Prozessen in Prozent (%).",
            }
        ),
    ]
}

pub struct FieldMetaData {
    pub description: &'static str,
    pub placeholder: Placeholder,
}

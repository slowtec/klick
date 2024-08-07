use serde::Serialize;

use klick_boundary::FormData;
use klick_domain::{
    self as domain,
    units::{Percent, RatioExt},
    InputValueId as Id, Value,
};

use crate::{Lng, ValueLabel, ValueUnit};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Formatting {
    Text,
    LaTeX,
}

impl Formatting {
    // TODO: rename to fmt_unit
    pub fn fmt<V>(&self, v: V) -> Option<&'static str>
    where
        V: ValueUnit,
    {
        match self {
            Self::Text => v.unit_as_text(),
            Self::LaTeX => v.unit_as_latex(),
        }
    }
    pub fn fmt_label<L>(&self, id: L) -> &'static str
    where
        L: ValueLabel,
    {
        match self {
            Self::Text => id.label(),
            Self::LaTeX => id.label_latex(),
        }
    }
}

#[derive(Default, Serialize)]
pub struct Table {
    pub sections: Vec<TableSection>,
}

#[derive(Serialize)]
pub struct TableSection {
    pub title: String,
    pub rows: Vec<(&'static str, Option<String>, Option<&'static str>)>,
}

#[must_use]
pub fn plant_profile_as_table(data: &FormData, formatting: Formatting) -> Table {
    // TODO: use as parameter
    let lang = Lng::De;

    let sections = [
        (
            "Angaben zur Kläranlage",
            vec![Id::PlantName, Id::PopulationEquivalent, Id::Wastewater],
        ),
        (
            "Zulauf-Parameter (Jahresmittelwerte)",
            vec![
                Id::InfluentNitrogen,
                Id::InfluentChemicalOxygenDemand,
                Id::InfluentTotalOrganicCarbohydrates,
            ],
        ),
        (
            "Ablauf-Parameter (Jahresmittelwerte)",
            vec![Id::EffluentNitrogen, Id::EffluentChemicalOxygenDemand],
        ),
        (
            "Energiebedarf",
            vec![
                Id::TotalPowerConsumption,
                Id::OnSitePowerGeneration,
                Id::EmissionFactorElectricityMix,
                Id::GasSupply,
                Id::PurchaseOfBiogas,
                Id::HeatingOil,
                Id::SewageGasProduced,
                Id::MethaneFraction,
            ],
        ),
        (
            "Klärschlammbehandlung",
            vec![
                Id::SludgeTreatmentDigesterCount,
                Id::SludgeTreatmentDisposal,
                Id::SludgeTreatmentTransportDistance,
                Id::SludgeTreatmentBagsAreOpen,
                Id::SludgeTreatmentStorageContainersAreOpen,
            ],
        ),
        (
            "Prozesswasserbehandlung",
            vec![Id::SideStreamTreatmentTotalNitrogen],
        ),
        (
            "Eingesetzte Betriebsstoffe",
            vec![
                Id::OperatingMaterialFeCl3,
                Id::OperatingMaterialFeClSO4,
                Id::OperatingMaterialCaOH2,
                Id::OperatingMaterialSyntheticPolymers,
            ],
        ),
    ]
    .into_iter()
    .map(|(title, ids)| TableSection {
        title: title.to_string(),
        rows: ids
            .into_iter()
            .map(|id| {
                (
                    formatting.fmt_label(id),
                    data.get(&id).as_ref().map(|v| lang.format_value(v)),
                    formatting.fmt(id),
                )
            })
            .collect(),
    })
    .collect();
    Table { sections }
}

#[must_use]
pub fn sensitivity_parameters_as_table(
    data: &FormData,
    formatting: Formatting,
    output: Option<&domain::EmissionsCalculationOutcome>,
) -> Table {
    let lang = Lng::De;

    let n2o_emission_factor: Option<Value> =
        output.map(|output| output.emission_factors.n2o.convert_to::<Percent>().into());

    let ch4_chp_emission_factor: Option<Value> =
        output.map(|output| output.emission_factors.ch4.convert_to::<Percent>().into());

    let sections = vec![
        (
            "Lachgasemissionen",
            vec![
                (
                    Id::SensitivityN2OCalculationMethod,
                    data.get(&Id::SensitivityN2OCalculationMethod),
                ),
                (Id::SensitivityN2OCustomFactor, n2o_emission_factor.as_ref()),
                (
                    Id::SensitivityN2OSideStreamFactor,
                    data.get(&Id::SensitivityN2OCustomFactor),
                ),
            ],
        ),
        (
            "Methanemissionen aus Blockheizkraftwerken (BHKW)",
            vec![
                (
                    Id::SensitivityCH4ChpCalculationMethod,
                    data.get(&Id::SensitivityCH4ChpCalculationMethod),
                ),
                (
                    Id::SensitivityCH4ChpCustomFactor,
                    ch4_chp_emission_factor.as_ref(),
                ),
            ],
        ),
        (
            "Methanemissionen aus offenen Faultürmen und bei der Schlammlagerung",
            vec![
                (
                    Id::SensitivitySludgeBagsCustomFactor,
                    data.get(&Id::SensitivitySludgeBagsCustomFactor),
                ),
                (
                    Id::SensitivitySludgeStorageCustomFactor,
                    data.get(&Id::SensitivitySludgeStorageCustomFactor),
                ),
            ],
        ),
        (
            match formatting {
                Formatting::Text => "Fossile CO₂-Emissionen aus Abwasser",
                Formatting::LaTeX => "Fossile $CO_2$-Emissionen aus Abwasser",
            },
            vec![(
                Id::SensitivityCO2FossilCustomFactor,
                data.get(&Id::SensitivityCO2FossilCustomFactor),
            )],
        ),
    ]
    .into_iter()
    .map(|(title, sections)| TableSection {
        title: title.to_string(),
        rows: sections
            .into_iter()
            .map(|(id, value)| {
                (
                    formatting.fmt_label(id),
                    value.as_ref().map(|v| lang.format_value(v)),
                    formatting.fmt(id),
                )
            })
            .collect(),
    })
    .collect();
    Table { sections }
}

#[must_use]
pub fn co2_equivalents_as_table(eq: &domain::CO2Equivalents, _unit: Formatting) -> Table {
    // TODO: use as parameger
    let lang = Lng::De;

    // TODO:
    // - value IDs and it's labels
    // - use unit parameter
    let rows = vec![
        (
            "N₂O Anlage",
            Some(lang.format_number(f64::from(eq.n2o_plant))),
            Some("t"),
        ),
        (
            "N₂O Gewässer",
            Some(lang.format_number(f64::from(eq.n2o_water))),
            Some("t"),
        ),
        (
            "N₂O Prozesswasserbehandlung",
            Some(lang.format_number(f64::from(eq.n2o_side_stream))),
            Some("t"),
        ),
        (
            "Lachgasemissionen",
            Some(lang.format_number(f64::from(eq.n2o_emissions))),
            Some("t"),
        ),
        (
            "CH₄ Anlage (unspez.)",
            Some(lang.format_number(f64::from(eq.ch4_plant))),
            Some("t"),
        ),
        (
            "CH₄ Schlupf Schlammlagerung",
            Some(lang.format_number(f64::from(eq.ch4_sludge_storage_containers))),
            Some("t"),
        ),
        (
            "CH₄ Schlupf Schlammtasche",
            Some(lang.format_number(f64::from(eq.ch4_sludge_bags))),
            Some("t"),
        ),
        (
            "CH₄ Gewässer",
            Some(lang.format_number(f64::from(eq.ch4_water))),
            Some("t"),
        ),
        (
            "CH₄ BHKW",
            Some(lang.format_number(f64::from(eq.ch4_combined_heat_and_power_plant))),
            Some("t"),
        ),
        (
            "Methanemissionen",
            Some(lang.format_number(f64::from(eq.ch4_emissions))),
            Some("t"),
        ),
        (
            "Fossile CO₂-Emissionen",
            Some(lang.format_number(f64::from(eq.fossil_emissions))),
            Some("t"),
        ),
        (
            "Eisen(III)-chlorid-Lösung",
            Some(lang.format_number(f64::from(eq.fecl3))),
            Some("t"),
        ),
        (
            "Eisenchloridsulfat-Lösung",
            Some(lang.format_number(f64::from(eq.feclso4))),
            Some("t"),
        ),
        (
            "Kalkhydrat",
            Some(lang.format_number(f64::from(eq.caoh2))),
            Some("t"),
        ),
        (
            "Synthetische Polymere",
            Some(lang.format_number(f64::from(eq.synthetic_polymers))),
            Some("t"),
        ),
        (
            "Strommix",
            Some(lang.format_number(f64::from(eq.electricity_mix))),
            Some("t"),
        ),
        (
            "Heizöl",
            Some(lang.format_number(f64::from(eq.oil_emissions))),
            Some("t"),
        ),
        (
            "Gas",
            Some(lang.format_number(f64::from(eq.gas_emissions))),
            Some("t"),
        ),
        (
            "Betriebsstoffe",
            Some(lang.format_number(f64::from(eq.operating_materials))),
            Some("t"),
        ),
        (
            "Klärschlamm Transport",
            Some(lang.format_number(f64::from(eq.sewage_sludge_transport))),
            Some("t"),
        ),
        (
            "Emission",
            Some(lang.format_number(f64::from(eq.total_emissions))),
            Some("t"),
        ),
        (
            "Direkte Emissionen",
            Some(lang.format_number(f64::from(eq.direct_emissions))),
            Some("t"),
        ),
        (
            "Energieeinsparung bei Prozessen",
            Some(lang.format_number(f64::from(eq.process_energy_savings))),
            Some("t"),
        ),
        (
            "Einsparung durch Photovoltaik",
            Some(lang.format_number(f64::from(eq.photovoltaic_expansion_savings))),
            Some("t"),
        ),
        (
            "Einsparung durch Windkraft",
            Some(lang.format_number(f64::from(eq.wind_expansion_savings))),
            Some("t"),
        ),
        (
            "Einsparung durch Wasserkraft",
            Some(lang.format_number(f64::from(eq.water_expansion_savings))),
            Some("t"),
        ),
        (
            "Einsparung durch Abwärmenutzung",
            Some(lang.format_number(f64::from(eq.district_heating_savings))),
            Some("t"),
        ),
        (
            "Einsparung bei Fossilen Energiequellen",
            Some(lang.format_number(f64::from(eq.fossil_energy_savings))),
            Some("t"),
        ),
        (
            "Indirekte Emissionen",
            Some(lang.format_number(f64::from(eq.indirect_emissions))),
            Some("t"),
        ),
        (
            "Weitere Indirekte Emissionen",
            Some(lang.format_number(f64::from(eq.other_indirect_emissions))),
            Some("t"),
        ),
        (
            "Energiebedingte Emissionen",
            Some(lang.format_number(f64::from(eq.excess_energy_co2_equivalent))),
            Some("t"),
        ),
    ];

    let sections = vec![TableSection {
        title: "CO₂-Emissionen".to_string(),
        rows,
    }];

    Table { sections }
}

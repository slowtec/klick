use std::collections::HashMap;

use serde::Serialize;

use klick_domain::{Id, InputValueId as In, OutputValueId as Out, Value};

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
pub fn plant_profile_as_table(data: &HashMap<Id, Value>, formatting: Formatting) -> Table {
    // TODO: use as parameter
    let lang = Lng::De;

    let sections = [
        (
            "Angaben zur Kläranlage",
            vec![In::PlantName, In::PopulationEquivalent, In::Wastewater],
        ),
        (
            "Zulauf-Parameter (Jahresmittelwerte)",
            vec![
                In::InfluentNitrogen,
                In::InfluentChemicalOxygenDemand,
                In::InfluentTotalOrganicCarbohydrates,
            ],
        ),
        (
            "Ablauf-Parameter (Jahresmittelwerte)",
            vec![In::EffluentNitrogen, In::EffluentChemicalOxygenDemand],
        ),
        (
            "Energiebedarf",
            vec![
                In::TotalPowerConsumption,
                In::OnSitePowerGeneration,
                In::EmissionFactorElectricityMix,
                In::GasSupply,
                In::PurchaseOfBiogas,
                In::HeatingOil,
                In::SewageGasProduced,
                In::MethaneFraction,
            ],
        ),
        (
            "Klärschlammbehandlung",
            vec![
                In::SludgeTreatmentDigesterCount,
                In::SludgeTreatmentDisposal,
                In::SludgeTreatmentTransportDistance,
                In::SludgeTreatmentBagsAreOpen,
                In::SludgeTreatmentStorageContainersAreOpen,
            ],
        ),
        (
            "Prozesswasserbehandlung",
            vec![In::SideStreamTreatmentTotalNitrogen],
        ),
        (
            "Eingesetzte Betriebsstoffe",
            vec![
                In::OperatingMaterialFeCl3,
                In::OperatingMaterialFeClSO4,
                In::OperatingMaterialCaOH2,
                In::OperatingMaterialSyntheticPolymers,
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
                    data.get(&id.into()).as_ref().map(|v| lang.format_value(v)),
                    formatting.fmt(id),
                )
            })
            .collect(),
    })
    .collect();
    Table { sections }
}

#[must_use]
pub fn sensitivity_parameters_as_table(data: &HashMap<Id, Value>, formatting: Formatting) -> Table {
    let lang = Lng::De;

    let sections = vec![
        (
            "Lachgasemissionen",
            vec![
                In::SensitivityN2OCalculationMethod,
                In::SensitivityN2OCustomFactor,
                In::SensitivityN2OSideStreamFactor,
            ],
        ),
        (
            "Methanemissionen aus Blockheizkraftwerken (BHKW)",
            vec![
                In::SensitivityCH4ChpCalculationMethod,
                In::SensitivityCH4ChpCustomFactor,
            ],
        ),
        (
            "Methanemissionen aus offenen Faultürmen und bei der Schlammlagerung",
            vec![
                In::SensitivitySludgeBagsCustomFactor,
                In::SensitivitySludgeStorageCustomFactor,
            ],
        ),
        (
            match formatting {
                Formatting::Text => "Fossile CO₂-Emissionen aus Abwasser",
                Formatting::LaTeX => "Fossile $CO_2$-Emissionen aus Abwasser",
            },
            vec![In::SensitivityCO2FossilCustomFactor],
        ),
    ]
    .into_iter()
    .map(|(title, sections)| TableSection {
        title: title.to_string(),
        rows: sections
            .into_iter()
            .map(|id| {
                (
                    formatting.fmt_label(id),
                    data.get(&id.into()).as_ref().map(|v| lang.format_value(v)),
                    formatting.fmt(id),
                )
            })
            .collect(),
    })
    .collect();
    Table { sections }
}

#[must_use]
pub fn co2_equivalents_as_table(values: &HashMap<Id, Value>, _unit: Formatting) -> Table {
    // TODO: use as parameger
    let lang = Lng::De;

    let emission_data = [
        Out::N2oPlant,
        Out::N2oWater,
        Out::N2oSideStream,
        Out::N2oEmissions,
        Out::Ch4Plant,
        Out::Ch4SludgeStorageContainers,
        Out::Ch4SludgeBags,
        Out::Ch4Water,
        Out::Ch4CombinedHeatAndPowerPlant,
        Out::Ch4Emissions,
        Out::FossilEmissions,
        Out::Fecl3,
        Out::Feclso4,
        Out::Caoh2,
        Out::SyntheticPolymers,
        Out::ElectricityMix,
        Out::OilEmissions,
        Out::GasEmissions,
        Out::OperatingMaterials,
        Out::SewageSludgeTransport,
        Out::TotalEmissions,
        Out::DirectEmissions,
        Out::ProcessEnergySavings,
        Out::PhotovoltaicExpansionSavings,
        Out::WindExpansionSavings,
        Out::WaterExpansionSavings,
        Out::DistrictHeatingSavings,
        Out::FossilEnergySavings,
        Out::IndirectEmissions,
        Out::OtherIndirectEmissions,
        Out::ExcessEnergyCo2Equivalent,
    ];

    // TODO: insert missing values with `Tons::zero()`

    let rows: Vec<_> = emission_data
        .into_iter()
        .filter_map(|id| {
            values
                .get(&id.into())
                .and_then(|value| value.clone().as_tons())
                .map(|tons| (id, tons))
        })
        .map(|(id, value)| {
            let formatted_value = lang.format_number(f64::from(value));
            (id.label(), Some(formatted_value), Some("t"))
        })
        .collect();

    let sections = vec![TableSection {
        title: "CO₂-Emissionen".to_string(),
        rows,
    }];

    Table { sections }
}

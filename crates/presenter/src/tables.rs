use serde::Serialize;
use std::collections::HashMap;

use klick_boundary::FormData;
use klick_domain::{
    self as domain,
    output_value::*,
    units::{Percent, RatioExt, Tons},
    InputValueId as Id, OutputValueId as Out, Value,
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

    let n2o_emission_factor: Option<Value> = output.map(|output| {
        required!(Out::N2oCalculatedEmissionFactor, output.values)
            .unwrap()
            .convert_to::<Percent>()
            .into()
    });

    let ch4_chp_emission_factor: Option<Value> = output.map(|output| {
        required!(Out::Ch4ChpCalculatedEmissionFactor, output.values)
            .unwrap()
            .convert_to::<Percent>()
            .into()
    });

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
pub fn co2_equivalents_as_table(eq: &HashMap<Out, Tons>, _unit: Formatting) -> Table {
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

    let rows: Vec<_> = emission_data
        .into_iter()
        .map(|id| {
            let value = eq.get(&id).copied().unwrap_or_else(Tons::zero); // TODO: log warning if value is None
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

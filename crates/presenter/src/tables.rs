use std::collections::HashMap;

use serde::Serialize;

use klick_domain::{self as domain, Id, InputValueId as In, Value};

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
    pub fn fmt_label<L>(&self, id: L, lang: Lng) -> String
    where
        L: ValueLabel,
    {
        match self {
            Self::Text => id.label(lang),
            Self::LaTeX => id.label_latex(lang),
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
    pub rows: Vec<(String, Option<String>, Option<String>)>,
}

#[must_use]
pub fn plant_profile_as_table(data: &HashMap<Id, Value>, formatting: Formatting) -> Table {
    // TODO: use as parameter
    let lang = Lng::De;

    let custom_emissions: Option<(&str, Vec<_>)> = if data.iter().any(|(id, _)| id.is_custom()) {
        Some((
            "Benutzerdefinierte Emissionen",
            data.iter()
                .filter_map(|(id, _)| {
                    if id.is_custom() {
                        Some(id.clone())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>(),
        ))
    } else {
        None
    };

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
    .map(|(title, ids)| (title, ids.into_iter().map(|id| Id::In(id)).collect()))
    .chain(custom_emissions.into_iter())
    .map(|(title, ids)| TableSection {
        title: title.to_string(),
        rows: ids
            .into_iter()
            .map(|id| {
                (
                    match id {
                        Id::Custom(ref id) => id.clone(),
                        Id::In(id) => formatting.fmt_label(id, lang).to_string(),
                        Id::Out(id) => formatting.fmt_label(id, lang).to_string(),
                    },
                    data.get(&id).as_ref().map(|v| lang.format_value(v)),
                    match id {
                        Id::Custom(_) => Some("t".to_string()), // FIXME @markus
                        Id::In(id) => formatting.fmt(id).map(|id| id.to_string()),
                        Id::Out(id) => formatting.fmt(id).map(|id| id.to_string()),
                    },
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
                    formatting.fmt_label(id, lang).to_string(),
                    data.get(&id.into()).as_ref().map(|v| lang.format_value(v)),
                    formatting.fmt(id).map(Into::into),
                )
            })
            .collect(),
    })
    .collect();
    Table { sections }
}

#[must_use]
pub fn co2_equivalents_as_table(
    (values, graph): &(HashMap<Id, Value>, Vec<(Id, Id)>),
    _unit: Formatting,
) -> Table {
    // TODO: use as parameger
    let lang = Lng::De;

    let ids = domain::emission_group_ids(graph);

    // TODO: insert missing values with `Tons::zero()`

    let rows: Vec<_> = ids
        .into_iter()
        .filter_map(|id| {
            values
                .get(&id)
                .and_then(|value| value.clone().as_tons())
                .map(|tons| (id, tons))
        })
        .map(|(id, value)| {
            let label = match id {
                Id::Custom(id) => id.clone(),
                Id::Out(id) => id.label(lang).to_string(),
                Id::In(id) => id.label(lang).to_string(),
            };
            let formatted_value = lang.format_number(f64::from(value));
            (label, Some(formatted_value), Some("t".to_string()))
        })
        .collect();

    let sections = vec![TableSection {
        title: "CO₂-Emissionen".to_string(),
        rows,
    }];

    Table { sections }
}

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
pub fn plant_profile_as_table(
    data: &HashMap<Id, Value>,
    formatting: Formatting,
    lang: Lng,
) -> Table {
    let custom_emissions: Option<(&str, Vec<_>)> = if data.iter().any(|(id, _)| id.is_custom()) {
        Some((
            {
                match lang {
                    Lng::De => "Benutzerdefinierte Emissionen",
                    Lng::En => "Custom defined emissions",
                }
            },
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
            {
                match lang {
                    Lng::De => "Angaben zur Kläranlage",
                    Lng::En => "Sewage treatment plant details",
                }
            },
            vec![
                In::ProfilePlantName,
                In::ProfilePopulationEquivalent,
                In::ProfileWastewater,
            ],
        ),
        (
            {
                match lang {
                    Lng::De => "Zulauf-Parameter (Jahresmittelwerte)",
                    Lng::En => "Inflow parameters (annual averages)",
                }
            },
            vec![
                In::ProfileInfluentNitrogen,
                In::ProfileInfluentChemicalOxygenDemand,
                In::ProfileInfluentTotalOrganicCarbohydrates,
            ],
        ),
        (
            {
                match lang {
                    Lng::De => "Ablauf-Parameter (Jahresmittelwerte)",
                    Lng::En => "Outflow parameters (annual averages)",
                }
            },
            vec![
                In::ProfileEffluentNitrogen,
                In::ProfileEffluentChemicalOxygenDemand,
            ],
        ),
        (
            {
                match lang {
                    Lng::De => "Energiebedarf",
                    Lng::En => "Energy requirements",
                }
            },
            vec![
                In::ProfileTotalPowerConsumption,
                In::ProfileOnSitePowerGeneration,
                In::ProfileEmissionFactorElectricityMix,
                In::ProfileGasSupply,
                In::ProfilePurchaseOfBiogas,
                In::ProfileHeatingOil,
                In::ProfileSewageGasProduced,
                In::ProfileMethaneFraction,
            ],
        ),
        (
            {
                match lang {
                    Lng::De => "Klärschlammbehandlung",
                    Lng::En => "Sewage sludge treatment",
                }
            },
            vec![
                In::ProfileSludgeDigesterCount,
                In::ProfileSludgeDisposal,
                In::ProfileSludgeTransportDistance,
                In::ProfileSludgeBagsAreOpen,
                In::ProfileSludgeStorageContainersAreOpen,
            ],
        ),
        (
            {
                match lang {
                    Lng::De => "Prozesswasserbehandlung",
                    Lng::En => "Process water treatment",
                }
            },
            vec![In::ProfileSideStreamTotalNitrogen],
        ),
        (
            {
                match lang {
                    Lng::De => "Eingesetzte Betriebsstoffe",
                    Lng::En => "Operating materials used",
                }
            },
            vec![
                In::ProfileOperatingMaterialFeCl3,
                In::ProfileOperatingMaterialFeClSO4,
                In::ProfileOperatingMaterialCaOH2,
                In::ProfileOperatingMaterialSyntheticPolymers,
            ],
        ),
    ]
    .into_iter()
    .map(|(title, ids)| (title, ids.into_iter().map(Id::In).collect()))
    .chain(custom_emissions)
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
                        Id::In(id) => formatting.fmt(id).map(std::string::ToString::to_string),
                        Id::Out(id) => formatting.fmt(id).map(std::string::ToString::to_string),
                    },
                )
            })
            .collect(),
    })
    .collect();

    Table { sections }
}

#[must_use]
pub fn sensitivity_parameters_as_table(
    data: &HashMap<Id, Value>,
    formatting: Formatting,
    lang: Lng,
) -> Table {
    let sections = vec![
        (
            {
                match lang {
                    Lng::De => "Lachgasemissionen",
                    Lng::En => "Nitrous oxide emissions",
                }
            },
            vec![
                In::SensitivityN2OCalculationMethod,
                In::SensitivityN2OCustomFactor,
                In::SensitivityN2OSideStreamFactor,
            ],
        ),
        (
            {
                match lang {
                    Lng::De => "Methanemissionen aus Blockheizkraftwerken (BHKW)",
                    Lng::En => "Methane emissions from combined heat and power plants (CHP)",
                }
            },
            vec![
                In::SensitivityCH4ChpCalculationMethod,
                In::SensitivityCH4ChpCustomFactor,
            ],
        ),
        (
            {
                match lang {
                    Lng::De => {
                        "Methanemissionen aus offenen Faultürmen und bei der Schlammlagerung"
                    }
                    Lng::En => "Methane emissions from open digestion towers and sludge storage",
                }
            },
            vec![
                In::SensitivitySludgeBagsCustomFactor,
                In::SensitivitySludgeStorageCustomFactor,
            ],
        ),
        (
            match formatting {
                Formatting::Text => match lang {
                    Lng::De => "Fossile CO₂-Emissionen aus Abwasser",
                    Lng::En => "Fossil CO₂ emissions from wastewater",
                },
                Formatting::LaTeX => match lang {
                    Lng::De => "Fossile $CO_2$-Emissionen aus Abwasser",
                    Lng::En => "Fossil $CO_2$ emissions from wastewater",
                },
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

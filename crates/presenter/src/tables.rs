use std::collections::HashMap;

use serde::Serialize;

use klick_domain::{InputValueId as In, Value, ValueId as Id};
use klick_interfaces::{
    self as interfaces, TablePresenter as _, ValueGroupId, ValueGroupPresenter as _,
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

pub struct TablePresenter {
    lang: Lng,
    formatting: Formatting,
}

impl interfaces::TablePresenter for TablePresenter {
    fn present_table(
        &self,
        data: HashMap<Id, Value>,
        sections: Vec<(String, Vec<Id>)>,
    ) -> interfaces::Table {
        let sections = sections
            .into_iter()
            .map(|(title, sections)| interfaces::TableSection {
                title,
                rows: sections
                    .into_iter()
                    .map(|id| {
                        let label = self.formatting.fmt_label(id.clone(), self.lang).to_string();
                        let value = {
                            let mut value = data.get(&id).cloned();
                            if value.is_none() {
                                match &id {
                                    Id::In(id) => {
                                        value = id.default_value();
                                    }
                                    Id::Out(id) => {
                                        value = id.default_value();
                                    }
                                    Id::Custom(_) => {}
                                }
                            }
                            value.as_ref().map(|v| self.lang.format_value(v))
                        };
                        let unit = self.formatting.fmt(id.clone()).map(Into::into);

                        interfaces::TableRow {
                            id,
                            label,
                            value,
                            unit,
                        }
                    })
                    .collect(),
            })
            .collect();
        interfaces::Table { sections }
    }
}

pub struct ValueGroupPresenter {
    lang: Lng,
    formatting: Formatting,
}

impl interfaces::ValueGroupPresenter for ValueGroupPresenter {
    fn present_value_group(&self, id: ValueGroupId) -> (String, Vec<Id>) {
        use ValueGroupId as G;

        let (label, ids) = match id {
            G::PlantDetails => (
                match self.lang {
                    Lng::De => "Angaben zur Kläranlage",
                    Lng::En => "Sewage treatment plant details",
                },
                vec![
                    In::ProfilePlantName,
                    In::ProfilePopulationEquivalent,
                    In::ProfileWastewater,
                ],
            ),
            G::InfluentParameters => (
                match self.lang {
                    Lng::De => "Zulauf-Parameter (Jahresmittelwerte)",
                    Lng::En => "Inflow parameters (annual averages)",
                },
                vec![
                    In::ProfileInfluentNitrogen,
                    In::ProfileInfluentChemicalOxygenDemand,
                    In::ProfileInfluentTotalOrganicCarbohydrates,
                ],
            ),
            G::EffluentParameters => (
                match self.lang {
                    Lng::De => "Ablauf-Parameter (Jahresmittelwerte)",
                    Lng::En => "Outflow parameters (annual averages)",
                },
                vec![
                    In::ProfileEffluentNitrogen,
                    In::ProfileEffluentChemicalOxygenDemand,
                ],
            ),
            G::EnergyConsumption => (
                match self.lang {
                    Lng::De => "Energiebedarf",
                    Lng::En => "Energy requirements",
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
            G::SludgeTreatment => (
                match self.lang {
                    Lng::De => "Klärschlammbehandlung",
                    Lng::En => "Sewage sludge treatment",
                },
                vec![
                    In::ProfileSludgeDigesterCount,
                    In::ProfileSludgeDisposal,
                    In::ProfileSludgeTransportDistance,
                    In::ProfileSludgeBagsAreOpen,
                    In::ProfileSludgeStorageContainersAreOpen,
                ],
            ),
            G::SideStreamTreatment => (
                match self.lang {
                    Lng::De => "Prozesswasserbehandlung",
                    Lng::En => "Process water treatment",
                },
                vec![In::ProfileSideStreamTotalNitrogen],
            ),
            G::OperatingMaterials => (
                match self.lang {
                    Lng::De => "Eingesetzte Betriebsstoffe",
                    Lng::En => "Operating materials used",
                },
                vec![
                    In::ProfileOperatingMaterialFeCl3,
                    In::ProfileOperatingMaterialFeClSO4,
                    In::ProfileOperatingMaterialCaOH2,
                    In::ProfileOperatingMaterialSyntheticPolymers,
                ],
            ),
            G::N2OEmissions => (
                match self.lang {
                    Lng::De => "Lachgasemissionen",
                    Lng::En => "Nitrous oxide emissions",
                },
                vec![
                    In::SensitivityN2OCalculationMethod,
                    In::SensitivityN2OCustomFactor,
                    In::SensitivityN2OSideStreamFactor,
                ],
            ),
            G::CH4ChpEmissions => (
                match self.lang {
                    Lng::De => "Methanemissionen aus Blockheizkraftwerken (BHKW)",
                    Lng::En => "Methane emissions from combined heat and power plants (CHP)",
                },
                vec![
                    In::SensitivityCH4ChpCalculationMethod,
                    In::SensitivityCH4ChpCustomFactor,
                ],
            ),
            G::CH4SludgeEmissions => (
                match self.lang {
                    Lng::De => {
                        "Methanemissionen aus offenen Faultürmen und bei der Schlammlagerung"
                    }
                    Lng::En => "Methane emissions from open digestion towers and sludge storage",
                },
                vec![
                    In::SensitivitySludgeBagsCustomFactor,
                    In::SensitivitySludgeStorageCustomFactor,
                ],
            ),
            G::FossilCO2Emissions => (
                match self.formatting {
                    Formatting::Text => match self.lang {
                        Lng::De => "Fossile CO₂-Emissionen aus Abwasser",
                        Lng::En => "Fossil CO₂ emissions from wastewater",
                    },
                    Formatting::LaTeX => match self.lang {
                        Lng::De => "Fossile $CO_2$-Emissionen aus Abwasser",
                        Lng::En => "Fossil $CO_2$ emissions from wastewater",
                    },
                },
                vec![In::SensitivityCO2FossilCustomFactor],
            ),
        };
        (label.to_string(), ids.into_iter().map(Id::from).collect())
    }
}

// TODO: #[deprecated]
#[derive(Default, Serialize)]
pub struct Table {
    pub sections: Vec<TableSection>,
}

// TODO: #[deprecated]
#[derive(Serialize)]
pub struct TableSection {
    pub title: String,
    pub rows: Vec<TableRow>,
}

// TODO: #[deprecated]
#[derive(Serialize)]
pub struct TableRow {
    #[serde(skip_serializing)]
    pub id: Id,
    pub label: String,
    pub value: Option<String>,
    pub unit: Option<String>,
}

impl From<interfaces::Table> for Table {
    fn from(from: interfaces::Table) -> Self {
        let sections = from.sections.into_iter().map(Into::into).collect();
        Self { sections }
    }
}

impl From<interfaces::TableRow> for TableRow {
    fn from(from: interfaces::TableRow) -> Self {
        let interfaces::TableRow {
            id,
            label,
            value,
            unit,
        } = from;
        Self {
            id,
            label,
            value,
            unit,
        }
    }
}

impl From<interfaces::TableSection> for TableSection {
    fn from(from: interfaces::TableSection) -> Self {
        let interfaces::TableSection { title, rows } = from;
        let rows = rows.into_iter().map(Into::into).collect();

        Self { title, rows }
    }
}

#[must_use]
pub fn plant_profile_as_table(
    data: &HashMap<Id, Value>,
    formatting: Formatting,
    lang: Lng,
) -> Table {
    let custom_emissions: Option<(_, Vec<_>)> = if data.iter().any(|(id, _)| id.is_custom()) {
        Some((
            {
                match lang {
                    Lng::De => "Benutzerdefinierte Emissionen",
                    Lng::En => "Custom defined emissions",
                }
                .to_string()
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
    let presenter = ValueGroupPresenter { lang, formatting };
    let sections = [
        ValueGroupId::PlantDetails,
        ValueGroupId::InfluentParameters,
        ValueGroupId::EffluentParameters,
        ValueGroupId::EnergyConsumption,
        ValueGroupId::SludgeTreatment,
        ValueGroupId::SideStreamTreatment,
        ValueGroupId::OperatingMaterials,
    ]
    .into_iter()
    .map(|id| presenter.present_value_group(id))
    .chain(custom_emissions)
    .collect();

    let table_presenter = TablePresenter { lang, formatting };
    let table: interfaces::Table = table_presenter.present_table(data.clone(), sections);
    Table::from(table)
}

#[must_use]
pub fn sensitivity_parameters_as_table(
    data: &HashMap<Id, Value>,
    formatting: Formatting,
    lang: Lng,
) -> Table {
    let presenter = ValueGroupPresenter { lang, formatting };
    let sections = [
        ValueGroupId::N2OEmissions,
        ValueGroupId::CH4ChpEmissions,
        ValueGroupId::CH4SludgeEmissions,
        ValueGroupId::FossilCO2Emissions,
    ]
    .into_iter()
    .map(|id| presenter.present_value_group(id))
    .collect();

    let table_presenter = TablePresenter { lang, formatting };
    let table: interfaces::Table = table_presenter.present_table(data.clone(), sections);
    Table::from(table)
}

#[must_use]
pub fn co2_equivalents_as_table(
    (values, graph): &(HashMap<Id, Value>, Vec<(Id, Id)>),
    _unit: Formatting,
) -> Table {
    // TODO: use as parameger
    let lang = Lng::De;

    let ids = klick_usecases::emission_group_ids(graph);

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
            let label = match &id {
                Id::Custom(id) => id.clone(),
                Id::Out(id) => id.label(lang).to_string(),
                Id::In(id) => id.label(lang).to_string(),
            };
            let value = Some(lang.format_number(f64::from(value)));
            let unit = Some("t".to_string());
            TableRow {
                id,
                label,
                value,
                unit,
            }
        })
        .collect();

    let sections = vec![TableSection {
        title: "CO₂-Emissionen".to_string(),
        rows,
    }];

    Table { sections }
}

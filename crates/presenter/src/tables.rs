use serde::Serialize;

use klick_boundary::FormData;
use klick_domain::{
    self as domain,
    units::{Percent, RatioExt},
    InputValueId as Id, Value,
};

use crate::{InputValueId, Lng, ValueLabel, ValueUnit};

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

    let sections = vec![
        TableSection {
            title: "Angaben zur Kläranlage".to_string(),
            rows: vec![
                {
                    let id = Id::PlantName;
                    (
                        id.label(),
                        data.get(&id).map(Value::as_text_unchecked).clone(),
                        formatting.fmt(id),
                    )
                },
                {
                    let id = Id::PopulationEquivalent;
                    (
                        id.label(),
                        data.get(&id)
                            .map(Value::as_count_unchecked)
                            .map(|v| u64::from(v) as f64)
                            .map(format_number_with_thousands_seperator(lang)),
                        formatting.fmt(id),
                    )
                },
                {
                    let id = Id::Wastewater;
                    (
                        id.label(),
                        data.get(&id)
                            .map(Value::as_qubicmeters_unchecked)
                            .map(f64::from)
                            .map(format_number_with_thousands_seperator(lang)),
                        formatting.fmt(id),
                    )
                },
            ],
        },
        TableSection {
            title: "Zulauf-Parameter (Jahresmittelwerte)".to_string(),
            rows: [
                Id::InfluentNitrogen,
                Id::InfluentChemicalOxygenDemand,
                Id::InfluentTotalOrganicCarbohydrates,
            ]
            .into_iter()
            .map(|id| {
                (
                    id.label(),
                    data.get(&id)
                        .map(Value::as_milligrams_per_liter_unchecked)
                        .map(f64::from)
                        .map(format_number(lang)),
                    formatting.fmt(id),
                )
            })
            .collect(),
        },
        TableSection {
            title: "Ablauf-Parameter (Jahresmittelwerte)".to_string(),
            rows: [Id::EffluentNitrogen, Id::EffluentChemicalOxygenDemand]
                .into_iter()
                .map(|id| {
                    (
                        id.label(),
                        data.get(&id)
                            .map(Value::as_milligrams_per_liter_unchecked)
                            .map(f64::from)
                            .map(format_number(lang)),
                        formatting.fmt(id),
                    )
                })
                .collect(),
        },
        TableSection {
            title: "Energiebedarf".to_string(),
            rows: vec![
                (
                    InputValueId::TotalPowerConsumption.label(),
                    data.get(&InputValueId::TotalPowerConsumption)
                        .map(Value::as_kilowatthours_unchecked)
                        .map(f64::from)
                        .map(format_number_with_thousands_seperator(lang)),
                    formatting.fmt(InputValueId::TotalPowerConsumption),
                ),
                (
                    InputValueId::OnSitePowerGeneration.label(),
                    data.get(&InputValueId::OnSitePowerGeneration)
                        .map(Value::as_kilowatthours_unchecked)
                        .map(f64::from)
                        .map(format_number_with_thousands_seperator(lang)),
                    formatting.fmt(InputValueId::OnSitePowerGeneration),
                ),
                (
                    InputValueId::EmissionFactorElectricityMix.label(),
                    data.get(&InputValueId::EmissionFactorElectricityMix)
                        .map(Value::as_grams_per_kilowatthour_unchecked)
                        .map(f64::from)
                        .map(format_number(lang)),
                    formatting.fmt(InputValueId::EmissionFactorElectricityMix),
                ),
                (
                    InputValueId::GasSupply.label(),
                    data.get(&InputValueId::GasSupply)
                        .map(Value::as_qubicmeters_unchecked)
                        .map(f64::from)
                        .map(format_number(lang)),
                    formatting.fmt(InputValueId::GasSupply),
                ),
                (
                    InputValueId::PurchaseOfBiogas.label(),
                    data.get(&InputValueId::PurchaseOfBiogas)
                        .map(Value::as_bool_unchecked)
                        .map(format_bool(lang)),
                    formatting.fmt(InputValueId::PurchaseOfBiogas),
                ),
                (
                    InputValueId::HeatingOil.label(),
                    data.get(&InputValueId::HeatingOil)
                        .map(Value::as_liters_unchecked)
                        .map(f64::from)
                        .map(format_number_with_thousands_seperator(lang)),
                    formatting.fmt(InputValueId::HeatingOil),
                ),
                (
                    InputValueId::SewageGasProduced.label(),
                    data.get(&InputValueId::SewageGasProduced)
                        .map(Value::as_qubicmeters_unchecked)
                        .map(f64::from)
                        .map(format_number_with_thousands_seperator(lang)),
                    formatting.fmt(InputValueId::SewageGasProduced),
                ),
                (
                    InputValueId::MethaneFraction.label(),
                    data.get(&InputValueId::MethaneFraction)
                        .map(Value::as_percent_unchecked)
                        .map(f64::from)
                        .map(format_number(lang)),
                    formatting.fmt(InputValueId::MethaneFraction),
                ),
            ],
        },
        TableSection {
            title: "Klärschlammbehandlung".to_string(),
            rows: vec![
                (
                    InputValueId::SludgeTreatmentDigesterCount.label(),
                    data.get(&InputValueId::SludgeTreatmentDigesterCount)
                        .map(Value::as_count_unchecked)
                        .map(u64::from)
                        .map(|n| n.to_string()),
                    formatting.fmt(InputValueId::SludgeTreatmentDigesterCount),
                ),
                (
                    InputValueId::SludgeTreatmentDisposal.label(),
                    data.get(&InputValueId::SludgeTreatmentDisposal)
                        .map(Value::as_tons_unchecked)
                        .map(f64::from)
                        .map(format_number(lang)),
                    formatting.fmt(InputValueId::SludgeTreatmentDisposal),
                ),
                (
                    InputValueId::SludgeTreatmentTransportDistance.label(),
                    data.get(&InputValueId::SludgeTreatmentTransportDistance)
                        .map(Value::as_kilometers_unchecked)
                        .map(f64::from)
                        .map(format_number(lang)),
                    formatting.fmt(InputValueId::SludgeTreatmentTransportDistance),
                ),
                (
                    InputValueId::SludgeTreatmentBagsAreOpen.label(),
                    data.get(&InputValueId::SludgeTreatmentBagsAreOpen)
                        .map(Value::as_bool_unchecked)
                        .map(format_bool(lang)),
                    formatting.fmt(InputValueId::SludgeTreatmentBagsAreOpen),
                ),
                (
                    InputValueId::SludgeTreatmentStorageContainersAreOpen.label(),
                    data.get(&InputValueId::SludgeTreatmentStorageContainersAreOpen)
                        .map(Value::as_bool_unchecked)
                        .map(format_bool(lang)),
                    formatting.fmt(InputValueId::SludgeTreatmentStorageContainersAreOpen),
                ),
            ],
        },
        TableSection {
            title: "Prozesswasserbehandlung".to_string(),
            rows: vec![(
                InputValueId::SideStreamTreatmentTotalNitrogen.label(),
                data.get(&InputValueId::SideStreamTreatmentTotalNitrogen)
                    .map(Value::as_tons_unchecked)
                    .map(f64::from)
                    .map(format_number(lang)),
                formatting.fmt(InputValueId::SideStreamTreatmentTotalNitrogen),
            )],
        },
        TableSection {
            title: "Eingesetzte Betriebsstoffe".to_string(),
            rows: [
                Id::OperatingMaterialFeCl3,
                Id::OperatingMaterialFeClSO4,
                Id::OperatingMaterialCaOH2,
                Id::OperatingMaterialSyntheticPolymers,
            ]
            .into_iter()
            .map(|id| {
                (
                    id.label(),
                    data.get(&id)
                        .map(Value::as_tons_unchecked)
                        .map(f64::from)
                        .map(format_number(lang)),
                    formatting.fmt(id),
                )
            })
            .collect(),
        },
    ];
    Table { sections }
}

#[must_use]
pub fn sensitivity_parameters_as_table(
    data: &FormData,
    formatting: Formatting,
    output: Option<&domain::EmissionsCalculationOutcome>,
) -> Table {
    let lang = Lng::De;

    let n2o_emission_factor = output.map(|output| {
        lang.format_number_with_precision(
            f64::from(output.emission_factors.n2o.convert_to::<Percent>()),
            3,
        )
    });

    let ch4_chp_emission_factor = output.map(|output| {
        lang.format_number(f64::from(
            output.emission_factors.ch4.convert_to::<Percent>(),
        ))
    });

    let sections = vec![
        TableSection {
            title: "Lachgasemissionen".to_string(),
            rows: vec![
                (
                    formatting.fmt_label(InputValueId::SensitivityN2OCalculationMethod),
                    data.get(&InputValueId::SensitivityN2OCalculationMethod)
                        .map(Value::as_n2o_emission_factor_calc_method_unchecked)
                        .map(|m| m.label().to_string()),
                    formatting.fmt(InputValueId::SensitivityN2OCalculationMethod),
                ),
                (
                    match formatting {
                        Formatting::Text => "N₂O-EF",
                        Formatting::LaTeX => "$N_2O$-EF",
                    },
                    n2o_emission_factor,
                    formatting.fmt(InputValueId::SensitivityN2OCustomFactor),
                ),
                (
                    formatting.fmt_label(InputValueId::SensitivityN2OSideStreamFactor),
                    data.get(&Id::ScenarioN2OSideStreamFactor)
                        .map(Value::as_factor_unchecked)
                        .map(f64::from)
                        .map(format_number(lang)),
                    formatting.fmt(InputValueId::SensitivityN2OSideStreamFactor),
                ),
            ],
        },
        TableSection {
            title: "Methanemissionen aus Blockheizkraftwerken (BHKW)".to_string(),
            rows: vec![
                (
                    formatting.fmt_label(InputValueId::SensitivityCH4ChpCalculationMethod),
                    data.get(&Id::SensitivityCH4ChpCalculationMethod)
                        .map(Value::as_ch4_chp_emission_factor_calc_method_unchecked)
                        .map(|m| m.label().to_string()),
                    formatting.fmt(InputValueId::SensitivityCH4ChpCalculationMethod),
                ),
                (
                    match formatting {
                        Formatting::Text => "BHKW CH₄-EF",
                        Formatting::LaTeX => "BHKW $CH_4$-EF",
                    },
                    ch4_chp_emission_factor,
                    formatting.fmt(InputValueId::SensitivityCH4ChpCustomFactor),
                ),
            ],
        },
        TableSection {
            title: "Methanemissionen aus offenen Faultürmen und bei der Schlammlagerung"
                .to_string(),
            rows: vec![
                (
                    formatting.fmt_label(InputValueId::SensitivitySludgeBagsCustomFactor),
                    data.get(&InputValueId::SensitivitySludgeBagsCustomFactor)
                        .map(Value::as_factor_unchecked)
                        .map(|f| f.convert_to::<Percent>())
                        .map(f64::from)
                        .map(format_number(lang)),
                    formatting.fmt(InputValueId::SensitivitySludgeBagsCustomFactor),
                ),
                (
                    formatting.fmt_label(InputValueId::SensitivitySludgeStorageCustomFactor),
                    data.get(&InputValueId::SensitivitySludgeStorageCustomFactor)
                        .map(Value::as_factor_unchecked)
                        .map(|f| f.convert_to::<Percent>())
                        .map(f64::from)
                        .map(format_number(lang)),
                    formatting.fmt(InputValueId::SensitivitySludgeStorageCustomFactor),
                ),
            ],
        },
        TableSection {
            title: match formatting {
                Formatting::Text => "Fossile CO₂-Emissionen aus Abwasser",
                Formatting::LaTeX => "Fossile $CO_2$-Emissionen aus Abwasser",
            }
            .to_string(),
            rows: vec![(
                formatting.fmt_label(InputValueId::SensitivityCO2FossilCustomFactor),
                data.get(&InputValueId::SensitivityCO2FossilCustomFactor)
                    .map(Value::as_factor_unchecked)
                    .map(|f| f.convert_to::<Percent>())
                    .map(f64::from)
                    .map(format_number(lang)),
                formatting.fmt(InputValueId::SensitivityCO2FossilCustomFactor),
            )],
        },
    ];
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

fn format_number(lang: Lng) -> impl Fn(f64) -> String {
    move |n| lang.format_number(n)
}

fn format_number_with_thousands_seperator(lang: Lng) -> impl Fn(f64) -> String {
    move |n| lang.format_number_with_thousands_seperator(n)
}

fn format_bool(lang: Lng) -> impl Fn(bool) -> String {
    move |x| lang.format_bool(x).to_string()
}

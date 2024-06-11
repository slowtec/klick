use serde::Serialize;

use klick_boundary::{FormData, PlantProfile, SensitivityParameters};
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
    let PlantProfile {
        wastewater,
        influent_average,
        effluent_average,
        energy_consumption,
        sewage_sludge_treatment,
        side_stream_treatment,
        operating_materials,
        ..
    } = &data.plant_profile;

    // TODO: use as parameter
    let lang = Lng::De;

    let sections = vec![
        TableSection {
            title: "Angaben zur Kläranlage".to_string(),
            rows: vec![
                (
                    InputValueId::PlantName.label(),
                    data.get(&Id::PlantName)
                        .map(Value::as_text_unchecked)
                        .clone(),
                    formatting.fmt(InputValueId::PlantName),
                ),
                (
                    InputValueId::PopulationEquivalent.label(),
                    data.get(&Id::PopulationEquivalent)
                        .map(Value::as_count_unchecked)
                        .map(|v| u64::from(v) as f64)
                        .map(format_number_with_thousands_seperator(lang)),
                    formatting.fmt(InputValueId::PopulationEquivalent),
                ),
                (
                    InputValueId::Wastewater.label(),
                    wastewater.map(format_number_with_thousands_seperator(lang)),
                    formatting.fmt(InputValueId::Wastewater),
                ),
            ],
        },
        TableSection {
            title: "Zulauf-Parameter (Jahresmittelwerte)".to_string(),
            rows: vec![
                (
                    InputValueId::InfluentNitrogen.label(),
                    influent_average.total_nitrogen.map(format_number(lang)),
                    formatting.fmt(InputValueId::InfluentNitrogen),
                ),
                (
                    InputValueId::InfluentChemicalOxygenDemand.label(),
                    influent_average
                        .chemical_oxygen_demand
                        .map(format_number(lang)),
                    formatting.fmt(InputValueId::InfluentChemicalOxygenDemand),
                ),
                (
                    InputValueId::InfluentTotalOrganicCarbohydrates.label(),
                    influent_average
                        .total_organic_carbohydrates
                        .map(format_number(lang)),
                    formatting.fmt(InputValueId::InfluentTotalOrganicCarbohydrates),
                ),
            ],
        },
        TableSection {
            title: "Ablauf-Parameter (Jahresmittelwerte)".to_string(),
            rows: vec![
                (
                    InputValueId::EffluentNitrogen.label(),
                    effluent_average.total_nitrogen.map(format_number(lang)),
                    formatting.fmt(InputValueId::EffluentNitrogen),
                ),
                (
                    InputValueId::EffluentChemicalOxygenDemand.label(),
                    effluent_average
                        .chemical_oxygen_demand
                        .map(format_number(lang)),
                    formatting.fmt(InputValueId::EffluentChemicalOxygenDemand),
                ),
            ],
        },
        TableSection {
            title: "Energiebedarf".to_string(),
            rows: vec![
                (
                    InputValueId::TotalPowerConsumption.label(),
                    energy_consumption
                        .total_power_consumption
                        .map(format_number_with_thousands_seperator(lang)),
                    formatting.fmt(InputValueId::TotalPowerConsumption),
                ),
                (
                    InputValueId::OnSitePowerGeneration.label(),
                    energy_consumption
                        .on_site_power_generation
                        .map(format_number_with_thousands_seperator(lang)),
                    formatting.fmt(InputValueId::OnSitePowerGeneration),
                ),
                (
                    InputValueId::EmissionFactorElectricityMix.label(),
                    energy_consumption
                        .emission_factor_electricity_mix
                        .map(format_number(lang)),
                    formatting.fmt(InputValueId::EmissionFactorElectricityMix),
                ),
                (
                    InputValueId::GasSupply.label(),
                    energy_consumption.gas_supply.map(format_number(lang)),
                    formatting.fmt(InputValueId::GasSupply),
                ),
                (
                    InputValueId::PurchaseOfBiogas.label(),
                    energy_consumption.purchase_of_biogas.map(format_bool(lang)),
                    formatting.fmt(InputValueId::PurchaseOfBiogas),
                ),
                (
                    InputValueId::HeatingOil.label(),
                    energy_consumption
                        .heating_oil
                        .map(format_number_with_thousands_seperator(lang)),
                    formatting.fmt(InputValueId::HeatingOil),
                ),
                (
                    InputValueId::SewageGasProduced.label(),
                    energy_consumption
                        .sewage_gas_produced
                        .map(format_number_with_thousands_seperator(lang)),
                    formatting.fmt(InputValueId::SewageGasProduced),
                ),
                (
                    InputValueId::MethaneFraction.label(),
                    energy_consumption.methane_fraction.map(format_number(lang)),
                    formatting.fmt(InputValueId::MethaneFraction),
                ),
            ],
        },
        TableSection {
            title: "Klärschlammbehandlung".to_string(),
            rows: vec![
                (
                    InputValueId::SludgeTreatmentDigesterCount.label(),
                    sewage_sludge_treatment
                        .digester_count
                        .map(|n| n.to_string()),
                    formatting.fmt(InputValueId::SludgeTreatmentDigesterCount),
                ),
                (
                    InputValueId::SludgeTreatmentDisposal.label(),
                    sewage_sludge_treatment
                        .sewage_sludge_for_disposal
                        .map(format_number(lang)),
                    formatting.fmt(InputValueId::SludgeTreatmentDisposal),
                ),
                (
                    InputValueId::SludgeTreatmentTransportDistance.label(),
                    sewage_sludge_treatment
                        .transport_distance
                        .map(format_number(lang)),
                    formatting.fmt(InputValueId::SludgeTreatmentTransportDistance),
                ),
                (
                    InputValueId::SludgeTreatmentBagsAreOpen.label(),
                    sewage_sludge_treatment
                        .sludge_bags_are_closed
                        .map(|v| !v) // closed => open
                        .map(format_bool(lang)),
                    formatting.fmt(InputValueId::SludgeTreatmentBagsAreOpen),
                ),
                (
                    InputValueId::SludgeTreatmentStorageContainersAreOpen.label(),
                    sewage_sludge_treatment
                        .sludge_storage_containers_are_closed
                        .map(|v| !v) // closed => open
                        .map(format_bool(lang)),
                    formatting.fmt(InputValueId::SludgeTreatmentStorageContainersAreOpen),
                ),
            ],
        },
        TableSection {
            title: "Prozesswasserbehandlung".to_string(),
            rows: vec![(
                InputValueId::SideStreamTreatmentTotalNitrogen.label(),
                side_stream_treatment
                    .total_nitrogen
                    .map(format_number(lang)),
                formatting.fmt(InputValueId::SideStreamTreatmentTotalNitrogen),
            )],
        },
        TableSection {
            title: "Eingesetzte Betriebsstoffe".to_string(),
            rows: vec![
                (
                    InputValueId::OperatingMaterialFeCl3.label(),
                    operating_materials.fecl3.map(format_number(lang)),
                    formatting.fmt(InputValueId::OperatingMaterialFeCl3),
                ),
                (
                    InputValueId::OperatingMaterialFeClSO4.label(),
                    operating_materials.feclso4.map(format_number(lang)),
                    formatting.fmt(InputValueId::OperatingMaterialFeClSO4),
                ),
                (
                    InputValueId::OperatingMaterialCaOH2.label(),
                    operating_materials.caoh2.map(format_number(lang)),
                    formatting.fmt(InputValueId::OperatingMaterialCaOH2),
                ),
                (
                    InputValueId::OperatingMaterialSyntheticPolymers.label(),
                    operating_materials
                        .synthetic_polymers
                        .map(format_number(lang)),
                    formatting.fmt(InputValueId::OperatingMaterialSyntheticPolymers),
                ),
            ],
        },
    ];
    Table { sections }
}

#[must_use]
pub fn sensitivity_parameters_as_table(
    parameters: &SensitivityParameters,
    formatting: Formatting,
    output: Option<&domain::EmissionsCalculationOutcome>,
) -> Table {
    let SensitivityParameters {
        n2o_emissions,
        ch4_chp_emissions,
        ch4_sewage_sludge_emissions,
        co2_fossil_emissions,
    } = parameters;

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
                    n2o_emissions
                        .calculation_method
                        .as_ref()
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
                    n2o_emissions
                        .side_stream_emission_factor
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
                    ch4_chp_emissions
                        .calculation_method
                        .as_ref()
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
                    ch4_sewage_sludge_emissions
                        .emission_factor_sludge_bags
                        .map(format_number(lang)),
                    formatting.fmt(InputValueId::SensitivitySludgeBagsCustomFactor),
                ),
                (
                    formatting.fmt_label(InputValueId::SensitivitySludgeStorageCustomFactor),
                    ch4_sewage_sludge_emissions
                        .emission_factor_sludge_storage_containers
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
                co2_fossil_emissions
                    .emission_factor
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

use serde::Serialize;

use klick_boundary::{PlantProfile, SensitivityParameters};
use klick_domain as domain;
use klick_domain::units::Percent;
use klick_domain::units::Ratio;

use crate::{
    AnnualAverageEffluentId, AnnualAverageInfluentId, EnergyConsumptionId, Lng,
    OperatingMaterialId, ProfileValueId, SensitivityParameterId, SewageSludgeTreatmentId,
    SideStreamTreatmentId, ValueLabel, ValueUnit,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnitFormatting {
    Text,
    LaTeX,
}

impl UnitFormatting {
    fn fmt<V>(&self, v: V) -> Option<&'static str>
    where
        V: ValueUnit,
    {
        match self {
            Self::Text => v.unit_as_text(),
            Self::LaTeX => v.unit_as_latex(),
        }
    }
}

#[derive(Serialize)]
pub struct Table {
    pub sections: Vec<TableSection>,
}

#[derive(Serialize)]
pub struct TableSection {
    pub title: String,
    pub rows: Vec<(&'static str, Option<String>, Option<&'static str>)>,
}

pub fn plant_profile_as_table(profile: &PlantProfile, unit: UnitFormatting) -> Table {
    let PlantProfile {
        plant_name,
        population_equivalent,
        wastewater,
        influent_average,
        effluent_average,
        energy_consumption,
        sewage_sludge_treatment,
        side_stream_treatment,
        operating_materials,
    } = profile;

    // TODO: use as parameter
    let lang = Lng::De;

    let sections = vec![
        TableSection {
            title: "Angaben zur Kläranlage".to_string(),
            rows: vec![
                (
                    ProfileValueId::PlantName.label(),
                    plant_name.clone(),
                    unit.fmt(ProfileValueId::PlantName),
                ),
                (
                    ProfileValueId::PopulationEquivalent.label(),
                    population_equivalent.map(format_number_with_thousands_seperator(lang)),
                    unit.fmt(ProfileValueId::PopulationEquivalent),
                ),
                (
                    ProfileValueId::Wastewater.label(),
                    wastewater.map(format_number_with_thousands_seperator(lang)),
                    unit.fmt(ProfileValueId::Wastewater),
                ),
            ],
        },
        TableSection {
            title: "Zulauf-Parameter (Jahresmittelwerte)".to_string(),
            rows: vec![
                (
                    AnnualAverageInfluentId::Nitrogen.label(),
                    influent_average.total_nitrogen.map(format_number(lang)),
                    unit.fmt(AnnualAverageInfluentId::Nitrogen),
                ),
                (
                    AnnualAverageInfluentId::ChemicalOxygenDemand.label(),
                    influent_average
                        .chemical_oxygen_demand
                        .map(format_number(lang)),
                    unit.fmt(AnnualAverageInfluentId::ChemicalOxygenDemand),
                ),
                (
                    AnnualAverageInfluentId::TotalOrganicCarbohydrates.label(),
                    influent_average
                        .total_organic_carbohydrates
                        .map(format_number(lang)),
                    unit.fmt(AnnualAverageInfluentId::TotalOrganicCarbohydrates),
                ),
            ],
        },
        TableSection {
            title: "Ablauf-Parameter (Jahresmittelwerte)".to_string(),
            rows: vec![
                (
                    AnnualAverageEffluentId::Nitrogen.label(),
                    effluent_average.total_nitrogen.map(format_number(lang)),
                    unit.fmt(AnnualAverageEffluentId::Nitrogen),
                ),
                (
                    AnnualAverageEffluentId::ChemicalOxygenDemand.label(),
                    effluent_average
                        .chemical_oxygen_demand
                        .map(format_number(lang)),
                    unit.fmt(AnnualAverageEffluentId::ChemicalOxygenDemand),
                ),
            ],
        },
        TableSection {
            title: "Energiebedarf".to_string(),
            rows: vec![
                (
                    EnergyConsumptionId::TotalPowerConsumption.label(),
                    energy_consumption
                        .total_power_consumption
                        .map(format_number_with_thousands_seperator(lang)),
                    unit.fmt(EnergyConsumptionId::TotalPowerConsumption),
                ),
                (
                    EnergyConsumptionId::OnSitePowerGeneration.label(),
                    energy_consumption
                        .on_site_power_generation
                        .map(format_number_with_thousands_seperator(lang)),
                    unit.fmt(EnergyConsumptionId::OnSitePowerGeneration),
                ),
                (
                    EnergyConsumptionId::EmissionFactorElectricityMix.label(),
                    energy_consumption
                        .emission_factor_electricity_mix
                        .map(format_number(lang)),
                    unit.fmt(EnergyConsumptionId::EmissionFactorElectricityMix),
                ),
                (
                    EnergyConsumptionId::GasSupply.label(),
                    energy_consumption.gas_supply.map(format_number(lang)),
                    unit.fmt(EnergyConsumptionId::GasSupply),
                ),
                (
                    EnergyConsumptionId::PurchaseOfBiogas.label(),
                    energy_consumption.purchase_of_biogas.map(format_bool(lang)),
                    unit.fmt(EnergyConsumptionId::PurchaseOfBiogas),
                ),
                (
                    EnergyConsumptionId::HeatingOil.label(),
                    energy_consumption
                        .heating_oil
                        .map(format_number_with_thousands_seperator(lang)),
                    unit.fmt(EnergyConsumptionId::HeatingOil),
                ),
                (
                    EnergyConsumptionId::SewageGasProduced.label(),
                    energy_consumption
                        .sewage_gas_produced
                        .map(format_number_with_thousands_seperator(lang)),
                    unit.fmt(EnergyConsumptionId::SewageGasProduced),
                ),
                (
                    EnergyConsumptionId::MethaneFraction.label(),
                    energy_consumption.methane_fraction.map(format_number(lang)),
                    unit.fmt(EnergyConsumptionId::MethaneFraction),
                ),
            ],
        },
        TableSection {
            title: "Klärschlammbehandlung".to_string(),
            rows: vec![
                (
                    SewageSludgeTreatmentId::DigesterCount.label(),
                    sewage_sludge_treatment
                        .digester_count
                        .map(|n| n.to_string()),
                    unit.fmt(SewageSludgeTreatmentId::DigesterCount),
                ),
                (
                    SewageSludgeTreatmentId::SewageSludgeForDisposal.label(),
                    sewage_sludge_treatment
                        .sewage_sludge_for_disposal
                        .map(format_number(lang)),
                    unit.fmt(SewageSludgeTreatmentId::SewageSludgeForDisposal),
                ),
                (
                    SewageSludgeTreatmentId::TransportDistance.label(),
                    sewage_sludge_treatment
                        .transport_distance
                        .map(format_number(lang)),
                    unit.fmt(SewageSludgeTreatmentId::TransportDistance),
                ),
                (
                    SewageSludgeTreatmentId::SludgeBags.label(),
                    sewage_sludge_treatment
                        .sludge_bags_are_closed
                        .map(format_bool(lang)),
                    unit.fmt(SewageSludgeTreatmentId::SludgeBags),
                ),
                (
                    SewageSludgeTreatmentId::SludgeStorageContainers.label(),
                    sewage_sludge_treatment
                        .sludge_storage_containers_are_closed
                        .map(format_bool(lang)),
                    unit.fmt(SewageSludgeTreatmentId::SludgeStorageContainers),
                ),
            ],
        },
        TableSection {
            title: "Prozesswasserbehandlung".to_string(),
            rows: vec![(
                SideStreamTreatmentId::TotalNitrogen.label(),
                side_stream_treatment
                    .total_nitrogen
                    .map(format_number(lang)),
                unit.fmt(SideStreamTreatmentId::TotalNitrogen),
            )],
        },
        TableSection {
            title: "Eingesetzte Betriebsstoffe".to_string(),
            rows: vec![
                (
                    OperatingMaterialId::FeCl3.label(),
                    operating_materials.fecl3.map(format_number(lang)),
                    unit.fmt(OperatingMaterialId::FeCl3),
                ),
                (
                    OperatingMaterialId::FeClSO4.label(),
                    operating_materials.feclso4.map(format_number(lang)),
                    unit.fmt(OperatingMaterialId::FeClSO4),
                ),
                (
                    OperatingMaterialId::CaOH2.label(),
                    operating_materials.caoh2.map(format_number(lang)),
                    unit.fmt(OperatingMaterialId::CaOH2),
                ),
                (
                    OperatingMaterialId::SyntheticPolymers.label(),
                    operating_materials
                        .synthetic_polymers
                        .map(format_number(lang)),
                    unit.fmt(OperatingMaterialId::SyntheticPolymers),
                ),
            ],
        },
    ];
    Table { sections }
}

pub fn sensitivity_parameters_as_table(
    parameters: &SensitivityParameters,
    unit: UnitFormatting,
    o: &domain::EmissionsCalculationOutcome,
) -> Table {
    let SensitivityParameters {
        n2o_emissions,
        ch4_chp_emissions,
        ch4_sewage_sludge_emissions,
        co2_fossil_emissions,
    } = parameters;

    let lang = Lng::De;

    let n2o_emission_factor: String = lang.format_number_with_precision(
        f64::from(o.emission_factors.n2o.convert_to::<Percent>()),
        3,
    );
    let ch4_chp_emission_factor: String =
        lang.format_number(f64::from(o.emission_factors.ch4.convert_to::<Percent>()));

    let sections = vec![
        TableSection {
            title: "Lachgasemissionen".to_string(),
            rows: vec![
                (
                    SensitivityParameterId::N2OCalculationMethod.label(),
                    n2o_emissions
                        .calculation_method
                        .as_ref()
                        .map(|m| m.label().to_string()),
                    unit.fmt(SensitivityParameterId::N2OCalculationMethod),
                ),
                (
                    "N₂O-EF",
                    Some(n2o_emission_factor),
                    unit.fmt(SensitivityParameterId::N2OCustomFactor),
                ),
                (
                    SensitivityParameterId::N2OSideStreamFactor.label(),
                    n2o_emissions
                        .side_stream_emission_factor
                        .map(format_number(lang)),
                    unit.fmt(SensitivityParameterId::N2OSideStreamFactor),
                ),
            ],
        },
        TableSection {
            title: "Methanemissionen aus Blockheizkraftwerken (BHKW)".to_string(),
            rows: vec![
                (
                    SensitivityParameterId::CH4ChpCalculationMethod.label(),
                    ch4_chp_emissions
                        .calculation_method
                        .as_ref()
                        .map(|m| m.label().to_string()),
                    unit.fmt(SensitivityParameterId::CH4ChpCalculationMethod),
                ),
                (
                    "BHKW CH₄-EF",
                    Some(ch4_chp_emission_factor),
                    unit.fmt(SensitivityParameterId::CH4ChpCustomFactor),
                ),
            ],
        },
        TableSection {
            title: "Methanemissionen aus offenen Faultürmen und bei der Schlammlagerung"
                .to_string(),
            rows: vec![
                (
                    SensitivityParameterId::SludgeBagsCustomFactor.label(),
                    ch4_sewage_sludge_emissions
                        .emission_factor_sludge_bags
                        .map(format_number(lang)),
                    unit.fmt(SensitivityParameterId::SludgeBagsCustomFactor),
                ),
                (
                    SensitivityParameterId::SludgeStorageCustomFactor.label(),
                    ch4_sewage_sludge_emissions
                        .emission_factor_sludge_storage_containers
                        .map(format_number(lang)),
                    unit.fmt(SensitivityParameterId::SludgeStorageCustomFactor),
                ),
            ],
        },
        TableSection {
            title: "Fossile CO₂-Emissionen aus Abwasser".to_string(),
            rows: vec![(
                SensitivityParameterId::CO2FossilCustomFactor.label(),
                co2_fossil_emissions
                    .emission_factor
                    .map(format_number(lang)),
                unit.fmt(SensitivityParameterId::CO2FossilCustomFactor),
            )],
        },
    ];
    Table { sections }
}

pub fn co2_equivalents_as_table(eq: &domain::CO2Equivalents, _unit: UnitFormatting) -> Table {
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

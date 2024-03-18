use serde::Serialize;

use klick_boundary::{PlantProfile, SensitivityParameters};

use crate::{
    AnnualAverageEffluentId, AnnualAverageInfluentId, EnergyConsumptionId, Lng,
    OperatingMaterialId, ProfileValueId, SensitivityParameterId, SewageSludgeTreatmentId,
    SideStreamTreatmentId, ValueLabel, ValueUnit,
};

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
) -> Table {
    let SensitivityParameters {
        n2o_emissions,
        ch4_chp_emissions,
        ch4_sewage_sludge_emissions,
        co2_fossil_emissions,
    } = parameters;

    let lang = Lng::De;

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
                    SensitivityParameterId::N2OCustomFactor.label(),
                    n2o_emissions
                        .custom_emission_factor
                        .map(format_number(lang)),
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
                    SensitivityParameterId::CH4ChpCustomFactor.label(),
                    ch4_chp_emissions
                        .custom_emission_factor
                        .map(format_number(lang)),
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

fn format_number(lang: Lng) -> impl Fn(f64) -> String {
    move |n| lang.format_number(n)
}

fn format_number_with_thousands_seperator(lang: Lng) -> impl Fn(f64) -> String {
    move |n| lang.format_number_with_thousands_seperator(n)
}

fn format_bool(lang: Lng) -> impl Fn(bool) -> String {
    move |x| lang.format_bool(x).to_string()
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

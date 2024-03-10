use serde::Serialize;

use klick_boundary::PlantProfile;

use crate::{
    AnnualAverageEffluentId, AnnualAverageInfluentId, EnergyConsumptionId, Lng,
    OperatingMaterialId, ProfileValueId, SewageSludgeTreatmentId, ValueLabel, ValueUnit,
};

pub fn plant_profile_as_table(profile: &PlantProfile) -> Table {
    let PlantProfile {
        plant_name,
        population_equivalent,
        wastewater,
        influent_average,
        effluent_average,
        energy_consumption,
        sewage_sludge_treatment,
        side_stream_treatment: _, // FIXME implement
        operating_materials,
        emission_factors: _,
        energy_emission_factors: _, // FIXME implement
    } = profile;

    let lang = Lng::De;

    let sections = vec![
        TableSection {
            title: "Angaben zur Kläranlage".to_string(),
            rows: vec![
                (
                    ProfileValueId::PlantName.label(),
                    plant_name.clone(),
                    ProfileValueId::PlantName.unit_as_latex(),
                ),
                (
                    ProfileValueId::PopulationEquivalent.label(),
                    population_equivalent.map(format_number_with_thousands_seperator(lang)),
                    ProfileValueId::PopulationEquivalent.unit_as_latex(),
                ),
                (
                    ProfileValueId::Wastewater.label(),
                    wastewater.map(format_number_with_thousands_seperator(lang)),
                    ProfileValueId::Wastewater.unit_as_latex(),
                ),
            ],
        },
        TableSection {
            title: "Zulauf-Parameter (Jahresmittelwerte)".to_string(),
            rows: vec![
                (
                    AnnualAverageInfluentId::Nitrogen.label(),
                    influent_average.nitrogen.map(format_number(lang)),
                    AnnualAverageInfluentId::Nitrogen.unit_as_latex(),
                ),
                (
                    AnnualAverageInfluentId::ChemicalOxygenDemand.label(),
                    influent_average
                        .chemical_oxygen_demand
                        .map(format_number(lang)),
                    AnnualAverageInfluentId::ChemicalOxygenDemand.unit_as_latex(),
                ),
            ],
        },
        TableSection {
            title: "Ablauf-Parameter (Jahresmittelwerte)".to_string(),
            rows: vec![
                (
                    AnnualAverageEffluentId::Nitrogen.label(),
                    effluent_average.nitrogen.map(format_number(lang)),
                    AnnualAverageEffluentId::Nitrogen.unit_as_latex(),
                ),
                (
                    AnnualAverageEffluentId::ChemicalOxygenDemand.label(),
                    effluent_average
                        .chemical_oxygen_demand
                        .map(format_number(lang)),
                    AnnualAverageEffluentId::ChemicalOxygenDemand.unit_as_latex(),
                ),
            ],
        },
        TableSection {
            title: "Energiebedarf".to_string(),
            rows: vec![
                (
                    EnergyConsumptionId::SewageGasProduced.label(),
                    energy_consumption
                        .sewage_gas_produced
                        .map(format_number_with_thousands_seperator(lang)),
                    EnergyConsumptionId::SewageGasProduced.unit_as_latex(),
                ),
                (
                    EnergyConsumptionId::MethaneFraction.label(),
                    energy_consumption.methane_fraction.map(format_number(lang)),
                    EnergyConsumptionId::MethaneFraction.unit_as_latex(),
                ),
                (
                    EnergyConsumptionId::GasSupply.label(),
                    energy_consumption.gas_supply.map(format_number(lang)),
                    EnergyConsumptionId::GasSupply.unit_as_latex(),
                ),
                (
                    EnergyConsumptionId::PurchaseOfBiogas.label(),
                    energy_consumption.purchase_of_biogas.map(format_bool(lang)),
                    EnergyConsumptionId::PurchaseOfBiogas.unit_as_latex(),
                ),
                (
                    EnergyConsumptionId::TotalPowerConsumption.label(),
                    energy_consumption
                        .total_power_consumption
                        .map(format_number_with_thousands_seperator(lang)),
                    EnergyConsumptionId::TotalPowerConsumption.unit_as_latex(),
                ),
                (
                    EnergyConsumptionId::OnSitePowerGeneration.label(),
                    energy_consumption
                        .on_site_power_generation
                        .map(format_number_with_thousands_seperator(lang)),
                    EnergyConsumptionId::OnSitePowerGeneration.unit_as_latex(),
                ),
                (
                    EnergyConsumptionId::EmissionFactorElectricityMix.label(),
                    energy_consumption
                        .emission_factor_electricity_mix
                        .map(format_number(lang)),
                    EnergyConsumptionId::EmissionFactorElectricityMix.unit_as_latex(),
                ),
            ],
        },
        TableSection {
            title: "Klärschlammbehandlung".to_string(),
            rows: vec![
                (
                    SewageSludgeTreatmentId::SewageSludgeForDisposal.label(),
                    sewage_sludge_treatment
                        .sewage_sludge_for_disposal
                        .map(format_number(lang)),
                    SewageSludgeTreatmentId::SewageSludgeForDisposal.unit_as_latex(),
                ),
                (
                    SewageSludgeTreatmentId::TransportDistance.label(),
                    sewage_sludge_treatment
                        .transport_distance
                        .map(format_number(lang)),
                    SewageSludgeTreatmentId::TransportDistance.unit_as_latex(),
                ),
                (
                    SewageSludgeTreatmentId::DigesterCount.label(),
                    sewage_sludge_treatment
                        .digester_count
                        .map(|n| n.to_string()),
                    SewageSludgeTreatmentId::DigesterCount.unit_as_latex(),
                ),
            ],
        },
        TableSection {
            title: "Eingesetzte Betriebsstoffe".to_string(),
            rows: vec![
                (
                    OperatingMaterialId::FeCl3.label(),
                    operating_materials.fecl3.map(format_number(lang)),
                    OperatingMaterialId::FeCl3.unit_as_latex(),
                ),
                (
                    OperatingMaterialId::FeClSO4.label(),
                    operating_materials.feclso4.map(format_number(lang)),
                    OperatingMaterialId::FeClSO4.unit_as_latex(),
                ),
                (
                    OperatingMaterialId::CaOH2.label(),
                    operating_materials.caoh2.map(format_number(lang)),
                    OperatingMaterialId::CaOH2.unit_as_latex(),
                ),
                (
                    OperatingMaterialId::SyntheticPolymers.label(),
                    operating_materials
                        .synthetic_polymers
                        .map(format_number(lang)),
                    OperatingMaterialId::SyntheticPolymers.unit_as_latex(),
                ),
            ],
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

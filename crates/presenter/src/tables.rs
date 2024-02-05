use serde::Serialize;

use klick_boundary::PlantProfile;

use crate::{
    AnnualAverageEffluentId, AnnualAverageInfluentId, EnergyConsumptionId, Lng,
    OperatingMaterialId, ProfileValueId, SewageSludgeTreatmentId, ValueLabel,
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
        operating_materials,
    } = profile;

    let lang = Lng::De;

    let sections = vec![
        TableSection {
            title: "Angaben zur Kläranlage".to_string(),
            rows: vec![
                (
                    ProfileValueId::PlantName.label().to_string(),
                    plant_name.clone(),
                ),
                (
                    ProfileValueId::PopulationEquivalent.label().to_string(),
                    population_equivalent.map(format_number_with_thousands_seperator(lang)),
                ),
                (
                    ProfileValueId::Wastewater.label().to_string(),
                    wastewater.map(format_number_with_thousands_seperator(lang)),
                ),
            ],
        },
        TableSection {
            title: "Zulauf-Parameter (Jahresmittelwerte)".to_string(),
            rows: vec![
                (
                    AnnualAverageInfluentId::Nitrogen.label().to_string(),
                    influent_average.nitrogen.map(format_number(lang)),
                ),
                (
                    AnnualAverageInfluentId::ChemicalOxygenDemand
                        .label()
                        .to_string(),
                    influent_average
                        .chemical_oxygen_demand
                        .map(format_number(lang)),
                ),
                (
                    AnnualAverageInfluentId::Phosphorus.label().to_string(),
                    influent_average.phosphorus.map(format_number(lang)),
                ),
            ],
        },
        TableSection {
            title: "Ablauf-Parameter (Jahresmittelwerte)".to_string(),
            rows: vec![
                (
                    AnnualAverageEffluentId::Nitrogen.label().to_string(),
                    effluent_average.nitrogen.map(format_number(lang)),
                ),
                (
                    AnnualAverageEffluentId::ChemicalOxygenDemand
                        .label()
                        .to_string(),
                    effluent_average
                        .chemical_oxygen_demand
                        .map(format_number(lang)),
                ),
                (
                    AnnualAverageEffluentId::Phosphorus.label().to_string(),
                    effluent_average.phosphorus.map(format_number(lang)),
                ),
            ],
        },
        TableSection {
            title: "Energiebedarf".to_string(),
            rows: vec![
                (
                    EnergyConsumptionId::SewageGasProduced.label().to_string(),
                    energy_consumption
                        .sewage_gas_produced
                        .map(format_number_with_thousands_seperator(lang)),
                ),
                (
                    EnergyConsumptionId::MethaneFraction.label().to_string(),
                    energy_consumption.methane_fraction.map(format_number(lang)),
                ),
                (
                    EnergyConsumptionId::GasSupply.label().to_string(),
                    energy_consumption.gas_supply.map(format_number(lang)),
                ),
                (
                    EnergyConsumptionId::PurchaseOfBiogas.label().to_string(),
                    energy_consumption.purchase_of_biogas.map(format_bool(lang)),
                ),
                (
                    EnergyConsumptionId::TotalPowerConsumption
                        .label()
                        .to_string(),
                    energy_consumption
                        .total_power_consumption
                        .map(format_number_with_thousands_seperator(lang)),
                ),
                (
                    EnergyConsumptionId::OnSitePowerGeneration
                        .label()
                        .to_string(),
                    energy_consumption
                        .on_site_power_generation
                        .map(format_number_with_thousands_seperator(lang)),
                ),
                (
                    EnergyConsumptionId::EmissionFactorElectricityMix
                        .label()
                        .to_string(),
                    energy_consumption
                        .emission_factor_electricity_mix
                        .map(format_number(lang)),
                ),
            ],
        },
        TableSection {
            title: "Klärschlammbehandlung".to_string(),
            rows: vec![
                (
                    SewageSludgeTreatmentId::SewageSludgeForDisposal
                        .label()
                        .to_string(),
                    sewage_sludge_treatment
                        .sewage_sludge_for_disposal
                        .map(format_number(lang)),
                ),
                (
                    SewageSludgeTreatmentId::TransportDistance
                        .label()
                        .to_string(),
                    sewage_sludge_treatment
                        .transport_distance
                        .map(format_number(lang)),
                ),
                (
                    SewageSludgeTreatmentId::DigesterCount.label().to_string(),
                    sewage_sludge_treatment
                        .digester_count
                        .map(|n| n.to_string()),
                ),
            ],
        },
        TableSection {
            title: "Eingesetzte Betriebsstoffe".to_string(),
            rows: vec![
                (
                    OperatingMaterialId::FeCl3.label().to_string(),
                    operating_materials.fecl3.map(format_number(lang)),
                ),
                (
                    OperatingMaterialId::FeClSO4.label().to_string(),
                    operating_materials.feclso4.map(format_number(lang)),
                ),
                (
                    OperatingMaterialId::CaOH2.label().to_string(),
                    operating_materials.caoh2.map(format_number(lang)),
                ),
                (
                    OperatingMaterialId::SyntheticPolymers.label().to_string(),
                    operating_materials
                        .synthetic_polymers
                        .map(format_number(lang)),
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
    pub rows: Vec<(String, Option<String>)>,
}

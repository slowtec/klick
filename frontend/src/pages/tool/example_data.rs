use klick_boundary::*;
use klick_domain::{InputValueId as Id, Value};

pub fn example_form_data() -> FormData {
    let mut plant_profile = PlantProfile::default();
    plant_profile.wastewater = Some(2_135_250.0);
    plant_profile.influent_average = AnnualAverageInfluent {
        total_nitrogen: Some(94.0),
        chemical_oxygen_demand: Some(1_020.0),
        total_organic_carbohydrates: Some(382.5),
    };
    plant_profile.effluent_average = AnnualAverageEffluent {
        total_nitrogen: Some(15.77),
        chemical_oxygen_demand: Some(47.18),
    };
    plant_profile.energy_consumption = EnergyConsumption {
        sewage_gas_produced: Some(420_000.0),
        methane_fraction: Some(62.0),
        purchase_of_biogas: Some(false),
        total_power_consumption: Some(1_665_000.0),
        on_site_power_generation: Some(810_000.0),
        emission_factor_electricity_mix: Some(420.0),
        heating_oil: None,
        gas_supply: None,
    };
    plant_profile.sewage_sludge_treatment = SewageSludgeTreatment {
        sludge_bags_are_closed: Some(false),
        sludge_storage_containers_are_closed: Some(false),
        sewage_sludge_for_disposal: Some(3016.5),
        transport_distance: Some(150.0),
        digester_count: Some(1),
    };
    plant_profile.operating_materials = OperatingMaterials {
        fecl3: Some(310.5),
        feclso4: None,
        caoh2: None,
        synthetic_polymers: Some(12.0),
    };
    plant_profile.side_stream_treatment = SideStreamTreatment {
        total_nitrogen: None,
    };
    let sensitivity_parameters = SensitivityParameters {
        n2o_emissions: N2OEmissionsSensitivity {
            calculation_method: Some(N2oEmissionFactorCalcMethod::Ipcc2019),
            custom_emission_factor: None,
            side_stream_emission_factor: None,
        },
        ch4_chp_emissions: CH4ChpEmissionsSensitivity {
            calculation_method: Some(CH4ChpEmissionFactorCalcMethod::GasolineEngine),
            custom_emission_factor: None,
        },
        ch4_sewage_sludge_emissions: SewageSludgeTreatmentEmissionsSensitivity {
            emission_factor_sludge_bags: None,
            emission_factor_sludge_storage_containers: None,
        },
        co2_fossil_emissions: FossilEmissonsSensitivity {
            emission_factor: None,
        },
    };

    let optimization_scenario = OptimizationScenario {
        sewage_sludge_treatment: SewageSludgeTreatmentScenario::default(),
        energy_emissions: EnergyEmissionScenario::default(),
        side_stream_treatment: SideStreamTreatmentScenario::default(),
    };
    let project_title = None;

    let mut data = FormData {
        project_title,
        plant_profile,
        sensitivity_parameters,
        optimization_scenario,
    };

    data.set(Id::PlantName, Some(Value::new_text("Muster Klärwerk")));
    data.set(Id::PopulationEquivalent, Some(Value::new_count(50_000)));
    data
}

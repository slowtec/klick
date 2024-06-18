use crate::*;

// FIXME:
// Move this constants into the domain layer
// but also define all values,
// that are optional also with `Option` type.
pub const N2O_DEFAULT_CUSTOM_FACTOR: f64 = 2.0;
pub const _N2O_DEFAULT_SIDE_STREAM_FACTOR: f64 = 2.0;
pub const CO2_DEFAULT_FOSSIL_FACTOR: f64 = 3.85;

pub fn profile(mut data: FormData) -> FormData {
    data.sensitivity_parameters = Default::default();
    data.optimization_scenario = Default::default();

    add_missing_values(data)
}

pub fn sensitivity(mut data: FormData) -> FormData {
    data.optimization_scenario = Default::default();
    data.sensitivity_parameters
        .n2o_emissions
        .custom_emission_factor
        .get_or_insert(N2O_DEFAULT_CUSTOM_FACTOR);

    // FIXME: set default values
    // match custom_sludge_bags_factor_field.get() {
    //     Some(_v) => custom_sludge_bags_factor.set(custom_sludge_bags_factor_field.get()),
    //     None => custom_sludge_bags_factor.set(Some(f64::from(
    //         klick_domain::constants::EMISSION_FACTOR_SLUDGE_BAGS,
    //     ))),
    // }
    // match custom_sludge_storage_containers_factor_field.get() {
    //     Some(_v) => custom_sludge_storage_containers_factor
    //         .set(custom_sludge_storage_containers_factor_field.get()),
    //     None => custom_sludge_storage_containers_factor.set(Some(f64::from(
    //         klick_domain::constants::EMISSION_FACTOR_SLUDGE_STORAGE,
    //     ))),
    // }

    add_missing_values(data)
}

pub fn recommendations(data: FormData) -> FormData {
    add_missing_values(data)
}

fn add_missing_values(mut data: FormData) -> FormData {
    add_missing_profile_values(&mut data.plant_profile);
    add_missing_sensitivity_parameters(&mut data.sensitivity_parameters);
    add_missing_optimization_scenario(&mut data.optimization_scenario);
    data
}

fn add_missing_profile_values(profile: &mut PlantProfile) {
    let PlantProfile {
        influent_average,
        energy_consumption,
        sewage_sludge_treatment,
        side_stream_treatment,
        operating_materials,
        ..
    } = profile;

    // -- Annual Average Influent -- //

    influent_average
        .total_organic_carbohydrates
        .get_or_insert(0.0);

    // -- EnergyConsumption --//

    let EnergyConsumption {
        on_site_power_generation,
        gas_supply,
        purchase_of_biogas,
        heating_oil,
        sewage_gas_produced,
        methane_fraction,
        ..
    } = energy_consumption;

    on_site_power_generation.get_or_insert(0.0);
    gas_supply.get_or_insert(0.0);
    purchase_of_biogas.get_or_insert(false);
    heating_oil.get_or_insert(0.0);
    sewage_gas_produced.get_or_insert(0.0);
    methane_fraction.get_or_insert(62.0);

    // -- Sewage Sludge Treatment -- //

    let SewageSludgeTreatment {
        digester_count,
        sludge_bags_are_closed,
        sludge_storage_containers_are_closed,
        transport_distance,
        ..
    } = sewage_sludge_treatment;

    digester_count.get_or_insert(0);
    sludge_bags_are_closed.get_or_insert(false);
    sludge_storage_containers_are_closed.get_or_insert(false);
    transport_distance.get_or_insert(0.0);

    // -- Side Stream Treatment -- //

    side_stream_treatment.total_nitrogen.get_or_insert(0.0);

    // -- Operating Materials -- //

    operating_materials.feclso4.get_or_insert(0.0);
    operating_materials.caoh2.get_or_insert(0.0);
}

fn add_missing_sensitivity_parameters(s: &mut SensitivityParameters) {
    let SensitivityParameters {
        n2o_emissions,
        ch4_chp_emissions,
        ch4_sewage_sludge_emissions,
        co2_fossil_emissions,
    } = s;

    // -- N2OEmissionsSensitivity -- //
    let N2OEmissionsSensitivity {
        calculation_method,
        custom_emission_factor,
        side_stream_emission_factor,
    } = n2o_emissions;

    calculation_method.get_or_insert(domain::units::N2oEmissionFactorCalcMethod::default().into());
    custom_emission_factor.get_or_insert(2.0);
    side_stream_emission_factor.get_or_insert(2.0);

    // -- CH4ChpEmissionsSensitivity -- //

    let CH4ChpEmissionsSensitivity {
        calculation_method,
        custom_emission_factor,
    } = ch4_chp_emissions;

    calculation_method
        .get_or_insert(domain::units::Ch4ChpEmissionFactorCalcMethod::default().into());
    custom_emission_factor.get_or_insert(3.0);

    //-- Sewage Sludge Treatment Emissions Sensitivity -- //

    let SewageSludgeTreatmentEmissionsSensitivity {
        emission_factor_sludge_bags,
        emission_factor_sludge_storage_containers,
    } = ch4_sewage_sludge_emissions;

    emission_factor_sludge_bags.get_or_insert(1.25);
    emission_factor_sludge_storage_containers.get_or_insert(2.0);

    // -- FossilEmissonsSensitivity -- //

    co2_fossil_emissions.emission_factor.get_or_insert(3.85);
}

fn add_missing_optimization_scenario(scenario: &mut OptimizationScenario) {
    let OptimizationScenario {
        sewage_sludge_treatment,
        energy_emissions,
        side_stream_treatment,
    } = scenario;

    // -- Sewage Sludge Treatment Scenario -- //

    let SewageSludgeTreatmentScenario {
        sludge_bags_are_closed,
        sludge_storage_containers_are_closed,
    } = sewage_sludge_treatment;

    sludge_bags_are_closed.get_or_insert(false);
    sludge_storage_containers_are_closed.get_or_insert(false);

    // -- Energy Emission Scenario -- //

    let EnergyEmissionScenario {
        process_energy_savings,
        fossil_energy_savings,
        photovoltaic_energy_expansion,
        estimated_self_photovoltaic_usage,
        wind_energy_expansion,
        estimated_self_wind_energy_usage,
        water_energy_expansion,
        estimated_self_water_energy_usage,
        district_heating,
    } = energy_emissions;

    process_energy_savings.get_or_insert(0.0);
    fossil_energy_savings.get_or_insert(0.0);
    photovoltaic_energy_expansion.get_or_insert(0.0);
    estimated_self_photovoltaic_usage.get_or_insert(100.0);
    wind_energy_expansion.get_or_insert(0.0);
    estimated_self_wind_energy_usage.get_or_insert(100.0);
    water_energy_expansion.get_or_insert(0.0);
    estimated_self_water_energy_usage.get_or_insert(100.0);
    district_heating.get_or_insert(0.0);

    // -- Side Stream Treatment Scenario -- //

    side_stream_treatment
        .side_stream_cover_is_closed
        .get_or_insert(false);
}

use std::collections::HashMap;

use klick_boundary::*;

use crate::forms::FieldSignal;

use super::fields::{load_fields, FieldId};

pub fn load_example_field_signal_values(signals: &HashMap<FieldId, FieldSignal>) {
    let (input, scenario) = example_input_data();
    load_fields(signals, input, scenario);
}

fn example_input_data() -> (InputData, Scenario) {
    // TODO: let csb_zu = 1045.0;
    // TODO: let p_zu = 9.9;
    // TODO: let p_ab = 0.4;
    // TODO: let gas_zusatz = 1_300_000.0;
    // TODO: let biogas = false;

    let input = InputData {
        plant_name: Some("Muster Klärwerk".to_string()),
        population_equivalent: Some(50_000.0),
        wastewater: Some(2_135_250.0),
        influent_average: AnnualAverage {
            nitrogen: Some(94.0),
            chemical_oxygen_demand: Some(1_020.0),
            phosphorus: Some(15.38),
        },
        effluent_average: AnnualAverage {
            nitrogen: Some(15.77),
            chemical_oxygen_demand: Some(47.18),
            phosphorus: Some(1.02),
        },
        energy_consumption: EnergyConsumption {
            sewage_gas_produced: Some(420_000.0),
            methane_fraction: Some(62.0),
            gas_supply: None,
            purchase_of_biogas: None,
            total_power_consumption: Some(1_665_000.0),
            on_site_power_generation: Some(810_000.0),
            emission_factor_electricity_mix: Some(420.0),
        },
        sewage_sludge_treatment: SewageSludgeTreatment {
            open_sludge_bags: Some(true),
            open_sludge_storage_containers: Some(false),
            sewage_sludge_for_disposal: Some(3016.5),
            transport_distance: Some(150.0),
        },
        operating_materials: OperatingMaterials {
            fecl3: Some(310.5),
            feclso4: Some(0.0),
            caoh2: Some(0.0),
            synthetic_polymers: Some(12.0),
        },
    };

    let scenario = Scenario {
        n2o_emission_factor: N2oEmissionFactorScenario {
            custom_factor: None,
            calculation_method: N2oEmissionFactorCalcMethod::Ipcc2019,
        },
        ch4_chp_emission_factor: None,
    };

    (input, scenario)
}

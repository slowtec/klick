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
        population_equivalent: Some(120_000.0),
        wastewater: Some(5_000_000.0),
        influent_average: AnnualAverage {
            nitrogen: Some(122.0),
            chemical_oxygen_demand: None,
            phosphorus: None,
        },
        effluent_average: AnnualAverage {
            nitrogen: Some(11.76),
            chemical_oxygen_demand: Some(129.0),
            phosphorus: None,
        },
        energy_consumption: EnergyConsumption {
            sewage_gas_produced: Some(1_260_000.0),
            methane_fraction: Some(62.0),
            gas_supply: None,
            purchase_of_biogas: None,
            total_power_consumption: Some(2_683_259.0),
            on_site_power_generation: Some(2_250_897.0),
            emission_factor_electricity_mix: Some(468.0),
        },
        sewage_sludge_treatment: SewageSludgeTreatment {
            open_sludge_bags: Some(true),
            open_sludge_storage_containers: Some(true),
            sewage_sludge_for_disposal: Some(3687.6),
            transport_distance: Some(47.0),
        },
        operating_materials: OperatingMaterials {
            fecl3: Some(0.0),
            feclso4: Some(326.0),
            caoh2: Some(326.26),
            synthetic_polymers: Some(23.62),
        },
    };

    let scenario = Scenario {
        n2o_emission_factor: N2oEmissionFactorScenario {
            custom_factor: None,
            calculation_method: N2oEmissionFactorCalcMethod::Ipcc2019,
        },
    };

    (input, scenario)
}

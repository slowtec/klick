use klick_domain::{
    units::{Percent, Ratio},
    CH4ChpEmissionFactorCalcMethod, CO2Equivalents, CalculatedEmissionFactors,
    EmissionFactorCalculationMethods, EmissionInfluencingValues, N2oEmissionFactorCalcMethod,
};

pub fn emission_factor_calculation_methods_to_csv(
    methods: EmissionFactorCalculationMethods,
) -> String {
    [
        [
            "emission_factor_calculation_methods.n2o",
            n2o_emission_factor_calc_method_to_csv_name(&methods.n2o),
        ]
        .join(","),
        [
            "emission_factor_calculation_methods.ch4",
            match &methods.ch4 {
                Some(ch4) => ch4_chp_emission_factor_calc_method_to_csv_name(ch4),
                None => "Nicht festgelegt",
            },
        ]
        .join(","),
    ]
    .join("\n")
}

pub fn emission_factors_to_csv(factors: &CalculatedEmissionFactors) -> String {
    [
        format!(
            "{}, {}\n",
            "emission_factors.n2o",
            &f64::from(factors.n2o.convert_to::<Percent>()).to_string()
        ),
        format!(
            "{}, {}\n",
            "emission_factors.ch4",
            &f64::from(factors.ch4.convert_to::<Percent>()).to_string()
        ),
    ]
    .join("")
}

// TODO: use ValueLabel & ValueId
fn n2o_emission_factor_calc_method_to_csv_name(
    method: &N2oEmissionFactorCalcMethod,
) -> &'static str {
    match method {
        N2oEmissionFactorCalcMethod::TuWien2016 => "TU Wien 2016",
        N2oEmissionFactorCalcMethod::Optimistic => "Optimistisch",
        N2oEmissionFactorCalcMethod::Pesimistic => "Pessimistisch",
        N2oEmissionFactorCalcMethod::Ipcc2019 => "PCC 2019",
        N2oEmissionFactorCalcMethod::Custom(_) => "Benutzerdefiniert",
    }
}

// TODO: use ValueLabel & ValueId
fn ch4_chp_emission_factor_calc_method_to_csv_name(
    method: &CH4ChpEmissionFactorCalcMethod,
) -> &'static str {
    match method {
        CH4ChpEmissionFactorCalcMethod::MicroGasTurbines => "Mikrogasturbinen",
        CH4ChpEmissionFactorCalcMethod::GasolineEngine => "Ottomotor",
        CH4ChpEmissionFactorCalcMethod::JetEngine => "Zündstrahlmotor",
        CH4ChpEmissionFactorCalcMethod::Custom(_) => "Benutzerdefiniert",
    }
}

pub fn emission_influencing_values_to_csv(v: EmissionInfluencingValues) -> String {
    // let mut output: String = String::new();
    // // make this multiple lines
    // output += &format!(
    //     "population_equivalent, {}\n",
    //     f64::from(self.population_equivalent)
    // );
    // output += &format!("wastewater, {}\n", f64::from(self.wastewater));
    // output += &format!(
    //     "influent_average.nitrogen, {}\n",
    //     f64::from(self.influent_average.nitrogen)
    // );
    // output += &format!(
    //     "effluent_average.nitrogen, {}\n",
    //     f64::from(self.effluent_average.nitrogen)
    // );
    // output += &format!(
    //     "effluent_average.chemical_oxygen_demand, {}\n",
    //     f64::from(self.effluent_average.chemical_oxygen_demand)
    // );
    // output += &format!(
    //     "energy_consumption.sewage_gas_produced, {}\n",
    //     f64::from(self.energy_consumption.sewage_gas_produced)
    // );
    // output += &format!(
    //     "energy_consumption.methane_fraction, {}\n",
    //     f64::from(self.energy_consumption.methane_fraction)
    // );
    // output += &format!(
    //     "energy_consumption.total_power_consumption, {}\n",
    //     f64::from(self.energy_consumption.total_power_consumption)
    // );
    // output += &format!(
    //     "energy_consumption.on_site_power_generation, {}\n",
    //     f64::from(self.energy_consumption.on_site_power_generation)
    // );
    // output += &format!(
    //     "energy_consumption.emission_factor_electricity_mix, {}\n",
    //     f64::from(self.energy_consumption.emission_factor_electricity_mix)
    // );
    // output += &format!(
    //     "sewage_sludge_treatment.sludge_bags_are_open, {}\n",
    //     self.sewage_sludge_treatment.sludge_bags_are_open
    // );
    // output += &format!(
    //     "sewage_sludge_treatment.custom_sludge_bags_factor, {}\n",
    //     f64::from(
    //         self.sewage_sludge_treatment
    //             .custom_sludge_bags_factor
    //             .unwrap_or(QubicmetersPerHour::new(-0.0))
    //     )
    // );
    // output += &format!(
    //     "sewage_sludge_treatment.sludge_storage_containers_are_open, {}\n",
    //     self.sewage_sludge_treatment
    //         .sludge_storage_containers_are_open
    // );
    // output += &format!(
    //     "sewage_sludge_treatment.custom_sludge_storage_containers_factor, {}\n",
    //     f64::from(
    //         self.sewage_sludge_treatment
    //             .custom_sludge_storage_containers_factor
    //             .unwrap_or(Percent::new(0.0))
    //     )
    // );
    // output += &format!(
    //     "sewage_sludge_treatment.sewage_sludge_for_disposal, {}\n",
    //     f64::from(self.sewage_sludge_treatment.sewage_sludge_for_disposal)
    // );
    // output += &format!(
    //     "sewage_sludge_treatment.transport_distance, {}\n",
    //     f64::from(self.sewage_sludge_treatment.transport_distance)
    // );
    // output += &format!(
    //     "sewage_sludge_treatment.digester_count, {}\n",
    //     self.sewage_sludge_treatment.digester_count.unwrap_or(0)
    // );
    // output += &format!(
    //     "operating_materials.fecl3, {}\n",
    //     f64::from(self.operating_materials.fecl3)
    // );
    // output += &format!(
    //     "operating_materials.feclso4, {}\n",
    //     f64::from(self.operating_materials.feclso4)
    // );
    // output += &format!(
    //     "operating_materials.caoh2, {}\n",
    //     f64::from(self.operating_materials.caoh2)
    // );
    // output += &format!(
    //     "operating_materials.synthetic_polymers, {}\n",
    //     f64::from(self.operating_materials.synthetic_polymers)
    // );
    // output
    todo!()
}

pub fn co2_equivalents_to_csv(eq: CO2Equivalents) -> String {
    // // FIXME add n2o_side_stream fossil_emissions
    // // FIXME add     pub oil_emissions: Tons,
    // //     pub gas_emissions: Tons,
    // let mut output: String = String::new();
    // output += &format!("n2o_plant, {}\n", f64::from(self.n2o_plant));
    // output += &format!("n2o_water, {}\n", f64::from(self.n2o_water));
    // output += &format!("n2o_emissions, {}\n", f64::from(self.n2o_emissions));
    // output += &format!("ch4_plant, {}\n", f64::from(self.ch4_plant));
    // output += &format!(
    //     "ch4_sludge_storage_containers, {}\n",
    //     f64::from(self.ch4_sludge_storage_containers)
    // );
    // output += &format!("ch4_sludge_bags, {}\n", f64::from(self.ch4_sludge_bags));
    // output += &format!("ch4_water, {}\n", f64::from(self.ch4_water));
    // output += &format!(
    //     "ch4_combined_heat_and_power_plant, {}\n",
    //     f64::from(self.ch4_combined_heat_and_power_plant)
    // );
    // output += &format!("ch4_emissions, {}\n", f64::from(self.ch4_emissions));
    // output += &format!("fecl3, {}\n", f64::from(self.fecl3));
    // output += &format!("feclso4, {}\n", f64::from(self.feclso4));
    // output += &format!("caoh2, {}\n", f64::from(self.caoh2));
    // output += &format!(
    //     "synthetic_polymers, {}\n",
    //     f64::from(self.synthetic_polymers)
    // );
    // output += &format!("electricity_mix, {}\n", f64::from(self.electricity_mix));
    // output += &format!(
    //     "operating_materials, {}\n",
    //     f64::from(self.operating_materials)
    // );
    // output += &format!(
    //     "sewage_sludge_transport, {}\n",
    //     f64::from(self.sewage_sludge_transport)
    // );
    // output += &format!("emissions, {}\n", f64::from(self.total_emissions));
    // output += &format!("direct_emissions, {}\n", f64::from(self.direct_emissions));
    // output += &format!(
    //     "indirect_emissions, {}\n",
    //     f64::from(self.indirect_emissions)
    // );
    // output += &format!(
    //     "other_indirect_emissions, {}\n",
    //     f64::from(self.other_indirect_emissions)
    // );
    // output += &format!(
    //     "excess_energy_co2_equivalent, {}\n",
    //     f64::from(self.excess_energy_co2_equivalent)
    // );
    // output
    todo!()
}

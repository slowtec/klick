use anyhow::bail;

use klick_application as app;

use crate::{
    AnnualAverages, CO2Equivalents, EnergyConsumption, InputData, N2oEmissionFactorCalcMethod,
    N2oEmissionFactorScenario, OperatingMaterials, OutputData, SewageSludgeTreatment,
};

impl TryFrom<N2oEmissionFactorScenario> for app::N2oEmissionFactorCalcMethod {
    type Error = anyhow::Error;

    fn try_from(from: N2oEmissionFactorScenario) -> Result<Self, Self::Error> {
        use app::N2oEmissionFactorCalcMethod as A;
        use N2oEmissionFactorCalcMethod as M;

        let f = match from.calculation_method {
            M::ExtrapolatedParravicini => A::ExtrapolatedParravicini,
            M::Optimistic => A::Optimistic,
            M::Pesimistic => A::Pesimistic,
            M::Ipcc2019 => A::Ipcc2019,
            M::CustomFactor => {
                let Some(factor) = from.custom_factor else {
                    bail!("custom N2O emission factor is missing");
                };
                A::CustomFactor(factor)
            }
        };
        Ok(f)
    }
}

impl From<app::N2oEmissionFactorCalcMethod> for N2oEmissionFactorScenario {
    fn from(from: app::N2oEmissionFactorCalcMethod) -> Self {
        use app::N2oEmissionFactorCalcMethod as A;
        use N2oEmissionFactorCalcMethod as M;

        let calculation_method = match from {
            A::ExtrapolatedParravicini => M::ExtrapolatedParravicini,
            A::Optimistic => M::Optimistic,
            A::Pesimistic => M::Pesimistic,
            A::Ipcc2019 => M::Ipcc2019,
            A::CustomFactor(_) => M::CustomFactor,
        };

        let custom_factor = if let A::CustomFactor(factor) = from {
            Some(factor)
        } else {
            None
        };

        Self {
            calculation_method,
            custom_factor,
        }
    }
}

impl TryFrom<InputData> for app::InputData {
    type Error = anyhow::Error;

    fn try_from(from: InputData) -> Result<Self, Self::Error> {
        let InputData {
            plant_name,
            population_values,
            waste_water,
            inflow_averages,
            effluent_averages,
            energy_consumption,
            sewage_sludge_treatment,
            operating_materials,
        } = from;

        let Some(population_values) = population_values else {
            bail!("missing population_values");
        };

        let Some(waste_water) = waste_water else {
            bail!("missing waste_water");
        };

        let AnnualAverages {
            nitrogen,
            chemical_oxygen_demand: _,
            phosphorus: _,
        } = inflow_averages;

        let Some(inflow_nitrogen) = nitrogen else {
            bail!("missing inflow nitrogen");
        };

        let AnnualAverages {
            nitrogen,
            chemical_oxygen_demand,
            phosphorus: _,
        } = effluent_averages;

        let Some(effluent_nitrogen) = nitrogen else {
            bail!("missing effluent nitrogen");
        };
        let Some(effluent_chemical_oxygen_demand) = chemical_oxygen_demand else {
            bail!("missing effluent chemical_oxygen_demand");
        };

        let EnergyConsumption {
            sewage_gas_produced,
            methane_level,
            gas_supply: _,
            purchase_of_biogas: _,
            total_power_consumption,
            in_house_power_generation,
            emission_factor_electricity_mix,
        } = energy_consumption;

        let Some(sewage_gas_produced) = sewage_gas_produced else {
            bail!("missing sewage_gas_produced");
        };
        let Some(methane_level) = methane_level else {
            bail!("missing methane_level");
        };
        let Some(total_power_consumption) = total_power_consumption else {
            bail!("missing total_power_consumption");
        };
        let Some(in_house_power_generation) = in_house_power_generation else {
            bail!("missing in_house_power_generation");
        };
        let Some(emission_factor_electricity_mix) = emission_factor_electricity_mix else {
            bail!("missing emission_factor_electricity_mix");
        };

        let SewageSludgeTreatment {
            open_sludge_bags,
            open_sludge_storage_containers,
            sewage_sludge_for_disposal,
            transport_distance,
        } = sewage_sludge_treatment;

        let Some(open_sludge_bags) = open_sludge_bags else {
            bail!("missing open_sludge_bags");
        };
        let Some(open_sludge_storage_containers) = open_sludge_storage_containers else {
            bail!("missing open_sludge_storage_containers");
        };
        let Some(sewage_sludge_for_disposal) = sewage_sludge_for_disposal else {
            bail!("missing sewage_sludge_for_disposal");
        };
        let Some(transport_distance) = transport_distance else {
            bail!("missing transport_distance");
        };

        let OperatingMaterials {
            fecl3,
            feclso4,
            caoh2,
            synthetic_polymers,
        } = operating_materials;

        let Some(fecl3) = fecl3 else {
            bail!("missing fecl3");
        };
        let Some(feclso4) = feclso4 else {
            bail!("missing feclso4");
        };
        let Some(caoh2) = caoh2 else {
            bail!("missing caoh2");
        };
        let Some(synthetic_polymers) = synthetic_polymers else {
            bail!("missing synthetic_polymers");
        };

        Ok(Self {
            plant_name,
            ew: population_values,
            abwasser: waste_water,
            n_ges_zu: inflow_nitrogen,
            n_ges_ab: effluent_nitrogen,
            csb_ab: effluent_chemical_oxygen_demand,
            klaergas_gesamt: sewage_gas_produced,
            methangehalt: methane_level,
            strombedarf: total_power_consumption,
            energie_eigen: in_house_power_generation,
            ef_co2_strommix: emission_factor_electricity_mix,
            schlammtaschen: open_sludge_bags,
            schlammstapel: open_sludge_storage_containers,
            klaerschlamm_transport_km: transport_distance,
            klaerschlamm_entsorgung_m: sewage_sludge_for_disposal,
            betriebsstoffe_fe3: fecl3,
            betriebsstoffe_feso4: feclso4,
            betriebsstoffe_kalk: caoh2,
            betriebsstoffe_poly: synthetic_polymers,
        })
    }
}

impl From<app::InputData> for InputData {
    fn from(from: app::InputData) -> Self {
        let app::InputData {
            plant_name,
            ew,
            abwasser,
            n_ges_zu,
            csb_ab,
            n_ges_ab,
            klaergas_gesamt,
            methangehalt,
            strombedarf,
            energie_eigen,
            ef_co2_strommix,
            schlammtaschen,
            schlammstapel,
            klaerschlamm_transport_km,
            klaerschlamm_entsorgung_m,
            betriebsstoffe_fe3,
            betriebsstoffe_feso4,
            betriebsstoffe_kalk,
            betriebsstoffe_poly,
        } = from;

        let population_values = Some(ew);
        let waste_water = Some(abwasser);
        let inflow_nitrogen = Some(n_ges_zu);
        let effluent_nitrogen = Some(n_ges_ab);
        let effluent_chemical_oxygen_demand = Some(csb_ab);
        let sewage_gas_produced = Some(klaergas_gesamt);
        let methane_level = Some(methangehalt);
        let total_power_consumption = Some(strombedarf);
        let in_house_power_generation = Some(energie_eigen);
        let emission_factor_electricity_mix = Some(ef_co2_strommix);
        let open_sludge_bags = Some(schlammtaschen);
        let open_sludge_storage_containers = Some(schlammstapel);
        let transport_distance = Some(klaerschlamm_transport_km);
        let sewage_sludge_for_disposal = Some(klaerschlamm_entsorgung_m);
        let fecl3 = Some(betriebsstoffe_fe3);
        let feclso4 = Some(betriebsstoffe_feso4);
        let caoh2 = Some(betriebsstoffe_kalk);
        let synthetic_polymers = Some(betriebsstoffe_poly);

        let inflow_averages = AnnualAverages {
            nitrogen: inflow_nitrogen,
            chemical_oxygen_demand: None,
            phosphorus: None,
        };

        let effluent_averages = AnnualAverages {
            nitrogen: effluent_nitrogen,
            chemical_oxygen_demand: effluent_chemical_oxygen_demand,
            phosphorus: None,
        };

        let energy_consumption = EnergyConsumption {
            sewage_gas_produced,
            methane_level,
            gas_supply: None,
            purchase_of_biogas: None,
            total_power_consumption,
            in_house_power_generation,
            emission_factor_electricity_mix,
        };

        let sewage_sludge_treatment = SewageSludgeTreatment {
            open_sludge_bags,
            open_sludge_storage_containers,
            sewage_sludge_for_disposal,
            transport_distance,
        };

        let operating_materials = OperatingMaterials {
            fecl3,
            feclso4,
            caoh2,
            synthetic_polymers,
        };

        Self {
            plant_name,
            population_values,
            waste_water,
            inflow_averages,
            effluent_averages,
            energy_consumption,
            sewage_sludge_treatment,
            operating_materials,
        }
    }
}

impl From<OutputData> for app::OutputData {
    fn from(from: OutputData) -> Self {
        let OutputData {
            co2_equivalents,
            n2o_emission_factor,
        } = from;

        let CO2Equivalents {
            n2o_plant,
            n2o_water,
            ch4_sewage_treatment,
            ch4_sludge_storage_containers,
            ch4_sludge_bags,
            ch4_water,
            ch4_combined_heat_and_power_plant,
            fecl3,
            feclso4,
            caoh2,
            synthetic_polymers,
            electricity_mix,
            operating_materials,
            sewage_sludge_transport,
            emissions,
            direct_emissions,
            indirect_emissions,
            other_indirect_emissions,
        } = co2_equivalents;

        Self {
            co2eq_n2o_anlage: n2o_plant,
            co2eq_n2o_gewaesser: n2o_water,
            co2eq_ch4_klaerprozes: ch4_sewage_treatment,
            co2eq_ch4_schlammstapel: ch4_sludge_storage_containers,
            co2eq_ch4_schlammtasche: ch4_sludge_bags,
            co2eq_ch4_gewaesser: ch4_water,
            co2eq_ch4_bhkw: ch4_combined_heat_and_power_plant,
            co2eq_betriebsstoffe_fe3: fecl3,
            co2eq_betriebsstoffe_feso4: feclso4,
            co2eq_betriebsstoffe_kalk: caoh2,
            co2eq_betriebsstoffe_poly: synthetic_polymers,
            co2eq_strommix: electricity_mix,
            co2eq_betriebsstoffe: operating_materials,
            co2eq_klaerschlamm_transport: sewage_sludge_transport,
            emissionen_co2_eq: emissions,
            direkte_emissionen_co2_eq: direct_emissions,
            indirekte_emissionen_co2_eq: indirect_emissions,
            weitere_indirekte_emissionen_co2_eq: other_indirect_emissions,
            ef_n2o_anlage: n2o_emission_factor,
        }
    }
}

use klick_domain::{KilogramsPerLiter, KilogramsPerQubicmeter, LitersPerTonKilometer, Percent};

/// `[g ch4 / (population values * year)]`
pub const EMISSION_FACTOR_CH4_PLANT: f64 = 230.0;

/// 0,9 `[%]` of chemical oxygen demand effluent
pub const EMISSION_FACTOR_CH4_WATER: Percent = Percent::new(0.9);

/// 1,0 `[%]` ch4 factor
pub const EMISSION_FACTOR_CH4_CHP: Percent = Percent::new(1.0);

/// 0,5 `[%]` nitrogen effulent
pub const EMISSION_FACTOR_N2O_WATER: Percent = Percent::new(0.5);

/// 0.3 `[%]` of the total methane gas yield
pub const EMISSION_FACTOR_SLUDGE_BAGS: Percent = Percent::new(0.3);

/// 1,7 `[%]` of the total digester gas production
pub const EMISSION_FACTOR_SLUDGE_STORAGE: Percent = Percent::new(1.7);

/// `[g co2 / kg]` solution
pub const EMISSION_FACTOR_FECL3: f64 = 395.0;

/// `[g co2 / kg]` solution
pub const EMISSION_FACTOR_FECLSO4: f64 = 76.0;

/// `[g co2 / kg]` solution
pub const EMISSION_FACTOR_CAOH2: f64 = 1_055.3;

/// `[g co2 / kg]` solution
pub const EMISSION_FACTOR_POLYMERS: f64 = 2_200.0;

pub const GWP_N2O: f64 = 273.0;
pub const GWP_CH4: f64 = 28.0;

/// `[kg/m^3]` for standard cubic meters (GESTIS substance database)
pub const CONVERSION_FACTOR_CH4_M3_TO_KG: KilogramsPerQubicmeter =
    KilogramsPerQubicmeter::new(0.7175);

/// Emission factor `[kg CO₂/l]` of diesel.
///
/// # Calculation
///
/// According to the German [Umwelt Bundesamt](https://www.umweltbundesamt.de/)
/// the factor is `3,17` `[kg CO₂/kg]`.
///
/// Converted with with factor `0,835` -> `2,65` `[kg CO₂/l]` diesel.
///
/// # References
///
/// - [CO2-Emissionsfaktoren für fossile Brennstoffe - 28/2022](https://www.umweltbundesamt.de/publikationen/co2-emissionsfaktoren-fuer-fossile-brennstoffe-0) p.36f
pub const EMISSION_FACTOR_DIESEL: KilogramsPerLiter = KilogramsPerLiter::new(2.65);

/// `[l/tkm]`
pub const FUEL_CONSUMPTION: LitersPerTonKilometer = LitersPerTonKilometer::new(0.02);

/// 0,3 `[%]` of the nitrogen inflow
pub const EMISSION_FACTOR_N2O_OPTIMISTIC: Percent = Percent::new(0.3);

/// 0,8 `[%]` of the nitrogen inflow
pub const EMISSION_FACTOR_N2O_PESIMISTIC: Percent = Percent::new(0.8);

/// 1,6 `[%]` of the nitrogen inflow
pub const EMISSION_FACTOR_N2O_IPCC2019: Percent = Percent::new(1.6);

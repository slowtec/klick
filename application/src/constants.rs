/// g ch4 / (population values * year)
pub const EMISSION_FACTOR_CH4_PLANT: f64 = 230.0;

/// 0,9 % of chemical oxygen demand effluent
pub const EMISSION_FACTOR_CH4_WATER: f64 = 0.009;

/// 1,164 g ch4 / kwh
pub const EMISSION_FACTOR_CH4_CHP: f64 = 1.164;

/// 0,5 % of nitrogen effulent
pub const EMISSION_FACTOR_N2O_WATER: f64 = 0.005;

/// 0.3 % of the total methane gas yield
pub const EMISSION_FACTOR_SLUDGE_BAGS: f64 = 0.003;

/// 1,7 % of the total digester gas production
pub const EMISSION_FACTOR_SLUDGE_STORAGE: f64 = 0.017;

/// g co2 / kg solution
pub const EMISSION_FACTOR_FECL3: f64 = 395.0;

/// g co2 / kg solution
pub const EMISSION_FACTOR_FECLSO4: f64 = 76.0;

/// g co2 / kg solution
pub const EMISSION_FACTOR_CAOH2: f64 = 1_055.3;

/// g co2 / kg solution
pub const EMISSION_FACTOR_POLYMERS: f64 = 2_200.0;

pub const GWP_N2O: f64 = 273.0;
pub const GWP_CH4: f64 = 28.0;

/// kg/m^3 for standard cubic meters (GESTIS substance database)
pub const CONVERSION_FACTOR_CH4_M3_TO_KG: f64 = 0.7175;

/// kg co2/l
pub const EMISSION_FACTOR_DIESEL: f64 = 3.24;

/// l/tkm
pub const FUEL_CONSUMPTION: f64 = 0.033;

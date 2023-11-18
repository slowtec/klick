#[derive(Debug, Clone, Copy, PartialEq)]
pub enum N2oEmissionFactorCalcMethod {
    ExtrapolatedParravicini,
    Optimistic,
    Pesimistic,
    Ipcc2019,
    CustomFactor(f64),
}

#[derive(Debug, Clone)]
pub struct InputData {
    pub plant_name: Option<String>,
    pub ew: f64,
    pub abwasser: f64,
    pub n_ges_zu: f64,
    pub csb_ab: f64,
    pub n_ges_ab: f64,
    pub klaergas_gesamt: f64,
    pub methangehalt: f64,
    pub strombedarf: f64,
    pub energie_eigen: f64,
    pub ef_co2_strommix: f64,
    pub schlammtaschen: bool,
    pub schlammstapel: bool,
    pub klaerschlamm_transport_km: f64,
    pub klaerschlamm_entsorgung_m: f64,
    pub betriebsstoffe_fe3: f64,
    pub betriebsstoffe_feso4: f64,
    pub betriebsstoffe_kalk: f64,
    pub betriebsstoffe_poly: f64,
}

#[derive(Debug, Clone)]
pub struct OutputData {
    pub co2eq_n2o_anlage: f64,
    pub co2eq_n2o_gewaesser: f64,
    pub co2eq_ch4_klaerprozes: f64,
    pub co2eq_ch4_schlammstapel: f64,
    pub co2eq_ch4_schlammtasche: f64,
    pub co2eq_ch4_gewaesser: f64,
    pub co2eq_ch4_bhkw: f64,
    pub co2eq_betriebsstoffe_fe3: f64,
    pub co2eq_betriebsstoffe_feso4: f64,
    pub co2eq_betriebsstoffe_kalk: f64,
    pub co2eq_betriebsstoffe_poly: f64,
    pub co2eq_strommix: f64,
    pub co2eq_betriebsstoffe: f64,
    pub co2eq_klaerschlamm_transport: f64,
    pub direkte_emissionen_co2_eq: f64,
    pub indirekte_emissionen_co2_eq: f64,
    pub weitere_indirekte_emissionen_co2_eq: f64,
    pub emissionen_co2_eq: f64,
    pub ef_n2o_anlage: f64,
}

// Emissionsfaktoren
const EF_CH4_ANLAGE: f64 = 230.0; // [g ch4/(ew*a)]
const EF_CH4_GEWAESSER: f64 = 0.009; // [0,9 % des csb-ablauf]
const EF_CH4_BHKW: f64 = 1.164; // [1,164 g ch4/kwh]

const EF_N2O_GEWAESSER: f64 = 0.005; // [0,5 % des Ges-N Ablauf]

// Emissionsfaktoren Schlupf
const EF_SCHLAMMTASCHE: f64 = 0.003; // [0.3 % des Gesamtmethangasertrags]
const EF_SCHLAMMSTAPEL: f64 = 0.017; // [1,7 % der Gesamtfaulgaserzeugung]

// Emissionsfaktoren Betriebsstoffe
const EF_FE3: f64 = 395.0; // [g co2/kg Lösung]
const EF_FESO4: f64 = 76.0; // [g co2/kg Lösung]
const EF_KALK: f64 = 1055.3; // [g co2/kg Lösung]
const EF_POLY: f64 = 2200.0; // [g co2/kg Lösung]

// Umrechnungsfaktoren
const GWP_N2O: f64 = 273.0;
const GWP_CH4: f64 = 28.0;
const UF_N_ZU_N2O: f64 = 44.0 / 28.0;
const UF_CH4_M3_ZU_KG: f64 = 0.7175; // [kg/m^3] für Normkubikmeter (GESTIS Stoffdatenbank)

// Emissionsfaktoren Transport per LKW
const EF_DIESEL: f64 = 3.24; // kg co2/l
const VERBRAUCH: f64 = 0.033; // l/tkm

#[must_use]
#[allow(clippy::too_many_lines)]
pub fn calc(input: &InputData, n2o_szenario: N2oEmissionFactorCalcMethod) -> OutputData {
    let InputData {
        plant_name: _,
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
    } = input;

    let klaergas_methangehalt = methangehalt / 100.0; // [%]
    let energie_fremd = strombedarf - energie_eigen; // [kwh/a]

    let n_elim = (n_ges_zu - n_ges_ab) / n_ges_zu * 100.0; // [%]

    let ef_n2o_anlage = match n2o_szenario {
        N2oEmissionFactorCalcMethod::ExtrapolatedParravicini => get_n2oef(n_elim), // [Berechnung nach Parravicini et al. 2016]
        N2oEmissionFactorCalcMethod::Optimistic => 0.003, // [0,3 % des Ges-N Zulauf]
        N2oEmissionFactorCalcMethod::Pesimistic => 0.008, // [0,8 % des Ges-N Zulauf]
        N2oEmissionFactorCalcMethod::Ipcc2019 => 0.016,   // [1,6 % des Ges-N Zulauf]
        N2oEmissionFactorCalcMethod::CustomFactor(factor) => factor,
    };

    // Direkte emissionen

    // Lachgas
    let n2o_anlage =
        n_ges_zu / f64::from(10_i32.pow(9)) * abwasser * 1_000.0 * ef_n2o_anlage * UF_N_ZU_N2O; // [t N2O/a]
    let n2o_gewaesser =
        n_ges_ab / f64::from(10_i32.pow(9)) * abwasser * 1_000.0 * EF_N2O_GEWAESSER * UF_N_ZU_N2O; // [t N2O/a]

    // Methan
    let ch4_klaerprozess = ew * EF_CH4_ANLAGE / f64::from(10_i32.pow(6)); // [t CH4/a]
    let ch4_gewaesser =
        csb_ab / f64::from(10_i32.pow(9)) * abwasser * f64::from(10_i32.pow(3)) * EF_CH4_GEWAESSER; // [t CH4/a]
    let ch4_bhkw = energie_eigen * EF_CH4_BHKW / f64::from(10_i32.pow(6)); // [t CH4/a]
    let mut ch4_schlupf_schlammstapel = 0.0;
    let mut ch4_schlupf_schlammtasche = 0.0;

    if *schlammtaschen {
        ch4_schlupf_schlammtasche =
            klaergas_gesamt * klaergas_methangehalt * EF_SCHLAMMTASCHE * UF_CH4_M3_ZU_KG / 1_000.0;
        // [t CH4 / a]
    }

    if *schlammstapel {
        ch4_schlupf_schlammstapel =
            klaergas_gesamt * klaergas_methangehalt * EF_SCHLAMMSTAPEL * UF_CH4_M3_ZU_KG / 10_000.0;
        // [t CH4 / a]
    }

    // co2eq
    let co2eq_n2o_anlage = n2o_anlage * GWP_N2O; // [t co2eq/a]
    let co2eq_n2o_gewaesser = n2o_gewaesser * GWP_N2O; // [t co2eq/a]

    let co2eq_ch4_klaerprozes = ch4_klaerprozess * GWP_CH4; // [t co2eq/a]
    let co2eq_ch4_schlammstapel = ch4_schlupf_schlammstapel * GWP_CH4; // [t co2eq/a]
    let co2eq_ch4_schlammtasche = ch4_schlupf_schlammtasche * GWP_CH4; // [t co2eq/a]
    let co2eq_ch4_gewaesser = ch4_gewaesser * GWP_CH4; // [t co2eq/a]
    let co2eq_ch4_bhkw = ch4_bhkw * GWP_CH4; // [t co2eq/a]

    let co2eq_strommix = energie_fremd * ef_co2_strommix / f64::from(10_i32.pow(6)); // [t co2eq/a]

    let co2eq_betriebsstoffe_poly = betriebsstoffe_poly * EF_POLY / f64::from(10_i32.pow(6)); // [t co2eq/a]
    let co2eq_betriebsstoffe_fe3 = betriebsstoffe_fe3 * EF_FE3 / f64::from(10_i32.pow(6)); // [t co2eq/a]
    let co2eq_betriebsstoffe_feso4 = betriebsstoffe_feso4 * EF_FESO4 / f64::from(10_i32.pow(6)); // [t co2eq/a]
    let co2eq_betriebsstoffe_kalk = betriebsstoffe_kalk * EF_KALK / f64::from(10_i32.pow(6)); // [t co2eq/a]
    let co2eq_betriebsstoffe = co2eq_betriebsstoffe_poly +                  // [t co2eq/a]
                            co2eq_betriebsstoffe_feso4 +
                            co2eq_betriebsstoffe_kalk +
                            co2eq_betriebsstoffe_fe3;

    let co2eq_klaerschlamm_transport =
        klaerschlamm_entsorgung_m * klaerschlamm_transport_km * VERBRAUCH * EF_DIESEL / 1_000.0; // [t co2eq/a]

    // Interimsgrößen
    let direkte_emissionen_co2_eq = co2eq_n2o_anlage
        + co2eq_n2o_gewaesser
        + co2eq_ch4_klaerprozes
        + co2eq_ch4_gewaesser
        + co2eq_ch4_bhkw
        + co2eq_ch4_schlammstapel
        + co2eq_ch4_schlammtasche;
    let indirekte_emissionen_co2_eq = co2eq_strommix;
    let weitere_indirekte_emissionen_co2_eq = co2eq_betriebsstoffe + co2eq_klaerschlamm_transport;
    let nutzung_co2_eq = direkte_emissionen_co2_eq
        + indirekte_emissionen_co2_eq
        + weitere_indirekte_emissionen_co2_eq;
    let emissionen_co2_eq = nutzung_co2_eq;

    OutputData {
        co2eq_n2o_anlage,
        co2eq_n2o_gewaesser,
        co2eq_ch4_klaerprozes,
        co2eq_ch4_schlammstapel,
        co2eq_ch4_schlammtasche,
        co2eq_ch4_gewaesser,
        co2eq_ch4_bhkw,
        co2eq_betriebsstoffe_fe3,
        co2eq_betriebsstoffe_feso4,
        co2eq_betriebsstoffe_kalk,
        co2eq_betriebsstoffe_poly,
        co2eq_strommix,
        co2eq_betriebsstoffe,
        co2eq_klaerschlamm_transport,
        direkte_emissionen_co2_eq,
        indirekte_emissionen_co2_eq,
        weitere_indirekte_emissionen_co2_eq,
        emissionen_co2_eq,
        ef_n2o_anlage,
    }
}

fn get_n2oef(n_elim: f64) -> f64 {
    let mut ef = (-0.049 * n_elim + 4.553) / 100.0;
    if ef < 0.0 {
        ef = 0.002;
    }
    ef
}

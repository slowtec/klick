#[derive(Debug, Clone)]
pub struct InputData {
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
    pub n2o_szenario: usize,
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
}

pub fn calc(input: &InputData) -> OutputData {
    let InputData {
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
        n2o_szenario,
    } = input;

    let klaergas_methangehalt = methangehalt / 100.0; // [%]
    let energie_fremd = strombedarf - energie_eigen; // [kwh/a]

    let n_elim = (n_ges_zu - n_ges_ab) / n_ges_zu * 100.0; // [%]

    // emissionsfaktoren
    let ef_ch4_anlage = 230.0; // [g ch4/(ew*a)]
    let ef_ch4_gewaesser = 0.009; // [0,9 % des csb-ablauf]
    let ef_ch4_bhkw = 1.124; // [1,124 g ch4/kwh]

    let ef_n2o_anlage = match n2o_szenario {
        0 => get_n2oef(n_elim), // [Berechnung nach Parravicini et al. 2016]
        1 => 0.005,             // [0,5 % des Ges-N Zulauf]
        2 => 0.016,             // [1,6 % des Ges-N Zulauf]
        3 => 0.032,             // [3,2 % des Ges-N Zulauf]
        _ => panic!("invalid input"),
    };

    let ef_n2o_gewaesser = 0.005; // [0,5 % des Ges-N Ablauf]

    // Emissionsfaktoren Schlupf
    let ef_schlammtasche = 0.003; // [0.3 % des Gesamtmethangasertrags]
    let ef_schlammstapel = 0.017; // [1,7 % der Gesamtfaulgaserzeugung]

    // emissionsfaktoren Betriebsstoffe
    let ef_fe3 = 395.0; // [g co2/kg Lösung]
    let ef_feso4 = 76.0; // [g co2/kg Lösung]
    let ef_kalk = 1055.3; // [g co2/kg Lösung]
    let ef_poly = 2200.0; // [g co2/kg Lösung]

    // Umrechnungsfaktoren
    let gwp_n2o = 265.0;
    let gwp_ch4 = 28.0;
    let uf_n_zu_n2o = 44.0 / 28.0;
    let uf_ch4_m3_zu_kg = 0.7175; // [kg/m^3] für Normkubikmeter (GESTIS Stoffdatenbank)

    // emissionsfaktoren Transport per LKW
    let ef_diesel = 3.24; // kg co2/l
    let verbrauch = 0.033; // l/tkm

    // Direkte emissionen

    // Lachgas
    let n2o_anlage =
        n_ges_zu / (10_i32.pow(9) as f64) * abwasser * 1000.0 * ef_n2o_anlage * uf_n_zu_n2o; // [t N2O/a]
    let n2o_gewaesser =
        n_ges_ab / (10_i32.pow(9) as f64) * abwasser * 1000.0 * ef_n2o_gewaesser * uf_n_zu_n2o; // [t N2O/a]

    // Methan
    let ch4_klaerprozess = ew * ef_ch4_anlage / (10_i32.pow(6) as f64); // [t CH4/a]
    let ch4_gewaesser =
        csb_ab / (10_i32.pow(9) as f64) * abwasser * (10_i32.pow(3) as f64) * ef_ch4_gewaesser; // [t CH4/a]
    let ch4_bhkw = energie_eigen * ef_ch4_bhkw / (10_i32.pow(6) as f64); // [t CH4/a]
    let mut ch4_schlupf_schlammstapel = 0.0;
    let mut ch4_schlupf_schlammtasche = 0.0;

    if *schlammtaschen {
        ch4_schlupf_schlammtasche =
            klaergas_gesamt * klaergas_methangehalt * ef_schlammtasche * uf_ch4_m3_zu_kg / 1_000.0;
        // [t CH4 / a]
    }

    if *schlammstapel {
        ch4_schlupf_schlammstapel =
            klaergas_gesamt * klaergas_methangehalt * ef_schlammstapel * uf_ch4_m3_zu_kg / 10_000.0;
        // [t CH4 / a]
    }

    // co2eq
    let co2eq_n2o_anlage = n2o_anlage * gwp_n2o; // [t co2eq/a]
    let co2eq_n2o_gewaesser = n2o_gewaesser * gwp_n2o; // [t co2eq/a]

    let co2eq_ch4_klaerprozes = ch4_klaerprozess * gwp_ch4; // [t co2eq/a]
    let co2eq_ch4_schlammstapel = ch4_schlupf_schlammstapel * gwp_ch4; // [t co2eq/a]
    let co2eq_ch4_schlammtasche = ch4_schlupf_schlammtasche * gwp_ch4; // [t co2eq/a]
    let co2eq_ch4_gewaesser = ch4_gewaesser * gwp_ch4; // [t co2eq/a]
    let co2eq_ch4_bhkw = ch4_bhkw * gwp_ch4; // [t co2eq/a]

    let co2eq_strommix = energie_fremd * ef_co2_strommix / (10_i32.pow(6) as f64); // [t co2eq/a]

    let co2eq_betriebsstoffe_poly = betriebsstoffe_poly * ef_poly / (10_i32.pow(6) as f64); // [t co2eq/a]
    let co2eq_betriebsstoffe_fe3 = betriebsstoffe_fe3 * ef_fe3 / (10_i32.pow(6) as f64); // [t co2eq/a]
    let co2eq_betriebsstoffe_feso4 = betriebsstoffe_feso4 * ef_feso4 / (10_i32.pow(6) as f64); // [t co2eq/a]
    let co2eq_betriebsstoffe_kalk = betriebsstoffe_kalk * ef_kalk / (10_i32.pow(6) as f64); // [t co2eq/a]
    let co2eq_betriebsstoffe = co2eq_betriebsstoffe_poly +                  // [t co2eq/a]
                            co2eq_betriebsstoffe_feso4 +
                            co2eq_betriebsstoffe_kalk +
                            co2eq_betriebsstoffe_fe3;

    let co2eq_klaerschlamm_transport =
        klaerschlamm_entsorgung_m * klaerschlamm_transport_km * verbrauch * ef_diesel / 1000.0; // [t co2eq/a]

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
    }
}

fn get_n2oef(n_elim: f64) -> f64 {
    let mut ef = (-0.049 * n_elim + 4.553) / 100.0;
    if ef < 0.0 {
        ef = 0.002;
    }
    ef
}

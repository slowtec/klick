#![warn(clippy::pedantic)]

#[cfg(feature = "conversion")]
mod conversion;

use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, AsRefStr, Serialize, Deserialize)]
pub enum ValueId {
    Name,
    Ew,
    Flow,
    CsbZu,
    TknZu,
    PZu,
    CsbAb,
    TknAb,
    PAb,
    Klaergas,
    Methangehalt,
    GasZusatz,
    Biogas,
    Strombedarf,
    Eigenstrom,
    EfStrommix,
    Schlammtaschen,
    Schlammstapel,
    KlaerschlammEnstorgung,
    KlaerschlammTransport,
    BetriebsstoffeFe3,
    BetriebsstoffeFeso4,
    BetriebsstoffeKalk,
    BetriebsstoffePoly,
    N2oSzenario,
    CustomN2oSzenario,
}

#[derive(Debug, Clone, Copy, PartialEq, EnumIter, Serialize, Deserialize)]
pub enum N2OSzenario {
    ExtrapolatedParravicini,
    Optimistic,
    Pesimistic,
    Ipcc2019,
    Custom(f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub n2o_szenario: N2OSzenario,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct FieldSet<ID> {
    pub title: &'static str,
    pub fields: Vec<Field<ID>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Field<ID> {
    pub id: ID,
    pub label: &'static str,
    pub description: Option<&'static str>,
    pub required: bool,
    pub field_type: FieldType,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MinMax {
    pub min: Option<f64>,
    pub max: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FieldType {
    Float {
        initial_value: Option<f64>,
        placeholder: Option<&'static str>,
        plausible: MinMax,
        unreasonable: MinMax,
        unit: &'static str,
    },
    Text {
        initial_value: Option<String>,
        placeholder: Option<&'static str>,
        max_len: Option<usize>,
    },
    Bool {
        initial_value: Option<bool>,
    },
    Selection {
        initial_value: Option<usize>,
        options: Vec<SelectOption>,
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SelectOption {
    pub label: &'static str,
    pub value: usize,
}

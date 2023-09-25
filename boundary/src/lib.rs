use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FormData {
    pub name: String,
    pub ew: String,
    pub flow: String,
    pub csb_zu: String,
    pub tkn_zu: String,
    pub p_zu: String,
    pub csb_ab: String,
    pub tkn_ab: String,
    pub p_ab: String,
    pub klaergas: String,
    pub methangehalt: String, // Form(65)
    pub gas_zusatz: String,
    pub biogas: Option<String>,
    pub strombedarf: String,
    pub eigenstrom: String,
    pub ef_strommix: String, // Form(485)
    pub schlammtaschen: Option<String>,
    pub schlammstapel: Option<String>,
    pub klaerschlamm_enstorgung: String,
    pub klaerschlamm_transport: String,
    pub betriebsstoffe_fe3: String,
    pub betriebsstoffe_feso4: String,
    pub betriebsstoffe_kalk: String,
    pub betriebsstoffe_poly: String,
    pub n2o_szenario: String,
}

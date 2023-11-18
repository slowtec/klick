use serde::{Deserialize, Serialize};
use strum::AsRefStr;

use crate::forms;

type FieldSet = forms::FieldSet<FieldId>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, AsRefStr, Serialize, Deserialize)]
pub enum FieldId {
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
    CustomN2oScenarioSupport,
    CustomN2oScenarioValue,
}

const JSON_FIELD_SETS: &[u8] = include_bytes!("form_field_sets.json");

pub fn field_sets() -> Vec<FieldSet> {
    serde_json::from_slice(JSON_FIELD_SETS).unwrap()
}

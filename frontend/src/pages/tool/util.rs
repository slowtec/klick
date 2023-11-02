use thiserror::Error;

use klick_boundary::N2OSzenario;

pub fn n2o_szenario_to_usize(szenario: N2OSzenario) -> usize {
    match szenario {
        N2OSzenario::ExtrapolatedParravicini => 0,
        N2OSzenario::Optimistic => 1,
        N2OSzenario::Pesimistic => 2,
        N2OSzenario::Ipcc2019 => 3,
    }
}

#[derive(Debug, Error)]
#[error("Invalid N2O szenario")]
pub struct InvalidN2OSzenario;

pub fn try_n2o_szenario_from_usize(szenario: usize) -> Result<N2OSzenario, InvalidN2OSzenario> {
    let szenario = match szenario {
        0 => N2OSzenario::ExtrapolatedParravicini,
        1 => N2OSzenario::Optimistic,
        2 => N2OSzenario::Pesimistic,
        3 => N2OSzenario::Ipcc2019,
        _ => return Err(InvalidN2OSzenario),
    };
    Ok(szenario)
}

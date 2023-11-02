use thiserror::Error;

use klick_boundary::N2OSzenario;

pub fn n2o_szenario_to_usize(szenario: N2OSzenario) -> usize {
    match szenario {
        N2OSzenario::ExtrapolatedParravicini => 0,
        N2OSzenario::Optimistic => 1,
        N2OSzenario::Pesimistic => 2,
        N2OSzenario::Ipcc2019 => 3,
        N2OSzenario::Custom(_) => 4,
    }
}

#[derive(Debug, Error)]
#[error("Invalid N2O szenario")]
pub struct InvalidN2OSzenario;

pub fn try_n2o_szenario_from_usize(
    szenario: usize,
    custom_value: Option<f64>,
) -> Result<N2OSzenario, InvalidN2OSzenario> {
    let szenario = match szenario {
        0 => N2OSzenario::ExtrapolatedParravicini,
        1 => N2OSzenario::Optimistic,
        2 => N2OSzenario::Pesimistic,
        3 => N2OSzenario::Ipcc2019,
        4 => custom_value
            .map(|value| {
                value / 100.0 // convert user input to percent value
            })
            .map(N2OSzenario::Custom)
            .ok_or(InvalidN2OSzenario)?,
        _ => return Err(InvalidN2OSzenario),
    };
    Ok(szenario)
}

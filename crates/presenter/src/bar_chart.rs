use klick_domain::CO2Equivalents;

#[must_use]
pub fn recommendation_diff_bar_chart(
    old: CO2Equivalents,
    new: CO2Equivalents,
) -> Vec<(&'static str, f64, Option<f64>)> {
    let diff = new.clone() - old;
    let total_emissions = f64::from(new.total_emissions);

    [
        ("CH₄ Schlupf Schlammtasche", f64::from(diff.ch4_sludge_bags)),
        (
            "CH₄ Schlupf Schlammlagerung",
            f64::from(diff.ch4_sludge_storage_containers),
        ),
        ("CH₄ Anlage (unspez.)", f64::from(diff.ch4_plant)),
        (
            "N₂O Prozesswasserbehandlung",
            f64::from(diff.n2o_side_stream),
        ),
        (
            "Fossile Energiequellen",
            -1.0 * f64::from(diff.fossil_energy_savings),
        ),
        ("Prozesse", -1.0 * f64::from(diff.process_energy_savings)),
        (
            "Erneurbare Energien",
            -1.0 * (f64::from(new.photovoltaic_expansion_savings)
                + f64::from(new.wind_expansion_savings)
                + f64::from(new.water_expansion_savings)),
        ),
        ("Abwärme", -1.0 * f64::from(diff.district_heating_savings)),
        ("Emissionen", f64::from(diff.total_emissions)),
    ]
    .into_iter()
    .map(|(label, value)| {
        let percentage = Some(value / total_emissions * 100.0);
        (label, value, percentage)
    })
    .collect()
}

#[must_use]
pub fn sensitivity_diff_bar_chart(
    old: CO2Equivalents,
    new: CO2Equivalents,
) -> Vec<(&'static str, f64, Option<f64>)> {
    let diff = new.clone() - old;
    let total_emissions = f64::from(new.total_emissions);
    [
        ("N₂O Anlage", f64::from(diff.n2o_plant)),
        ("CH₄ Schlammtasche", f64::from(diff.ch4_sludge_bags)),
        (
            "CH₄ Schlammlagerung",
            f64::from(diff.ch4_sludge_storage_containers),
        ),
        ("CH₄ Anlage (unspez.)", f64::from(diff.ch4_plant)),
        (
            "CH₄ BHKW",
            f64::from(diff.ch4_combined_heat_and_power_plant),
        ),
        ("Fossiles CO₂", f64::from(diff.fossil_emissions)),
        ("N₂O Prozesswasser", f64::from(diff.n2o_side_stream)),
        ("Emissionen", f64::from(diff.total_emissions)),
    ]
    .into_iter()
    .map(|(label, value)| {
        let percentage = Some(value / total_emissions * 100.0);
        (label, value, percentage)
    })
    .collect()
}

use charming::{
    //element::{Emphasis, EmphasisFocus},
    series::Sankey,
    Chart,
};
// use log::info;

use klick_application as app;

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
fn format_large_number(f: f64) -> String {
    // Convert the f64 to u64
    let t = f.ceil();
    let u = t as u64;

    // Format the u64 as a string with a comma
    let formatted_string = format!("{u:0}");
    println!("formatted_string {formatted_string}");

    // Insert a comma at the appropriate position
    let comma_separated_string = formatted_string
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            if i > 0 && i % 3 == 0 {
                format!(".{c}")
            } else {
                c.to_string()
            }
        })
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>();

    comma_separated_string
}

#[allow(clippy::too_many_lines, clippy::needless_pass_by_value)]
pub fn render(output_data: app::Output, element_id: &str) {
    let app::Output {
        co2_equivalents,
        n2o_emission_factor: _,
    } = output_data;

    let app::CO2Equivalents {
        n2o_plant,
        n2o_water,
        ch4_sewage_treatment,
        ch4_sludge_storage_containers,
        ch4_sludge_bags,
        ch4_water,
        ch4_combined_heat_and_power_plant,
        fecl3,
        feclso4,
        caoh2,
        synthetic_polymers,
        electricity_mix,
        operating_materials,
        sewage_sludge_transport,
        emissions,
        direct_emissions,
        indirect_emissions,
        other_indirect_emissions,
    } = co2_equivalents;

    let dir_em = "Direkte Emissionen".to_string();
    let dir_em_string = format!("{dir_em} {}", format_large_number(direct_emissions));

    let indir_em = "Indirekte Emissionen".to_string();
    let indir_em_string = format!("{indir_em} {}", format_large_number(indirect_emissions));

    let wei_indir_em = "Weitere Indirekte Emissionen".to_string();
    let wei_indir_em_string = format!(
        "{wei_indir_em} {}",
        format_large_number(other_indirect_emissions)
    );

    // TODO: what's the difference to emissions?
    let nu = "Nutzung";
    let nu_string = format!("{nu} {}", format_large_number(emissions));

    let em = "Emission";
    let em_string = format!("{em} {}", format_large_number(emissions));

    let bs = "Betriebsstoffe";
    let bs_string = format!("{bs} {}", format_large_number(operating_materials));

    let streams: Vec<(_, _, _)> = vec![
        (
            format!("Eisen(III)-chlorid-Lösung {}", format_large_number(fecl3)),
            bs_string.as_str(),
            fecl3,
        ),
        (
            format!("Eisenchloridsulfat-Lösung {}", format_large_number(feclso4)),
            bs_string.as_str(),
            feclso4,
        ),
        (
            format!("Kalkhydrat {}", format_large_number(caoh2)),
            bs_string.as_str(),
            caoh2,
        ),
        (
            format!(
                "Synthetische Polymere {}",
                format_large_number(synthetic_polymers)
            ),
            bs_string.as_str(),
            synthetic_polymers,
        ),
        (
            format!(
                "Klaerschlamm Transport {}",
                format_large_number(sewage_sludge_transport)
            ),
            wei_indir_em_string.as_str(),
            sewage_sludge_transport,
        ),
        (
            format!("N₂O Anlage {}", format_large_number(n2o_plant)),
            dir_em_string.as_str(),
            n2o_plant,
        ),
        (
            format!("N₂O Gewässer {}", format_large_number(n2o_water)),
            dir_em_string.as_str(),
            n2o_water,
        ),
        (
            format!(
                "CH₄ Klärprozess {}",
                format_large_number(ch4_sewage_treatment)
            ),
            dir_em_string.as_str(),
            ch4_sewage_treatment,
        ),
        (
            format!(
                "CH₄ Schlupf Schlammstapel {}",
                format_large_number(ch4_sludge_storage_containers)
            ),
            dir_em_string.as_str(),
            ch4_sludge_storage_containers,
        ),
        (
            format!(
                "CH₄ Schlupf Schlammtasche {}",
                format_large_number(ch4_sludge_bags)
            ),
            dir_em_string.as_str(),
            ch4_sludge_bags,
        ),
        (
            format!("CH₄ Gewässer {}", format_large_number(ch4_water)),
            dir_em_string.as_str(),
            ch4_water,
        ),
        (
            format!(
                "CH₄ BHKW {}",
                format_large_number(ch4_combined_heat_and_power_plant)
            ),
            dir_em_string.as_str(),
            ch4_combined_heat_and_power_plant,
        ),
        (
            format!("Strommix {}", format_large_number(electricity_mix)),
            indir_em_string.as_str(),
            electricity_mix,
        ),
        (
            bs_string.clone().to_string(),
            wei_indir_em_string.as_str(),
            operating_materials,
        ),
        (
            dir_em_string.clone().to_string(),
            nu_string.as_str(),
            direct_emissions,
        ),
        (
            indir_em_string.clone().to_string(),
            nu_string.as_str(),
            indirect_emissions,
        ),
        (
            wei_indir_em_string.clone().to_string(),
            nu_string.as_str(),
            other_indirect_emissions,
        ),
        (nu_string.clone().to_string(), em_string.as_str(), emissions),
    ];

    let mut labels: Vec<_> = vec![];

    for (src, target, value) in &streams {
        for x in [src, *target] {
            let label = x.to_string();
            if value < &0.000001 {
                continue;
            }

            if !labels.contains(&label) {
                labels.push(label);
            }
        }
    }
    // info!("{:?}", labels);
    // info!("{:?}", streams);

    let sankey_data: Vec<_> = labels;
    let sankey_links: Vec<(_, _, f64)> = streams
        .into_iter()
        .map(|(src, target, value)| (src, target.to_string(), value))
        .collect();

    let chart = Chart::new().series(
        Sankey::new()
            //.emphasis(Emphasis::new().focus(EmphasisFocus::Adjacency))
            .layout_iterations(0u64)
            .data(sankey_data)
            .links(sankey_links),
    );
    //log::debug!("Render Sankey chart");
    let renderer = charming::WasmRenderer::new(1200, 800);
    renderer.render(element_id, &chart).unwrap();
}

pub fn clear(element_id: &str) {
    let el = leptos::document().get_element_by_id(element_id).unwrap();
    el.set_inner_html("");
    el.remove_attribute("_echarts_instance_").unwrap();
}

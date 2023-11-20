use charming::{
    element::{ItemStyle, LineStyle},
    series::Sankey,
    Chart,
};
use log::info;

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

    let style_red    = ItemStyle::new().color("red").border_color("black");
    let style_orange = ItemStyle::new().color("#ff7400").border_color("black");
    let style_yellow = ItemStyle::new().color("#ffc100").border_color("black");

    #[derive(Debug, Clone)]
    struct item {
        value: f64,
        name: String,
        itemStyle: ItemStyle,
    }

    // red
    let emission: item = item { value: emissions, name: format!("Emission {}", format_large_number(emissions)), itemStyle: style_red.clone() };
    let nutzung: item = item { value: emissions, name: format!("Nutzung {}", format_large_number(emissions)), itemStyle: style_red.clone() };

    // orange
    let indir_em: item = item { value: indirect_emissions, name: format!("Indirekte Emissionen {}", format_large_number(indirect_emissions)), itemStyle: style_orange.clone() };
    let strommix: item = item { value: electricity_mix, name: format!("Strommix {}", format_large_number(electricity_mix)), itemStyle: style_orange.clone() };

    // yellow
    let wei_indir_em: item = item { value: other_indirect_emissions, name: format!("Weitere Indirekte Emissionen {}", format_large_number(other_indirect_emissions)), itemStyle: style_yellow.clone() };
    let betriebsstoffe: item = item { value: operating_materials, name: format!("Betriebsstoffe {}", format_large_number(operating_materials)), itemStyle: style_yellow.clone() };
    let eischlorsulfatsol: item = item { value: feclso4, name: format!("Eisenchloridsulfat-Lösung {}", format_large_number(feclso4)), itemStyle: style_yellow.clone() };
    let kalkhydrat: item = item { value: caoh2, name: format!("Kalkhydrat {}", format_large_number(caoh2)), itemStyle: style_yellow.clone() };
    let synth_poly: item = item { value: synthetic_polymers, name: format!("Synthetische Polymere {}", format_large_number(synthetic_polymers)), itemStyle: style_yellow.clone() };
    let klaerschl_trans: item = item { value: sewage_sludge_transport, name: format!("Klaerschlamm Transport {}", format_large_number(sewage_sludge_transport)), itemStyle: style_yellow.clone() };

    // red
    let dir_em: item = item { value: direct_emissions, name: format!("Direkte Emissionen {}", format_large_number(direct_emissions)), itemStyle: style_red.clone() };
    let fe3cl: item = item { value: fecl3, name: format!("Eisen(III)-chlorid-Lösung {}", format_large_number(fecl3)), itemStyle: style_red.clone() };
    let n2o_anlage: item = item { value: n2o_plant, name: format!("N₂O Anlage {}", format_large_number(n2o_plant)), itemStyle: style_red.clone() };
    let n2o_gewaesser: item = item { value: n2o_water, name: format!("N₂O Gewässer {}", format_large_number(n2o_water)), itemStyle: style_red.clone() };
    let ch4_klaerprozess: item = item { value: ch4_sewage_treatment, name: format!("CH₄ Klärprozess {}",format_large_number(ch4_sewage_treatment)), itemStyle: style_red.clone() };
    let ch4_schlupf_schlammstapel: item = item { value: ch4_sludge_storage_containers, name: format!("CH₄ Schlupf Schlammstapel {}",format_large_number(ch4_sludge_storage_containers)), itemStyle: style_red.clone() };
    let ch4_schlupf_schlammtasche: item = item { value: ch4_sludge_bags, name: format!("CH₄ Schlupf Schlammtasche {}",format_large_number(ch4_sludge_bags)), itemStyle: style_red.clone() };
    let ch4_gewaesser: item = item { value: ch4_water, name: format!("CH₄ Gewässer {}", format_large_number(ch4_water)), itemStyle: style_red.clone() };
    let ch4_bhkw: item = item { value: ch4_combined_heat_and_power_plant, name: format!("CH₄ BHKW {}",format_large_number(ch4_combined_heat_and_power_plant)), itemStyle: style_red.clone() };


    let streams: Vec<(_, _)> = vec![
        (
            fe3cl.clone(),
            betriebsstoffe.clone(),
        ),
        (
            eischlorsulfatsol.clone(),
            betriebsstoffe.clone(),
        ),
        (
            kalkhydrat.clone(),
            betriebsstoffe.clone(),
        ),
        (
            synth_poly.clone(),
            betriebsstoffe.clone(),
        ),
        (
            klaerschl_trans.clone(),
            wei_indir_em.clone(),
        ),
        (
            n2o_anlage.clone(),
            dir_em.clone(),
        ),
        (
            n2o_gewaesser.clone(),
            dir_em.clone(),
        ),
        (
            ch4_klaerprozess.clone(),
            dir_em.clone(),
        ),
        (
            ch4_schlupf_schlammstapel.clone(),
            dir_em.clone(),
        ),
        (
            ch4_schlupf_schlammtasche.clone(),
            dir_em.clone(),
        ),
        (
            ch4_gewaesser.clone(),
            dir_em.clone(),
        ),
        (
            ch4_bhkw.clone(),
            dir_em.clone(),
        ),
        (
            strommix.clone(),
            indir_em.clone(),
        ),
        (
            betriebsstoffe.clone(),
            wei_indir_em.clone(),
        ),
        (
            dir_em.clone(),
            nutzung.clone(),
        ),
        (
            indir_em.clone(),
            nutzung.clone(),
        ),
        (
            wei_indir_em.clone(),
            nutzung.clone(),
        ),
        (
            nutzung.clone(),
            emission.clone(),
        ),
    ];

    let mut labels: Vec<(String, ItemStyle)> = vec![];

    for (source, target) in &streams {
        for x in [source.name.clone(), target.name.clone()] {
            let label = x.to_string();
            if source.value < 0.000001 {
                continue;
            }

            if !labels.iter().any(|(n, s)| {
                if n == &label {
                    return true;
                }
                false
            }){
                labels.push((label, source.itemStyle.clone()));
            }
        }
    }
    info!("{:?}", labels);
    info!("{:?}", streams);

    let sankey_data: Vec<_> = labels;
    let sankey_links: Vec<(_, _, f64)> = streams
        .into_iter()
        .map(|(src, target)| (src.name.to_string(), target.name.to_string(), src.value))
        .collect();

    let color_style: LineStyle = LineStyle::new().color("source").curveness(0.4);

    let chart = Chart::new().series(
        Sankey::new()
            .layout_iterations(0u64)
            .links(sankey_links)
            .line_style(color_style)
            .data(sankey_data)
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

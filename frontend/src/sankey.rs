use charming::{
    element::{ItemStyle, LineStyle},
    series::Sankey,
    series::SankeyNode,
    Chart,
};

use klick_application as app;

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
fn format_large_number<T>(number: T) -> String
where
    T: Into<f64>,
{
    // Convert the f64 to u64
    let t = number.into().ceil();
    let u = t as u64;

    // Format the u64 as a string with a comma
    let formatted_string = format!("{u:0}");

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

#[derive(Debug, Clone)]
struct SankeyItem {
    value: f64,
    name: String,
    item_style: ItemStyle,
}

#[allow(clippy::too_many_lines, clippy::needless_pass_by_value)]
pub fn render(output_data: app::Output, element_id: &str) {
    log::debug!("Render sankey chart for {output_data:#?}");

    let app::Output {
        co2_equivalents,
        n2o_emission_factor: _,
    } = output_data;

    let app::CO2Equivalents {
        n2o_plant,
        n2o_water,
        n2o_emissions,
        ch4_sewage_treatment,
        ch4_sludge_storage_containers,
        ch4_sludge_bags,
        ch4_water,
        ch4_combined_heat_and_power_plant,
        ch4_emissions,
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
        excess_energy_co2_equivalent,
    } = co2_equivalents;

    let style_red = ItemStyle::new().color("red").border_color("black");
    let style_orange = ItemStyle::new().color("#ff7400").border_color("black");
    let style_yellow = ItemStyle::new().color("#ffc100").border_color("black");

    // red
    let emission: SankeyItem = SankeyItem {
        value: emissions.into(),
        name: format!("Emission {}", format_large_number(emissions)),
        item_style: style_red.clone(),
    };

    // orange
    let indir_em: SankeyItem = SankeyItem {
        value: indirect_emissions.into(),
        name: format!(
            "Indirekte Emissionen {}",
            format_large_number(indirect_emissions)
        ),
        item_style: style_orange.clone(),
    };
    let strommix: SankeyItem = SankeyItem {
        value: electricity_mix.into(),
        name: format!("Strommix {}", format_large_number(electricity_mix)),
        item_style: style_orange.clone(),
    };

    // yellow
    let wei_indir_em: SankeyItem = SankeyItem {
        value: other_indirect_emissions.into(),
        name: format!(
            "Weitere Indirekte Emissionen {}",
            format_large_number(other_indirect_emissions)
        ),
        item_style: style_yellow.clone(),
    };
    let betriebsstoffe: SankeyItem = SankeyItem {
        value: operating_materials.into(),
        name: format!(
            "Betriebsstoffe {}",
            format_large_number(operating_materials)
        ),
        item_style: style_yellow.clone(),
    };
    let fe3cl: SankeyItem = SankeyItem {
        value: fecl3.into(),
        name: format!("Eisen(III)-chlorid-Lösung {}", format_large_number(fecl3)),
        item_style: style_yellow.clone(),
    };
    let eischlorsulfatsol: SankeyItem = SankeyItem {
        value: feclso4.into(),
        name: format!("Eisenchloridsulfat-Lösung {}", format_large_number(feclso4)),
        item_style: style_yellow.clone(),
    };
    let kalkhydrat: SankeyItem = SankeyItem {
        value: caoh2.into(),
        name: format!("Kalkhydrat {}", format_large_number(caoh2)),
        item_style: style_yellow.clone(),
    };
    let synth_poly: SankeyItem = SankeyItem {
        value: synthetic_polymers.into(),
        name: format!(
            "Synthetische Polymere {}",
            format_large_number(synthetic_polymers)
        ),
        item_style: style_yellow.clone(),
    };
    let klaerschl_trans: SankeyItem = SankeyItem {
        value: sewage_sludge_transport.into(),
        name: format!(
            "Klaerschlamm Transport {}",
            format_large_number(sewage_sludge_transport)
        ),
        item_style: style_yellow.clone(),
    };

    // red
    let dir_em: SankeyItem = SankeyItem {
        value: direct_emissions.into(),
        name: format!(
            "Direkte Emissionen {}",
            format_large_number(direct_emissions)
        ),
        item_style: style_red.clone(),
    };
    let lachgas_em: SankeyItem = SankeyItem {
        value: n2o_emissions.into(),
        name: format!("Lachgasemissionen {}", format_large_number(n2o_emissions)),
        item_style: style_red.clone(),
    };
    let methan_em: SankeyItem = SankeyItem {
        value: ch4_emissions.into(),
        name: format!("Methanemissionen {}", format_large_number(ch4_emissions)),
        item_style: style_red.clone(),
    };
    let n2o_anlage: SankeyItem = SankeyItem {
        value: n2o_plant.into(),
        name: format!("N₂O Anlage {}", format_large_number(n2o_plant)),
        item_style: style_red.clone(),
    };
    let n2o_gewaesser: SankeyItem = SankeyItem {
        value: n2o_water.into(),
        name: format!("N₂O Gewässer {}", format_large_number(n2o_water)),
        item_style: style_red.clone(),
    };
    let ch4_klaerprozess: SankeyItem = SankeyItem {
        value: ch4_sewage_treatment.into(),
        name: format!(
            "CH₄ Klärprozess {}",
            format_large_number(ch4_sewage_treatment)
        ),
        item_style: style_red.clone(),
    };
    let ch4_schlupf_schlammstapel: SankeyItem = SankeyItem {
        value: ch4_sludge_storage_containers.into(),
        name: format!(
            "CH₄ Schlupf Schlammstapel {}",
            format_large_number(ch4_sludge_storage_containers)
        ),
        item_style: style_red.clone(),
    };
    let ch4_schlupf_schlammtasche: SankeyItem = SankeyItem {
        value: ch4_sludge_bags.into(),
        name: format!(
            "CH₄ Schlupf Schlammtasche {}",
            format_large_number(ch4_sludge_bags)
        ),
        item_style: style_red.clone(),
    };
    let ch4_gewaesser: SankeyItem = SankeyItem {
        value: ch4_water.into(),
        name: format!("CH₄ Gewässer {}", format_large_number(ch4_water)),
        item_style: style_red.clone(),
    };
    let ch4_bhkw: SankeyItem = SankeyItem {
        value: ch4_combined_heat_and_power_plant.into(),
        name: format!(
            "CH₄ BHKW {}",
            format_large_number(ch4_combined_heat_and_power_plant)
        ),
        item_style: style_red.clone(),
    };

    let streams: Vec<(_, _)> = vec![
        (fe3cl.clone(), betriebsstoffe.clone()),
        (eischlorsulfatsol.clone(), betriebsstoffe.clone()),
        (kalkhydrat.clone(), betriebsstoffe.clone()),
        (synth_poly.clone(), betriebsstoffe.clone()),
        (klaerschl_trans.clone(), wei_indir_em.clone()),
        (n2o_anlage.clone(), lachgas_em.clone()),
        (n2o_gewaesser.clone(), lachgas_em.clone()),
        (lachgas_em.clone(), dir_em.clone()),
        (ch4_klaerprozess.clone(), methan_em.clone()),
        (ch4_schlupf_schlammstapel.clone(), methan_em.clone()),
        (ch4_schlupf_schlammtasche.clone(), methan_em.clone()),
        (ch4_gewaesser.clone(), methan_em.clone()),
        (ch4_bhkw.clone(), methan_em.clone()),
        (methan_em.clone(), dir_em.clone()),
        (strommix.clone(), indir_em.clone()),
        (betriebsstoffe.clone(), wei_indir_em.clone()),
        (dir_em.clone(), emission.clone()),
        (indir_em.clone(), emission.clone()),
        (wei_indir_em.clone(), emission.clone()),
    ];

    let mut labels: Vec<SankeyNode> = vec![];

    for (source, target) in &streams {
        for x in [source.name.clone(), target.name.clone()] {
            let label = x.to_string();
            if source.value < 0.000_001 {
                continue;
            }

            if !labels.iter().any(|s| {
                if s.name == label {
                    return true;
                }
                false
            }) {
                labels.push(SankeyNode::new(label).item_style(source.item_style.clone()));
            }
        }
    }

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
            .data(labels),
    );
    let renderer = charming::WasmRenderer::new(1200, 800);
    renderer.render(element_id, &chart).unwrap();
}

pub fn clear(element_id: &str) {
    let el = leptos::document().get_element_by_id(element_id).unwrap();
    el.set_inner_html("");
    el.remove_attribute("_echarts_instance_").unwrap();
}

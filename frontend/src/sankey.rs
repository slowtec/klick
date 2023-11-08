use charming::{
    element::{Emphasis, EmphasisFocus},
    series::Sankey,
    Chart,
};

// fn format_large_number(number: f64) -> String {
//     let formatted = if number >= 1_000_000_000.0 {
//         format!("{:.2} Gt", number / 1_000_000_000.0)
//     } else if number >= 1_000_000.0 {
//         format!("{:.2} Mt", number / 1_000_000.0)
//     } else if number >= 1_000.0 {
//         format!("{:.2} kt", number / 1_000.0)
//     } else {
//         format!("{:.2} t", number)
//     };
//     formatted.replace(".", ",")
// }

//
// fn format_large_number(number: f64) -> String { // 5963,86 t CO₂-eq/a
//     let formatted = format!("{:.2}", number);
//     formatted.replace(".", ",")
// }

fn format_large_number(f: f64) -> String {
    // Convert the f64 to u64
    let t = f.ceil();
    let u = t as u64;

    // Format the u64 as a string with a comma
    let formatted_string = format!("{:0}", u);
    println!("formatted_string {formatted_string}");

    // Insert a comma at the appropriate position
    let comma_separated_string = formatted_string.chars().rev().enumerate()
        .map(|(i, c)| if i > 0 && i % 3 == 0 { format!(".{}", c) } else { c.to_string() })
        .collect::<String>()
        .chars().rev().collect::<String>();

    comma_separated_string
}

pub fn render(output_data: klick_application::OutputData, element_id: &str) {
    let klick_application::OutputData {
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
        ef_n2o_anlage: _,
    } = output_data;

    let dir_em = "Direkte Emissionen".to_string();
    let dir_em_string = format!("{dir_em} {}", format_large_number(direkte_emissionen_co2_eq));

    let indir_em = "Indirekte Emissionen".to_string();
    let indir_em_string = format!("{indir_em} {}", format_large_number(indirekte_emissionen_co2_eq));
    
    let wei_indir_em = "Weitere Indirekte Emissionen".to_string();
    let wei_indir_em_string = format!("{wei_indir_em} {}", format_large_number(weitere_indirekte_emissionen_co2_eq));

    let nu = "Nutzung";
    let nu_string =  format!("{nu} {}", format_large_number(emissionen_co2_eq));

    let em = "Emission";
    let em_string = format!("{em} {}", format_large_number(emissionen_co2_eq));

    let bs = "Betriebsstoffe";
    let bs_string = format!("{bs} {}", format_large_number(co2eq_betriebsstoffe));

    let streams: Vec<(_, _, _)> = vec![
        (
            format!("Eisen(III)-chlorid-Lösung {}", format_large_number(co2eq_betriebsstoffe_fe3)),
            bs_string.as_str(),
            co2eq_betriebsstoffe_fe3,
        ),
        (
            format!("Eisenchloridsulfat-Lösung {}", format_large_number(co2eq_betriebsstoffe_feso4)),
            bs_string.as_str(),
            co2eq_betriebsstoffe_feso4,
        ),
        (
            format!("Kalkhydrat {}", format_large_number(co2eq_betriebsstoffe_kalk)),
            bs_string.as_str(),
            co2eq_betriebsstoffe_kalk,
        ),
        (
            format!("Synthetische Polymere {}", format_large_number(co2eq_betriebsstoffe_poly)),
            bs_string.as_str(),
            co2eq_betriebsstoffe_poly,
        ),
        (
            format!("Klaerschlamm Transport {}", format_large_number(co2eq_klaerschlamm_transport)),
            wei_indir_em_string.as_str(),
            co2eq_klaerschlamm_transport,
        ),
        (
            format!("N₂O Anlage {}", format_large_number(co2eq_n2o_anlage)),
            dir_em_string.as_str(),
            co2eq_n2o_anlage,
        ),
        (
            format!("N₂O Gewässer {}", format_large_number(co2eq_n2o_gewaesser)),
            dir_em_string.as_str(),
            co2eq_n2o_gewaesser,
        ),
        (
            format!("CH₄ Klärprozess {}", format_large_number(co2eq_ch4_klaerprozes)),
            dir_em_string.as_str(),
            co2eq_ch4_klaerprozes,
        ),
        (
            format!("CH₄ Schlupf Schlammstapel {}", format_large_number(co2eq_ch4_schlammstapel)),
            dir_em_string.as_str(),
            co2eq_ch4_schlammstapel,
        ),
        (
            format!("CH₄ Schlupf Schlammtasche {}", format_large_number(co2eq_ch4_schlammtasche)),
            dir_em_string.as_str(),
            co2eq_ch4_schlammtasche,
        ),
        (
            format!("CH₄ Gewässer {}", format_large_number(co2eq_ch4_gewaesser)),
            dir_em_string.as_str(),
            co2eq_ch4_gewaesser,
        ),
        (
            format!("CH₄ BHKW {}", format_large_number(co2eq_ch4_bhkw)),
            dir_em_string.as_str(),
            co2eq_ch4_bhkw,
        ),
        (
            format!("Strommix {}", format_large_number(co2eq_strommix)),
            indir_em_string.as_str(),
            co2eq_strommix,
        ),
        (
            format!("{}", bs_string.clone()),
            wei_indir_em_string.as_str(),
            co2eq_betriebsstoffe,
        ),
        (
            format!("{}",dir_em_string.clone()),
            nu_string.as_str(),
            direkte_emissionen_co2_eq,
        ),
        (
            format!("{}",indir_em_string.clone()),
            nu_string.as_str(),
            indirekte_emissionen_co2_eq,
        ),
        (
            format!("{}",wei_indir_em_string.clone()),
            nu_string.as_str(),
            weitere_indirekte_emissionen_co2_eq,
        ),
        (
            format!("{}",nu_string.clone()),
            em_string.as_str(),
            emissionen_co2_eq
        ),
    ];

    let mut labels: Vec<_> = vec![];

    for (src, target, _) in &streams {
        for x in [&*src, *target] {
            let label = x.to_string();

            if !labels.contains(&label) {
                //info!("Label {} added", label);
                labels.push(label);
            } else {
                //info!("Label {} already exists", label);
            }
        }
    }

    let sankey_data: Vec<_> = labels;
    let sankey_links: Vec<(_, _, f64)> = streams
        .into_iter()
        .map(|(src, target, value)| (src, target.to_string(), value))
        .collect();

    let chart = Chart::new().series(
        Sankey::new()
            .emphasis(Emphasis::new().focus(EmphasisFocus::Adjacency))
            .data(sankey_data)
            .links(sankey_links),
    );
    log::debug!("Render Sankey chart");
    //info!("{}", chart.to_string());
    let renderer = charming::WasmRenderer::new(1200, 800);
    renderer.render(element_id, &chart).unwrap();
}

pub fn clear(element_id: &str) {
    let el = leptos::document().get_element_by_id(element_id).unwrap();
    el.set_inner_html("");
    el.remove_attribute("_echarts_instance_").unwrap();
}

use charming::{
    component::Title,
    element::{Emphasis, EmphasisFocus},
    series::Sankey,
    Chart,
};

pub fn render(name: &str, ew: f64, output_data: klick_application::OutputData, element_id: &str) {
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
    } = output_data;

    let dir_em = "Direkte Emissionen";
    let indir_em = "Indirekte Emissionen";
    let wei_indir_em = "Weitere Indirekte Emissionen";
    let nu = "Nutzung";
    let em = "Emission";

    let streams: Vec<(_, _, _)> = vec![
        (
            format!("Eisen(III)-chlorid-Lösung\n{co2eq_betriebsstoffe_fe3}"),
            "Betriebsstoffe",
            co2eq_betriebsstoffe_fe3,
        ),
        (
            format!("Eisenchloridsulfat-Lösung\n{co2eq_betriebsstoffe_feso4}"),
            "Betriebsstoffe",
            co2eq_betriebsstoffe_feso4,
        ),
        (
            format!("Kalkhydrat\n{co2eq_betriebsstoffe_kalk}"),
            "Betriebsstoffe",
            co2eq_betriebsstoffe_kalk,
        ),
        (
            format!("Synthetische Polymere\n{co2eq_betriebsstoffe_poly}"),
            "Betriebsstoffe",
            co2eq_betriebsstoffe_poly,
        ),
        (
            format!("Klaerschlamm Transport\n{co2eq_klaerschlamm_transport}"),
            wei_indir_em,
            co2eq_klaerschlamm_transport,
        ),
        (
            format!("N₂O Anlage\n{co2eq_n2o_anlage}"),
            dir_em,
            co2eq_n2o_anlage,
        ),
        (
            format!("N₂O Gewässer\n{co2eq_n2o_gewaesser}"),
            dir_em,
            co2eq_n2o_gewaesser,
        ),
        (
            format!("CH₄ Klärprozess\n{co2eq_ch4_klaerprozes}"),
            dir_em,
            co2eq_ch4_klaerprozes,
        ),
        (
            format!("CH₄ Schlupf Schlammstapel\n{co2eq_ch4_schlammstapel}"),
            dir_em,
            co2eq_ch4_schlammstapel,
        ),
        (
            format!("CH₄ Schlupf Schlammtasche\n{co2eq_ch4_schlammtasche}"),
            dir_em,
            co2eq_ch4_schlammtasche,
        ),
        (
            format!("CH₄ Gewässer\n{co2eq_ch4_gewaesser}"),
            dir_em,
            co2eq_ch4_gewaesser,
        ),
        (
            format!("CH₄ BHKW\n{co2eq_ch4_bhkw}"),
            dir_em,
            co2eq_ch4_bhkw,
        ),
        (
            format!("Strommix\n{co2eq_strommix}"),
            indir_em,
            co2eq_strommix,
        ),
        (
            format!("Betriebsstoffe\n{co2eq_betriebsstoffe}"),
            wei_indir_em,
            co2eq_betriebsstoffe,
        ),
        (
            format!("{dir_em}\n{direkte_emissionen_co2_eq}"),
            nu,
            direkte_emissionen_co2_eq,
        ),
        (
            format!("{indir_em}\n{indirekte_emissionen_co2_eq}"),
            nu,
            indirekte_emissionen_co2_eq,
        ),
        (
            format!("{wei_indir_em}\n{weitere_indirekte_emissionen_co2_eq}"),
            nu,
            weitere_indirekte_emissionen_co2_eq,
        ),
        (format!("{nu}\n{emissionen_co2_eq}"), em, emissionen_co2_eq),
    ];

    let mut labels: Vec<_> = vec![];

    for (src, target, _) in &streams {
        for x in [&*src, *target] {
            let label = x.to_string();
            if !labels.contains(&label) {
                labels.push(label);
            }
        }
    }

    let einheit = "t CO₂-eq/Jahr"; // Ebenfalls in Anführungszeichen, Einheitliche - Gesamt KA oder Bezug auf EW
    let title = format!("{name} ({ew} EW) / Treibhausgasemissionen [{einheit}]");

    let sankey_data: Vec<_> = labels;
    let sankey_links: Vec<(_, _, f64)> = streams
        .into_iter()
        .map(|(src, target, value)| (src, target.to_string(), value))
        .collect();

    let chart = Chart::new().title(Title::new().text(title)).series(
        Sankey::new()
            .emphasis(Emphasis::new().focus(EmphasisFocus::Adjacency))
            .data(sankey_data)
            .links(sankey_links),
    );
    log::debug!("Render Sankey chart");
    let renderer = charming::WasmRenderer::new(1200, 800);
    renderer.render(element_id, &chart).unwrap();
}

pub fn clear(element_id: &str) {
    let el = leptos::document().get_element_by_id(element_id).unwrap();
    el.set_inner_html("");
    el.remove_attribute("_echarts_instance_").unwrap();
}

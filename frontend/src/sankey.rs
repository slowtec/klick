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
        ("N₂O Anlage", dir_em, co2eq_n2o_anlage),
        ("N₂O Gewässer", dir_em, co2eq_n2o_gewaesser),
        ("CH₄ Klärprozess", dir_em, co2eq_ch4_klaerprozes),
        ("CH₄ Schlupf Schlammstapel", dir_em, co2eq_ch4_schlammstapel),
        ("CH₄ Schlupf Schlammtasche", dir_em, co2eq_ch4_schlammtasche),
        ("CH₄ Gewässer", dir_em, co2eq_ch4_gewaesser),
        ("CH₄ BHKW", dir_em, co2eq_ch4_bhkw),
        (
            "Eisen(III)-chlorid-Lösung",
            "Betriebsstoffe",
            co2eq_betriebsstoffe_fe3,
        ),
        (
            "Eisenchloridsulfat-Lösung",
            "Betriebsstoffe",
            co2eq_betriebsstoffe_feso4,
        ),
        ("Kalkhydrat", "Betriebsstoffe", co2eq_betriebsstoffe_kalk),
        (
            "Synthetische Polymere",
            "Betriebsstoffe",
            co2eq_betriebsstoffe_poly,
        ),
        ("Strommix", indir_em, co2eq_strommix),
        ("Betriebsstoffe", wei_indir_em, co2eq_betriebsstoffe),
        (
            "Klaerschlamm Transport",
            wei_indir_em,
            co2eq_klaerschlamm_transport,
        ),
        (dir_em, nu, direkte_emissionen_co2_eq),
        (indir_em, nu, indirekte_emissionen_co2_eq),
        (wei_indir_em, nu, weitere_indirekte_emissionen_co2_eq),
        (nu, em, emissionen_co2_eq),
    ];

    let mut labels: Vec<_> = vec![];

    for (src, target, _) in &streams {
        labels.push(src.to_string());
        labels.push(target.to_string());
    }

    let einheit = "t CO₂-eq/Jahr"; // Ebenfalls in Anführungszeichen, Einheitliche - Gesamt KA oder Bezug auf EW
    let title = format!("{name} ({ew} EW) / Treibhausgasemissionen [{einheit}]");

    labels.sort();
    labels.dedup();
    let sankey_data: Vec<_> = labels;
    let sankey_links: Vec<(_, _, f64)> = streams;

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

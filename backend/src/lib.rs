use std::net::SocketAddr;

use axum::{
    http::{header, StatusCode, Uri},
    response::{Html, IntoResponse, Response},
    routing::post,
    Form, Router,
};
use charming::{
    element::{Emphasis, EmphasisFocus},
    series::Sankey,
    Chart,
};
use rust_embed::RustEmbed;

static INDEX_HTML: &str = "index.html";

#[derive(RustEmbed)]
#[folder = "../frontend/dist/"]
struct Assets;

pub async fn run(addr: SocketAddr) -> anyhow::Result<()> {
    log::info!("Start KlicK server");
    let app = Router::new()
        .route("/submit", post(create_sankey))
        .fallback(static_handler);

    log::info!("Start listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');

    if path.is_empty() || path == INDEX_HTML {
        return index_html().await;
    }

    match Assets::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();

            ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
        }
        None => {
            if path.contains('.') {
                return not_found().await;
            }

            index_html().await
        }
    }
}

async fn index_html() -> Response {
    match Assets::get(INDEX_HTML) {
        Some(content) => Html(content.data).into_response(),
        None => not_found().await,
    }
}

async fn not_found() -> Response {
    (StatusCode::NOT_FOUND, "404").into_response()
}

async fn create_sankey(Form(data): Form<klick_boundary::FormData>) -> Html<String> {
    log::debug!("{data:?}");

    let klick_boundary::FormData {
        name,
        ew,
        flow,
        csb_zu,
        tkn_zu,
        p_zu,
        csb_ab,
        tkn_ab,
        p_ab,
        klaergas,
        methangehalt,
        gas_zusatz: _,
        biogas: _,
        strombedarf,
        eigenstrom,
        ef_strommix,
        schlammtaschen,
        schlammstapel,
        klaerschlamm_enstorgung,
        klaerschlamm_transport,
        betriebsstoffe_fe3,
        betriebsstoffe_feso4,
        betriebsstoffe_kalk,
        betriebsstoffe_poly,
        n2o_szenario,
    } = data;

    let name_ka = name; // Name muss in Anführungszeichen stehen
    let ew = float(ew); // Anzahl EW
    let einheit = "t co2-eq/Jahr"; // Ebenfalls in Anführungszeichen, Einheitliche - Gesamt KA oder Bezug auf EW
    let abwasser = float(flow); // [m^3/a]
    let n2o_szenario = n2o_szenario.parse::<usize>().unwrap();

    // Engergie
    let energie_eigen = float(eigenstrom); // [kwh/a]
    let energie_fremd = float(strombedarf) - energie_eigen; // [kwh/a]
    let klaergas_gesamt = float(klaergas); // [m^3/a]
    let klaergas_methangehalt = float(methangehalt) / 100.0; // [%]

    // Zulauf-Parameter
    let _csb_zu = float(csb_zu); // [mg/l]
    let n_ges_zu = float(tkn_zu); // [mg/l]
    let _p_zu = float(p_zu); // [mg/l]

    // Ablauf-Parameter
    let csb_ab = float(csb_ab); // [mg/l]
    let n_ges_ab = float(tkn_ab); // [mg/l]
    let _p_ab = float(p_ab); // [mg/l]
    let n_elim = (n_ges_zu - n_ges_ab) / n_ges_zu * 100.0; // [%]

    // Weitere Indirekte emissionen

    let betriebsstoffe_fe3 = float(betriebsstoffe_fe3);
    let betriebsstoffe_feso4 = float(betriebsstoffe_feso4);
    let betriebsstoffe_kalk = float(betriebsstoffe_kalk);
    let betriebsstoffe_poly = float(betriebsstoffe_poly);

    let klaerschlamm_transport_km = float(klaerschlamm_transport); // [t co2eq/a]
    let klaerschlamm_entsorgung_m = float(klaerschlamm_enstorgung); // [t co2eq/a]

    // emissionsfaktoren
    let ef_ch4_anlage = 230.0; // [g ch4/(ew*a)]
    let ef_ch4_gewaesser = 0.009; // [0,9 % des csb-ablauf]
    let ef_ch4_bhkw = 1.124; // [1,124 g ch4/kwh]

    let ef_n2o_anlage = match n2o_szenario {
        0 => get_n2oef(n_elim), // [Berechnung nach Parravicini et al. 2016]
        1 => 0.005,             // [0,5 % des Ges-N Zulauf]
        2 => 0.016,             // [1,6 % des Ges-N Zulauf]
        3 => 0.032,             // [3,2 % des Ges-N Zulauf]
        _ => panic!("invalid input"),
    };

    let ef_n2o_gewaesser = 0.005; // [0,5 % des Ges-N Ablauf]

    let ef_co2_strommix = float(ef_strommix); // [g co2/kwh]

    // Emissionsfaktoren Schlupf
    let ef_schlammtasche = 0.003; // [0.3 % des Gesamtmethangasertrags]
    let ef_schlammstapel = 0.017; // [1,7 % der Gesamtfaulgaserzeugung]

    // emissionsfaktoren Betriebsstoffe
    let ef_fe3 = 395.0; // [g co2/kg Lösung]
    let ef_feso4 = 76.0; // [g co2/kg Lösung]
    let ef_kalk = 1055.3; // [g co2/kg Lösung]
    let ef_poly = 2200.0; // [g co2/kg Lösung]

    // Umrechnungsfaktoren
    let gwp_n2o = 265.0;
    let gwp_ch4 = 28.0;
    let uf_n_zu_n2o = 44.0 / 28.0;
    let uf_ch4_m3_zu_kg = 0.7175; // [kg/m^3] für Normkubikmeter (GESTIS Stoffdatenbank)

    // emissionsfaktoren Transport per LKW
    let ef_diesel = 3.24; // kg co2/l
    let verbrauch = 0.033; // l/tkm

    // Direkte emissionen

    // Lachgas
    let n2o_anlage =
        n_ges_zu / (10_i32.pow(9) as f64) * abwasser * 1000.0 * ef_n2o_anlage * uf_n_zu_n2o; // [t N2O/a]
    let n2o_gewaesser =
        n_ges_ab / (10_i32.pow(9) as f64) * abwasser * 1000.0 * ef_n2o_gewaesser * uf_n_zu_n2o; // [t N2O/a]

    // Methan
    let ch4_klaerprozess = ew * ef_ch4_anlage / (10_i32.pow(6) as f64); // [t CH4/a]
    let ch4_gewaesser =
        csb_ab / (10_i32.pow(9) as f64) * abwasser * (10_i32.pow(3) as f64) * ef_ch4_gewaesser; // [t CH4/a]
    let ch4_bhkw = energie_eigen * ef_ch4_bhkw / (10_i32.pow(6) as f64); // [t CH4/a]
    let mut ch4_schlupf_schlammstapel = 0.0;
    let mut ch4_schlupf_schlammtasche = 0.0;

    if schlammtaschen.as_deref() == Some("yes") {
        ch4_schlupf_schlammtasche =
            klaergas_gesamt * klaergas_methangehalt * ef_schlammtasche * uf_ch4_m3_zu_kg / 1_000.0;
        // [t CH4 / a]
    }

    if schlammstapel.as_deref() == Some("yes") {
        ch4_schlupf_schlammstapel =
            klaergas_gesamt * klaergas_methangehalt * ef_schlammstapel * uf_ch4_m3_zu_kg / 10_000.0;
        // [t CH4 / a]
    }

    // co2eq
    let co2eq_n2o_anlage = n2o_anlage * gwp_n2o; // [t co2eq/a]
    let co2eq_n2o_gewaesser = n2o_gewaesser * gwp_n2o; // [t co2eq/a]

    let co2eq_ch4_klaerprozes = ch4_klaerprozess * gwp_ch4; // [t co2eq/a]
    let co2eq_ch4_schlammstapel = ch4_schlupf_schlammstapel * gwp_ch4; // [t co2eq/a]
    let co2eq_ch4_schlammtasche = ch4_schlupf_schlammtasche * gwp_ch4; // [t co2eq/a]
    let co2eq_ch4_gewaesser = ch4_gewaesser * gwp_ch4; // [t co2eq/a]
    let co2eq_ch4_bhkw = ch4_bhkw * gwp_ch4; // [t co2eq/a]

    let co2eq_strommix = energie_fremd * ef_co2_strommix / (10_i32.pow(6) as f64); // [t co2eq/a]

    let co2eq_betriebsstoffe_poly = betriebsstoffe_poly * ef_poly / (10_i32.pow(6) as f64); // [t co2eq/a]
    let co2eq_betriebsstoffe_fe3 = betriebsstoffe_fe3 * ef_fe3 / (10_i32.pow(6) as f64); // [t co2eq/a]
    let co2eq_betriebsstoffe_feso4 = betriebsstoffe_feso4 * ef_feso4 / (10_i32.pow(6) as f64); // [t co2eq/a]
    let co2eq_betriebsstoffe_kalk = betriebsstoffe_kalk * ef_kalk / (10_i32.pow(6) as f64); // [t co2eq/a]
    let co2eq_betriebsstoffe = co2eq_betriebsstoffe_poly +                  // [t co2eq/a]
                            co2eq_betriebsstoffe_feso4 +
                            co2eq_betriebsstoffe_kalk +
                            co2eq_betriebsstoffe_fe3;

    let co2eq_klaerschlamm_transport =
        klaerschlamm_entsorgung_m * klaerschlamm_transport_km * verbrauch * ef_diesel / 1000.0; // [t co2eq/a]

    // Interimsgrößen
    let direkte_emissionen_co2_eq = co2eq_n2o_anlage
        + co2eq_n2o_gewaesser
        + co2eq_ch4_klaerprozes
        + co2eq_ch4_gewaesser
        + co2eq_ch4_bhkw
        + co2eq_ch4_schlammstapel
        + co2eq_ch4_schlammtasche;
    let indirekte_emissionen_co2_eq = co2eq_strommix;
    let weitere_indirekte_emissionen_co2_eq = co2eq_betriebsstoffe + co2eq_klaerschlamm_transport;
    let nutzung_co2_eq = direkte_emissionen_co2_eq
        + indirekte_emissionen_co2_eq
        + weitere_indirekte_emissionen_co2_eq;
    let emissionen_co2_eq = nutzung_co2_eq;

    let dir_em = "Direkte Emissionen";
    let indir_em = "Indirekte Emissionen";
    let wei_indir_em = "Weitere Indirekte Emissionen";
    let nu = "Nutzung";
    let em = "Emission";

    let streams: Vec<(_, _, _)> = vec![
        ("N<sub>2</sub>O Anlage", dir_em, co2eq_n2o_anlage),
        ("N<sub>2</sub>O Gewaesser", dir_em, co2eq_n2o_gewaesser),
        ("CH<sub>4</sub> Klärprozess", dir_em, co2eq_ch4_klaerprozes),
        (
            "CH<sub>4</sub> Schlupf Schlammstapel",
            dir_em,
            co2eq_ch4_schlammstapel,
        ),
        (
            "CH<sub>4</sub> Schlupf Schlammtasche",
            dir_em,
            co2eq_ch4_schlammtasche,
        ),
        ("CH<sub>4</sub> Gewaesser", dir_em, co2eq_ch4_gewaesser),
        ("CH<sub>4</sub> BHKW", dir_em, co2eq_ch4_bhkw),
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

    let title = format!("{name_ka} ({ew} EW)<br />Treibhausgasemissionen [{einheit}]");

    labels.sort();
    labels.dedup();
    let sankey_data: Vec<_> = labels;
    let sankey_links: Vec<(_, _, f64)> = streams;

    let chart = Chart::new().series(
        Sankey::new()
            .emphasis(Emphasis::new().focus(EmphasisFocus::Adjacency))
            .data(sankey_data)
            .links(sankey_links),
    );
    let renderer = charming::HtmlRenderer::new(title, 1200, 800);
    let html_string = renderer.render(&chart).unwrap();
    Html(html_string)
}

fn float(txt: String) -> f64 {
    txt.trim().replace(',', ".").parse::<f64>().unwrap()
}

fn get_n2oef(n_elim: f64) -> f64 {
    let mut ef = (-0.049 * n_elim + 4.553) / 100.0;
    if ef < 0.0 {
        ef = 0.002;
    }
    ef
}

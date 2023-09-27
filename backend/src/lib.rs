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
    log::debug!("Received {data:#?}");

    let klick_boundary::FormData {
        name,
        ew,
        flow,
        csb_zu: _,
        tkn_zu,
        p_zu: _,
        csb_ab,
        tkn_ab,
        p_ab: _,
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
    let strombedarf = float(strombedarf);
    let klaergas_gesamt = float(klaergas); // [m^3/a]
    let methangehalt = float(methangehalt);

    // Zulauf-Parameter
    let n_ges_zu = float(tkn_zu); // [mg/l]

    // Ablauf-Parameter
    let csb_ab = float(csb_ab); // [mg/l]
    let n_ges_ab = float(tkn_ab); // [mg/l]

    // Weitere Indirekte emissionen

    let betriebsstoffe_fe3 = float(betriebsstoffe_fe3);
    let betriebsstoffe_feso4 = float(betriebsstoffe_feso4);
    let betriebsstoffe_kalk = float(betriebsstoffe_kalk);
    let betriebsstoffe_poly = float(betriebsstoffe_poly);

    let klaerschlamm_transport_km = float(klaerschlamm_transport); // [t co2eq/a]
    let klaerschlamm_entsorgung_m = float(klaerschlamm_enstorgung); // [t co2eq/a]

    let schlammtaschen =
        schlammtaschen.as_deref() == Some("yes") || schlammtaschen.as_deref() == Some("on");
    let schlammstapel =
        schlammstapel.as_deref() == Some("yes") || schlammstapel.as_deref() == Some("on");

    let ef_co2_strommix = float(ef_strommix); // [g co2/kwh]

    let input_data = klick_application::InputData {
        ew,
        abwasser,
        n_ges_zu,
        csb_ab,
        n_ges_ab,
        klaergas_gesamt,
        methangehalt,
        strombedarf,
        energie_eigen,
        ef_co2_strommix,
        schlammtaschen,
        schlammstapel,
        klaerschlamm_transport_km,
        klaerschlamm_entsorgung_m,
        betriebsstoffe_fe3,
        betriebsstoffe_feso4,
        betriebsstoffe_kalk,
        betriebsstoffe_poly,
        n2o_szenario,
    };

    log::debug!("Calculating with {input_data:#?}");
    let output_data = klick_application::calc(&input_data);

    log::debug!("Result is {output_data:#?}");

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
    log::debug!("Render Sankey chart");
    let renderer = charming::HtmlRenderer::new(title, 1200, 800);
    let html_string = renderer.render(&chart).unwrap();
    Html(html_string)
}

fn float(txt: String) -> f64 {
    txt.trim().replace(',', ".").parse::<f64>().unwrap()
}

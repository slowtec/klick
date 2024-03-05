use std::{io::Write, path::Path};

use anyhow::bail;
use lazy_static::lazy_static;
use pandoc::{InputKind, OutputFormat, OutputKind, PandocOutput};
use serde::Serialize;
use tera::{Context, Tera};
use time::{format_description::FormatItem, macros::format_description, OffsetDateTime, UtcOffset};

use klick_app_charts as charts;
use klick_boundary as boundary;
use klick_domain::{
    self as domain, CO2Equivalents, EmissionFactors, EmissionInfluencingValues,
    N2oEmissionFactorCalcMethod,
};
use klick_presenter::{self as presenter, Lng};

const MARKDOWN_TEMPLATE: &str = include_str!("../templates/report.md.template");
const MARKDOWN_TEMPLATE_NAME: &str = "report.md";
const LATEX_TABLE_TEMPLATE: &str = include_str!("../templates/table.tex.template");
const LATEX_TABLE_TEMPLATE_NAME: &str = "table.tex";

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = Tera::default();
        tera.add_raw_template(MARKDOWN_TEMPLATE_NAME, MARKDOWN_TEMPLATE)
            .expect("valid markdown template");
        tera.add_raw_template(LATEX_TABLE_TEMPLATE_NAME, LATEX_TABLE_TEMPLATE)
            .expect("valid table template");
        tera
    };
}

pub fn export_to_pdf(project: boundary::ProjectData) -> anyhow::Result<Vec<u8>> {
    let date = current_date_as_string()?;
    let boundary::ProjectData {
        plant_profile,
        optimization_scenario,
        ..
    } = project;
    let boundary::OptimizationScenario {
        n2o_emission_factor,
        ch4_chp_emission_factor: _,
    } = optimization_scenario;
    let emission_influencing_values = EmissionInfluencingValues::try_from(plant_profile.clone())?;
    let n2o_emission_factor_calc_method =
        N2oEmissionFactorCalcMethod::try_from(n2o_emission_factor)?;
    let custom_factor = match n2o_emission_factor_calc_method {
        N2oEmissionFactorCalcMethod::Custom(factor) => Some(factor),
        _ => None,
    };
    let _n2o_scenarios = domain::calculate_all_n2o_emission_factor_scenarios(
        &emission_influencing_values,
        custom_factor,
        None,
    );
    todo!(); // FIXME @markus help
             // let n2o_scenarios_bar_chart = render_svg_bar_chart(n2o_scenarios.clone());
             // let mut bar_svg_file = tempfile::Builder::new().suffix(".svg").tempfile()?;
             // bar_svg_file.write_all(n2o_scenarios_bar_chart.as_bytes())?;
             //
             // let sankey_chart = render_svg_sankey_chart(n2o_scenarios[0].1.clone());
             // let mut sankey_svg_file = tempfile::Builder::new().suffix(".svg").tempfile()?;
             // sankey_svg_file.write_all(sankey_chart.as_bytes())?;
             //
             // let markdown = render_markdown_template(
             //     date,
             //     plant_profile,
             //     bar_svg_file.path(),
             //     sankey_svg_file.path(),
             // )?;
             // let bytes = render_pdf(markdown)?;
             // bar_svg_file.close()?;
             // sankey_svg_file.close()?;
             // Ok(bytes)
}

#[derive(Serialize)]
struct TemplateData {
    date: String,
    table: String,
    profile: boundary::PlantProfile,
    n2o_svg_barchart_file_path: String,
    sankey_svg_file_path: String,
}

fn render_markdown_template(
    date: String,
    profile: boundary::PlantProfile,
    n2o_svg_barchart_file: &Path,
    sankey_svg_file: &Path,
) -> anyhow::Result<String> {
    let table_data = presenter::plant_profile_as_table(&profile);
    let table = create_latex_table(&table_data)?;
    let n2o_svg_barchart_file_path = n2o_svg_barchart_file.display().to_string();
    let sankey_svg_file_path = sankey_svg_file.display().to_string();
    let data = TemplateData {
        date,
        profile,
        table,
        n2o_svg_barchart_file_path,
        sankey_svg_file_path,
    };
    let rendered = TEMPLATES.render(MARKDOWN_TEMPLATE_NAME, &Context::from_serialize(&data)?)?;
    Ok(rendered)
}

const BAR_CHART_WIDTH: f64 = 600.0;
const BAR_CHART_HEIGHT: f64 = 300.0;

fn render_svg_bar_chart(
    n2o_scenarios: Vec<(N2oEmissionFactorCalcMethod, CO2Equivalents, EmissionFactors)>,
) -> String {
    let data = n2o_scenarios
        .into_iter()
        .map(|(_method, co2_equivalents, emission_factors)| {
            charts::BarChartRadioInputArguments {
                label: None, // TODO: Render method name
                value: co2_equivalents.total_emissions.into(),
                emission_factor: emission_factors.n2o.into(),
            }
        })
        .collect();
    charts::ssr::bar_chart(data, BAR_CHART_WIDTH, BAR_CHART_HEIGHT, None, None)
}

fn render_svg_sankey_chart(co2_equivalents: CO2Equivalents) -> String {
    let (nodes, edges) = presenter::create_sankey_chart_data(co2_equivalents);

    let mut sankey = charts::SankeyData::new();
    let node_count = nodes.len();
    let node_ids: Vec<_> = nodes
        .into_iter()
        .map(|(value, label, color)| {
            sankey.insert_node(value, label, Some(charts::Color::new(color)))
        })
        .collect();
    assert_eq!(node_ids.len(), node_count);

    for (from_idx, to_idx) in edges {
        let from = node_ids[from_idx];
        let to = node_ids[to_idx];
        sankey.insert_edge(from, to);
    }

    let number_format = |n| Lng::De.format_number_with_thousands_seperator(n);

    charts::ssr::sankey_chart(sankey, 1200.0, 800.0, number_format, 16.0)
}

fn render_pdf(markdown: String) -> anyhow::Result<Vec<u8>> {
    let mut pandoc = pandoc::new();
    pandoc.set_input(InputKind::Pipe(markdown));
    pandoc.set_output(OutputKind::Pipe);
    pandoc.set_output_format(OutputFormat::Pdf, vec![]);
    let output = pandoc.execute()?;

    let PandocOutput::ToBufferRaw(bytes) = output else {
        bail!("unexpeced pandoc output");
    };
    Ok(bytes)
}

const DATE_FORMAT_DESCRIPTION: &[FormatItem<'_>] = format_description!("[day].[month].[year]");

const DEFAULT_OFFSET_HOURS: i8 = 1;

fn current_date_as_string() -> anyhow::Result<String> {
    let now_utc = OffsetDateTime::now_utc();
    // FIXME:
    // This offset depends on the location of the system.
    // Use German offset but also check the Sommer-/Winterzeit.
    let local_offset = UtcOffset::current_local_offset()
        .or_else(|_| UtcOffset::from_hms(DEFAULT_OFFSET_HOURS, 0, 0))?;
    let local_date_time = now_utc.to_offset(local_offset);
    let date = local_date_time.format(DATE_FORMAT_DESCRIPTION)?;
    Ok(date)
}

fn create_latex_table(table: &presenter::Table) -> anyhow::Result<String> {
    let rendered = TEMPLATES.render(LATEX_TABLE_TEMPLATE_NAME, &Context::from_serialize(table)?)?;
    Ok(rendered)
}

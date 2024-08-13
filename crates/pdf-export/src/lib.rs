#![allow(unused)] // FIXME

use std::{collections::HashMap, io::Write, path::Path, sync::LazyLock};

use anyhow::bail;
use num_traits::ToPrimitive;
use pandoc::{InputFormat, InputKind, MarkdownExtension, OutputFormat, OutputKind, PandocOutput};
use serde::Serialize;
use tera::{Context, Tera};
use time::{format_description::FormatItem, macros::format_description, OffsetDateTime, UtcOffset};

use klick_app_charts as charts;
use klick_boundary as boundary;
use klick_domain::{
    self as domain,
    units::{Ch4ChpEmissionFactorCalcMethod, Factor, N2oEmissionFactorCalcMethod, Tons},
    CO2Equivalents, EmissionsCalculationOutcome, InputValueId as Id, OutputValueId as Out, Value,
};
use klick_presenter::{self as presenter, Formatting, Lng, ValueLabel};

const MARKDOWN_TEMPLATE: &str = include_str!("../templates/report.md.template");
const MARKDOWN_TEMPLATE_NAME: &str = "report.md";
const LATEX_TABLE_TEMPLATE: &str = include_str!("../templates/table.tex.template");
const LATEX_TABLE_TEMPLATE_NAME: &str = "table.tex";

pub static TEMPLATES: LazyLock<Tera> = LazyLock::new(|| {
    let mut tera = Tera::default();
    tera.add_raw_template(MARKDOWN_TEMPLATE_NAME, MARKDOWN_TEMPLATE)
        .expect("valid markdown template");
    tera.add_raw_template(LATEX_TABLE_TEMPLATE_NAME, LATEX_TABLE_TEMPLATE)
        .expect("valid table template");
    tera
});

pub fn export_to_pdf(form_data: boundary::FormData) -> anyhow::Result<Vec<u8>> {
    let date = current_date_as_string()?;
    let outcome = boundary::calculate(form_data);

    let mut n2o_scenarios_svg_file = tempfile::Builder::new().suffix(".svg").tempfile()?;
    let mut ch4_chp_scenarios_svg_file = tempfile::Builder::new().suffix(".svg").tempfile()?;
    let mut profile_sankey_svg_file = tempfile::Builder::new().suffix(".svg").tempfile()?;
    let mut sensitivity_sankey_svg_file = tempfile::Builder::new().suffix(".svg").tempfile()?;
    let mut recommendation_sankey_svg_file = tempfile::Builder::new().suffix(".svg").tempfile()?;
    let mut sensitivity_barchart_svg_file = tempfile::Builder::new().suffix(".svg").tempfile()?;
    let mut recommendation_barchart_svg_file =
        tempfile::Builder::new().suffix(".svg").tempfile()?;

    let plant_profile_sankey_svg_file_path = if let Some(output) = &outcome.output {
        let sankey_chart = render_svg_sankey_chart(output.co2_equivalents.clone());
        profile_sankey_svg_file.write_all(sankey_chart.as_bytes())?;
        Some(profile_sankey_svg_file.path().display().to_string())
    } else {
        None
    };

    let sensitivity_sankey_svg_file_path = if let Some(output) = &outcome.output {
        let sankey_chart = render_svg_sankey_chart(output.co2_equivalents.clone());
        sensitivity_sankey_svg_file.write_all(sankey_chart.as_bytes())?;
        Some(sensitivity_sankey_svg_file.path().display().to_string())
    } else {
        None
    };

    let recommendation_sankey_svg_file_path = if let Some(output) = &outcome.output {
        let sankey_chart = render_svg_sankey_chart(output.co2_equivalents.clone());
        recommendation_sankey_svg_file.write_all(sankey_chart.as_bytes())?;
        Some(recommendation_sankey_svg_file.path().display().to_string())
    } else {
        None
    };

    let selected_n2o_scenario = &outcome
        .input
        .get(&Id::SensitivityN2OCalculationMethod)
        .cloned()
        .map(Value::as_n2o_emission_factor_calc_method_unchecked)
        .as_ref()
        .and_then(ToPrimitive::to_u64);
    let selected_ch4_chp_scenario = &outcome
        .input
        .get(&Id::SensitivityCH4ChpCalculationMethod)
        .cloned()
        .map(Value::as_ch4_chp_emission_factor_calc_method_unchecked)
        .as_ref()
        .and_then(ToPrimitive::to_u64);

    let n2o_scenarios_svg_file_path = if let Some(scenarios) = &outcome.sensitivity_n2o_calculations
    {
        let svg_chart =
            render_n2o_scenarios_svg_bar_chart(scenarios.clone(), *selected_n2o_scenario);
        n2o_scenarios_svg_file.write_all(svg_chart.as_bytes())?;
        Some(n2o_scenarios_svg_file.path().display().to_string())
    } else {
        None
    };

    let ch4_chp_scenarios_svg_file_path: Option<String> = if let Some(scenarios) =
        &outcome.sensitivity_ch4_chp_calculations
    {
        let svg_chart =
            render_ch4_chp_scenarios_svg_bar_chart(scenarios.clone(), *selected_ch4_chp_scenario);
        ch4_chp_scenarios_svg_file.write_all(svg_chart.as_bytes())?;
        Some(ch4_chp_scenarios_svg_file.path().display().to_string())
    } else {
        None
    };

    let sensitivity_barchart_svg_file_path: Option<String> = if let Some(data) = outcome
        .output
        .as_ref()
        .map(|o| o.co2_equivalents.clone())
        .and_then(|old| {
            outcome
                .output
                .as_ref()
                .map(|o| (o.co2_equivalents.clone(), old))
        })
        .map(|(new, old)| {
            presenter::sensitivity_diff_bar_chart(old, new)
                .into_iter()
                .filter(|(_, value, _)| f64::abs(*value) > 0.1)
                .map(|(label, value, percentage)| charts::BarChartArguments {
                    label,
                    value,
                    percentage,
                })
                .collect::<Vec<_>>()
        }) {
        if data.is_empty() {
            None
        } else {
            let svg_chart = charts::ssr::bar_chart(data, BAR_CHART_WIDTH, 450.0);
            sensitivity_barchart_svg_file.write_all(svg_chart.as_bytes())?;
            Some(sensitivity_barchart_svg_file.path().display().to_string())
        }
    } else {
        None
    };

    let recommendation_barchart_svg_file_path: Option<String> = if let Some(data) = outcome
        .output
        .as_ref()
        .map(|o| o.co2_equivalents.clone())
        .and_then(|old| {
            outcome
                .output
                .as_ref()
                .map(|o| (o.co2_equivalents.clone(), old))
        })
        .map(|(new, old)| {
            presenter::recommendation_diff_bar_chart(old, new)
                .into_iter()
                .filter(|(_, value, _)| f64::abs(*value) > 0.1)
                .map(|(label, value, percentage)| charts::BarChartArguments {
                    label,
                    value,
                    percentage,
                })
                .collect::<Vec<_>>()
        }) {
        if data.is_empty() {
            None
        } else {
            let svg_chart = charts::ssr::bar_chart(data, BAR_CHART_WIDTH, 450.0);
            recommendation_barchart_svg_file.write_all(svg_chart.as_bytes())?;
            Some(
                recommendation_barchart_svg_file
                    .path()
                    .display()
                    .to_string(),
            )
        }
    } else {
        None
    };

    let markdown = render_markdown_template(
        date,
        outcome,
        plant_profile_sankey_svg_file_path,
        sensitivity_sankey_svg_file_path,
        sensitivity_barchart_svg_file_path,
        recommendation_sankey_svg_file_path,
        n2o_scenarios_svg_file_path,
        ch4_chp_scenarios_svg_file_path,
        recommendation_barchart_svg_file_path,
    )?;

    let bytes = render_pdf(markdown)?;

    n2o_scenarios_svg_file.close()?;
    ch4_chp_scenarios_svg_file.close()?;
    profile_sankey_svg_file.close()?;
    sensitivity_sankey_svg_file.close()?;
    sensitivity_barchart_svg_file.close()?;
    recommendation_sankey_svg_file.close()?;
    recommendation_barchart_svg_file.close()?;

    Ok(bytes)
}

fn render_markdown_template(
    date: String,
    outcome: boundary::CalculationOutcome,
    plant_profile_sankey_svg_file_path: Option<String>,
    sensitivity_sankey_svg_file_path: Option<String>,
    sensitivity_barchart_svg_file_path: Option<String>,
    recommendation_sankey_svg_file_path: Option<String>,
    n2o_scenarios_svg_file_path: Option<String>,
    ch4_chp_scenarios_svg_file_path: Option<String>,
    recommendation_barchart_svg_file_path: Option<String>,
) -> anyhow::Result<String> {
    let plant_profile_table_data =
        presenter::plant_profile_as_table(&outcome.input, Formatting::LaTeX);
    let plant_profile_table = create_latex_table(&plant_profile_table_data)?;

    let sensitivity_table_data = presenter::sensitivity_parameters_as_table(
        &outcome.input,
        Formatting::LaTeX,
        outcome.output.as_ref(),
    );
    let sensitivity_parameters_table = create_latex_table(&sensitivity_table_data)?;

    let plant_name = outcome
        .input
        .get(&Id::PlantName)
        .cloned()
        .map_or_else(|| "Klärwerk".to_string(), Value::as_text_unchecked);

    let plant_profile_sankey_header = outcome
        .output
        .map(|output| {
            presenter::create_sankey_chart_header(
                &outcome.input,
                output.emission_factors,
                output.calculation_methods,
                Formatting::LaTeX,
            )
        })
        .unwrap_or_default();

    let data = TemplateData {
        date,
        plant_profile_table,
        sensitivity_parameters_table,
        plant_name,
        plant_profile_sankey_header,
        n2o_scenarios_svg_file_path,
        ch4_chp_scenarios_svg_file_path,
        plant_profile_sankey_svg_file_path,
        sensitivity_sankey_svg_file_path,
        sensitivity_barchart_svg_file_path,
        recommendation_sankey_svg_file_path,
        recommendation_barchart_svg_file_path,
    };

    let rendered = TEMPLATES.render(MARKDOWN_TEMPLATE_NAME, &Context::from_serialize(data)?)?;
    Ok(rendered)
}

// TODO:
// - Restructure the fields
// - Improve field names
#[derive(Serialize, Debug)]
struct TemplateData {
    date: String,
    plant_profile_table: String,
    sensitivity_parameters_table: String,
    plant_name: String,
    plant_profile_sankey_header: String,
    n2o_scenarios_svg_file_path: Option<String>,
    ch4_chp_scenarios_svg_file_path: Option<String>,
    plant_profile_sankey_svg_file_path: Option<String>,
    sensitivity_sankey_svg_file_path: Option<String>,
    sensitivity_barchart_svg_file_path: Option<String>,
    recommendation_sankey_svg_file_path: Option<String>,
    recommendation_barchart_svg_file_path: Option<String>,
}

const BAR_CHART_WIDTH: f64 = 1100.0;
const BAR_CHART_HEIGHT: f64 = 300.0;

fn render_n2o_scenarios_svg_bar_chart(
    n2o_scenarios: Vec<(N2oEmissionFactorCalcMethod, EmissionsCalculationOutcome)>,
    selected: Option<u64>,
) -> String {
    let data = n2o_scenarios
        .into_iter()
        .map(|(method, emissions_calculation_outcome)| {
            let EmissionsCalculationOutcome {
                co2_equivalents,
                emission_factors,
                calculation_methods: _,
            } = emissions_calculation_outcome;
            charts::BarChartRadioInputArguments {
                label: Some(method.label()),
                value: co2_equivalents
                    .get(&Out::TotalEmissions)
                    .copied()
                    .unwrap()
                    .into(),
                emission_factor: emission_factors.n2o.into(),
            }
        })
        .collect();
    let emission_factor_label = Some("N₂O EF");
    charts::ssr::bar_chart_radio_input(
        data,
        BAR_CHART_WIDTH,
        BAR_CHART_HEIGHT,
        selected,
        emission_factor_label,
    )
}

fn render_ch4_chp_scenarios_svg_bar_chart(
    scenarios: Vec<(Ch4ChpEmissionFactorCalcMethod, Tons, Factor)>,
    selected: Option<u64>,
) -> String {
    let data = scenarios
        .into_iter()
        .map(
            |(method, emissions, emission_factor)| charts::BarChartRadioInputArguments {
                label: Some(method.label()),
                value: emissions.into(),
                emission_factor: emission_factor.into(),
            },
        )
        .collect();
    let emission_factor_label = Some("CH₄ EF");
    charts::ssr::bar_chart_radio_input(
        data,
        BAR_CHART_WIDTH,
        BAR_CHART_HEIGHT,
        selected,
        emission_factor_label,
    )
}

fn render_svg_sankey_chart(co2_equivalents: HashMap<Out, Tons>) -> String {
    let (nodes, edges) = presenter::create_sankey_chart_data(co2_equivalents);

    let mut sankey = charts::SankeyData::new();
    let node_count = nodes.len();
    let node_ids: Vec<_> = nodes
        .into_iter()
        .map(|(value, label, color, edge_color)| {
            sankey.insert_node(
                value,
                label,
                Some(charts::Color::new(color)),
                Some(charts::Color::new(edge_color)),
            )
        })
        .collect();
    assert_eq!(node_ids.len(), node_count);

    for (from_idx, to_idx) in edges {
        let from = node_ids[from_idx];
        let to = node_ids[to_idx];
        sankey.insert_edge(from, to);
    }

    let number_format = |n| Lng::De.format_number_with_fixed_precision(n, 0);

    charts::ssr::sankey_chart(sankey, 1200.0, 800.0, number_format, 16.0, None)
}

fn render_pdf(markdown: String) -> anyhow::Result<Vec<u8>> {
    let mut pandoc = pandoc::new();
    pandoc.set_input(InputKind::Pipe(markdown));
    pandoc.set_output(OutputKind::Pipe);
    pandoc.set_input_format(InputFormat::Markdown, vec![]);
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

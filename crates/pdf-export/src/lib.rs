use anyhow::bail;
use lazy_static::lazy_static;
use pandoc::{InputKind, OutputFormat, OutputKind, PandocOutput};
use serde::Serialize;
use tera::{Context, Tera};
use time::{format_description::FormatItem, macros::format_description, OffsetDateTime};

use klick_application::usecases;
use klick_boundary as boundary;
use klick_domain::{EmissionInfluencingValues, N2oEmissionFactorCalcMethod};
use klick_presenter as presenter;

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
    let _scenarios = usecases::calculate_all_n2o_emission_factor_scenarios(
        &emission_influencing_values,
        custom_factor,
        None,
    );
    let markdown = render_markdown_template(date, plant_profile)?;
    let bytes = render_pdf(markdown)?;
    Ok(bytes)
}

#[derive(Serialize)]
struct TemplateData {
    date: String,
    table: String,
    profile: boundary::PlantProfile,
}

fn render_markdown_template(
    date: String,
    profile: boundary::PlantProfile,
) -> anyhow::Result<String> {
    let table_data = presenter::plant_profile_as_table(&profile);
    let table = create_latex_table(&table_data)?;
    let data = TemplateData {
        date,
        profile,
        table,
    };
    let rendered = TEMPLATES.render(MARKDOWN_TEMPLATE_NAME, &Context::from_serialize(&data)?)?;
    Ok(rendered)
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

fn current_date_as_string() -> anyhow::Result<String> {
    let now_utc = OffsetDateTime::now_utc();
    // FIXME:
    // This offset depends on the location of the system.
    // Use German offset but also check the Sommer-/Winterzeit.
    let local_offset = time::UtcOffset::current_local_offset()?;
    let local_date_time = now_utc.to_offset(local_offset);
    let date = local_date_time.format(DATE_FORMAT_DESCRIPTION)?;
    Ok(date)
}

fn create_latex_table(table: &presenter::Table) -> anyhow::Result<String> {
    let rendered = TEMPLATES.render(LATEX_TABLE_TEMPLATE_NAME, &Context::from_serialize(table)?)?;
    Ok(rendered)
}

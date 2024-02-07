use leptos::*;

use klick_app_components::forms;
use klick_boundary::PlantProfile;

use crate::forms::MissingField;

mod field_sets;
mod fields;

use self::field_sets::{field_sets, Id};

#[component]
pub fn PlantProfileInputForm(
    plant_profile: RwSignal<PlantProfile>,
    missing_fields: RwSignal<Vec<MissingField<Id>>>,
) -> impl IntoView {
    let field_sets = field_sets();
    let (signals, set_views, required_fields) = forms::render_field_sets(field_sets.clone());

    // TODO:
    // if let Some(chemical_oxygen_demand_influent) =
    //     input_data.influent_average.chemical_oxygen_demand
    // {
    //     if input_data.effluent_average.chemical_oxygen_demand
    //         > chemical_oxygen_demand_influent
    //     {
    //         chemical_oxygen_io_warning.set(Some(format!(
    //             "Ablauf Chemischer Sauerstoffbedarf {} größer als dessen Zulauf {}!",
    //             Lng::De.format_number(input_data.effluent_average.chemical_oxygen_demand),
    //             Lng::De.format_number(chemical_oxygen_demand_influent)
    //         )));
    //         input_data_validation_error = true;
    //     } else {
    //         chemical_oxygen_io_warning.set(None);
    //     }
    // }

    // TODO:
    // if let Some(phosphorus_influent) = input_data.influent_average.phosphorus {
    //     if let Some(phosphorus_effluent) = input_data.effluent_average.phosphorus {
    //         if phosphorus_effluent > phosphorus_influent {
    //             phosphorus_io_warning.set(Some(format!(
    //                 "Ablauf Phosphor {} größer als dessen Zulauf {}!",
    //                 Lng::De.format_number(phosphorus_effluent),
    //                 Lng::De.format_number(phosphorus_influent),
    //             )));
    //             input_data_validation_error = true;
    //         } else {
    //             phosphorus_io_warning.set(None);
    //         }
    //     }
    // }

    create_effect(move |_| {
        let (profile, filtered_required_fields) =
            fields::read_input_fields(&signals, &required_fields);
        missing_fields.set(filtered_required_fields);
        plant_profile.set(profile);
    });

    set_views
}

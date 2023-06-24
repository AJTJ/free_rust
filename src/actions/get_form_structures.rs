use crate::dive_forms::form_helper::{FormStructure, FormStructureOutput};

pub fn get_form_structures() -> FormStructureOutput {
    let latest = FormStructure::get_latest_form_template();

    latest
}

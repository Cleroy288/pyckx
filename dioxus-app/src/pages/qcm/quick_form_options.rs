//! Select options for the Quick QCM form

use crate::components::ui::select::SelectOption;

/// Level select options
pub fn level_options() -> Vec<SelectOption> {
    vec![
        SelectOption::new("easy", "Easy"),
        SelectOption::new("medium", "Medium"),
        SelectOption::new("hard", "Hard"),
    ]
}

/// Number of questions options
pub fn num_options() -> Vec<SelectOption> {
    vec![
        SelectOption::new("5", "5"),
        SelectOption::new("10", "10"),
        SelectOption::new("15", "15"),
        SelectOption::new("20", "20"),
    ]
}

/// Language select options
pub fn lang_options() -> Vec<SelectOption> {
    vec![
        SelectOption::new("en", "English"),
        SelectOption::new("fr", "Francais"),
        SelectOption::new("es", "Espanol"),
    ]
}

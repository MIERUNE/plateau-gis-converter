use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SelectionOptions {
    pub label: String,
    pub value: String,
}

impl SelectionOptions {
    pub fn new(label: &str, value: &str) -> Self {
        Self {
            label: label.to_string(),
            value: value.to_string(),
        }
    }

    pub fn get_label(&self) -> String {
        self.label.clone()
    }

    pub fn get_value(&self) -> String {
        self.value.clone()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Selection {
    pub options: Vec<SelectionOptions>,
    pub selected_value: String,
}

impl Selection {
    pub fn new(options: Vec<(&str, &str)>, selected_value: &str) -> Self {
        let options: Vec<SelectionOptions> = options
            .into_iter()
            .map(|(label, value)| SelectionOptions::new(label, value))
            .collect();

        let valid_value = options.iter().any(|opt| opt.value == selected_value);
        if !valid_value {
            panic!("selected_value must be one of the options");
        }

        Self {
            options,
            selected_value: selected_value.to_string(),
        }
    }

    pub fn set_selected_value(&mut self, value: &str) -> Result<(), String> {
        if self.options.iter().any(|opt| opt.value == value) {
            self.selected_value = value.to_string();
            Ok(())
        } else {
            Err("Invalid value".to_string())
        }
    }

    pub fn get_options(&self) -> Vec<SelectionOptions> {
        self.options.clone()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LodSelection;

impl LodSelection {
    /// Returns only the default LOD selection options.
    fn get_default_lod_selection_options() -> Vec<(&'static str, &'static str)> {
        vec![("最大LOD", "max_lod"), ("最小LOD", "min_lod")]
    }

    /// Returns extended options based on the provided keys.
    fn get_extended_options(keys: &[&str]) -> Vec<(&'static str, &'static str)> {
        let all_extended_options = vec![
            ("テクスチャ付き最大LOD", "textured_max_lod"),
            ("すべてのLOD", "all_lod"),
        ];

        all_extended_options
            .into_iter()
            .filter(|(_, value)| keys.contains(value))
            .collect()
    }

    /// Returns LOD selection options, optionally including extended options.
    fn get_lod_selection_options(
        extended_keys: Option<&[&str]>,
    ) -> Vec<(&'static str, &'static str)> {
        let mut options = Self::get_default_lod_selection_options();

        // Add the filtered extended options if provided
        if let Some(keys) = extended_keys {
            let extended_options = Self::get_extended_options(keys);
            options.extend(extended_options);
        }

        options
    }

    /// Creates a selection with optional extended options.
    pub fn create_lod_selection(default_value: &str, extended_keys: Option<&[&str]>) -> Selection {
        let options = Self::get_lod_selection_options(extended_keys);

        // Ensure the default value exists in the options
        if !options.iter().any(|&(_, value)| value == default_value) {
            panic!("Default value '{default_value}' must be a valid option");
        }

        Selection::new(options, default_value)
    }
}

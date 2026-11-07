//! adwaita model — GNOME theme/icon set.

#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub variant: String,
    pub type_: String,
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub themes: Vec<Theme>,
    pub active: Option<String>,
}

impl Default for Model {
    fn default() -> Self { Self { themes: vec![], active: None } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn add_theme(&mut self, name: &str, variant: &str, type_: &str) -> usize {
        let id = self.themes.len();
        self.themes.push(Theme { name: name.into(), variant: variant.into(), type_: type_.into(), enabled: false });
        id
    }

    pub fn activate(&mut self, name: &str) -> bool {
        if let Some(t) = self.themes.iter_mut().find(|t| t.name == name) {
            t.enabled = true;
            self.active = Some(name.into());
            true
        } else { false }
    }

    pub fn deactivate(&mut self, name: &str) -> bool {
        if let Some(t) = self.themes.iter_mut().find(|t| t.name == name) {
            t.enabled = false;
            if self.active.as_deref() == Some(name) {
                self.active = None;
            }
            true
        } else { false }
    }

    pub fn list(&self) -> &[Theme] { &self.themes }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn activate_deactivate() {
        let mut a = Model::new();
        a.add_theme("Adwaita", "dark", "gtk");
        assert!(a.activate("Adwaita"));
        assert_eq!(a.active.as_deref(), Some("Adwaita"));
        assert!(a.deactivate("Adwaita"));
    }
}

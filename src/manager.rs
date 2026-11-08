//! Adwaita theme manager.

use std::collections::HashMap;
use crate::theme::{AdwaitaTheme, ThemeVariant, ThemeState};
use crate::colors::{ColorScheme, NamedColor, RgbColor};
use crate::widget::WidgetStyle;
use crate::stylesheet::{Stylesheet, CssRule, CssSelector};

/// Manager options
#[derive(Debug, Clone)]
pub struct ManagerOptions {
    /// Max themes
    pub max_themes: usize,
    /// Default variant
    pub default_variant: ThemeVariant,
    /// Enable auto-switch
    pub enable_auto_switch: bool,
    /// Enable CSS generation
    pub enable_css_generation: bool,
    /// Theme directory
    pub theme_directory: String,
}

impl Default for ManagerOptions {
    fn default() -> Self {
        Self {
            max_themes: 100,
            default_variant: ThemeVariant::Light,
            enable_auto_switch: true,
            enable_css_generation: true,
            theme_directory: "~/.config/gtk-4.0".into(),
        }
    }
}

/// Manager statistics
#[derive(Debug, Clone)]
pub struct ManagerStatistics {
    /// Total themes
    pub total_themes: usize,
    /// Active themes
    pub active_themes: usize,
    /// Dark themes
    pub dark_themes: usize,
    /// Light themes
    pub light_themes: usize,
    /// Total widget styles
    pub total_widget_styles: usize,
    /// Total CSS rules
    pub total_css_rules: usize,
    /// Manager uptime in seconds
    pub uptime: u64,
}

/// Adwaita manager
#[derive(Debug, Clone)]
pub struct AdwaitaManager {
    /// Themes
    themes: HashMap<String, AdwaitaTheme>,
    /// Stylesheets
    stylesheets: HashMap<String, Stylesheet>,
    /// Options
    options: ManagerOptions,
    /// Statistics
    statistics: ManagerStatistics,
    /// Manager created timestamp
    pub created_at: u64,
}

impl Default for AdwaitaManager {
    fn default() -> Self {
        Self {
            themes: HashMap::new(),
            stylesheets: HashMap::new(),
            options: ManagerOptions::default(),
            statistics: ManagerStatistics {
                total_themes: 0,
                active_themes: 0,
                dark_themes: 0,
                light_themes: 0,
                total_widget_styles: 0,
                total_css_rules: 0,
                uptime: 0,
            },
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

impl AdwaitaManager {
    /// Create new Adwaita manager
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create manager with options
    pub fn with_options(options: ManagerOptions) -> Self {
        Self {
            themes: HashMap::new(),
            stylesheets: HashMap::new(),
            options,
            statistics: ManagerStatistics {
                total_themes: 0,
                active_themes: 0,
                dark_themes: 0,
                light_themes: 0,
                total_widget_styles: 0,
                total_css_rules: 0,
                uptime: 0,
            },
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Initialize manager
    pub fn initialize(&mut self) {
        // Create default themes
        let light_theme = AdwaitaTheme::new("Adwaita Light".into());
        let dark_theme = AdwaitaTheme::new("Adwaita Dark".into());
        
        let mut dark_theme = dark_theme;
        dark_theme.set_variant(ThemeVariant::Dark);
        
        self.themes.insert("adwaita-light".into(), light_theme);
        self.themes.insert("adwaita-dark".into(), dark_theme);
        
        self.update_statistics();
    }
    
    // Theme methods
    
    /// Create theme
    pub fn create_theme(&mut self, name: String) -> String {
        let theme = AdwaitaTheme::new(name.clone());
        let id = format!("theme-{}", self.themes.len() + 1);
        
        let mut theme = theme;
        theme.name = name;
        
        self.themes.insert(id.clone(), theme);
        self.update_statistics();
        id
    }
    
    /// Get theme
    pub fn get_theme(&self, theme_id: &str) -> Option<&AdwaitaTheme> {
        self.themes.get(theme_id)
    }
    
    /// Get theme mutably
    pub fn get_theme_mut(&mut self, theme_id: &str) -> Option<&mut AdwaitaTheme> {
        self.themes.get_mut(theme_id)
    }
    
    /// Get all themes
    pub fn list_themes(&self) -> Vec<&AdwaitaTheme> {
        self.themes.values().collect()
    }
    
    /// Delete theme
    pub fn delete_theme(&mut self, theme_id: &str) -> bool {
        if self.themes.remove(theme_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Set theme variant
    pub fn set_theme_variant(&mut self, theme_id: &str, variant: ThemeVariant) -> bool {
        if let Some(theme) = self.themes.get_mut(theme_id) {
            theme.set_variant(variant);
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Set theme state
    pub fn set_theme_state(&mut self, theme_id: &str, state: ThemeState) -> bool {
        if let Some(theme) = self.themes.get_mut(theme_id) {
            theme.set_state(state);
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Set theme color
    pub fn set_theme_color(&mut self, theme_id: &str, color_name: NamedColor, color: RgbColor) -> bool {
        if let Some(theme) = self.themes.get_mut(theme_id) {
            theme.colors.set_color(color_name, color);
            return true;
        }
        false
    }
    
    /// Get theme color
    pub fn get_theme_color(&self, theme_id: &str, color_name: &NamedColor) -> Option<&RgbColor> {
        self.themes.get(theme_id)
            .and_then(|t| t.colors.get_color(color_name))
    }
    
    /// Add widget style to theme
    pub fn add_widget_style(&mut self, theme_id: &str, style: WidgetStyle) -> bool {
        if let Some(theme) = self.themes.get_mut(theme_id) {
            theme.add_widget_style(style);
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Get themes by variant
    pub fn get_themes_by_variant(&self, variant: &ThemeVariant) -> Vec<&AdwaitaTheme> {
        self.themes.values()
            .filter(|t| t.variant == *variant)
            .collect()
    }
    
    /// Get themes by state
    pub fn get_themes_by_state(&self, state: &ThemeState) -> Vec<&AdwaitaTheme> {
        self.themes.values()
            .filter(|t| t.state == *state)
            .collect()
    }
    
    /// Get active themes
    pub fn get_active_themes(&self) -> Vec<&AdwaitaTheme> {
        self.get_themes_by_state(&ThemeState::Active)
    }
    
    /// Get dark themes
    pub fn get_dark_themes(&self) -> Vec<&AdwaitaTheme> {
        self.get_themes_by_variant(&ThemeVariant::Dark)
    }
    
    /// Get light themes
    pub fn get_light_themes(&self) -> Vec<&AdwaitaTheme> {
        self.get_themes_by_variant(&ThemeVariant::Light)
    }
    
    /// Search themes
    pub fn search_themes(&self, query: &str) -> Vec<&AdwaitaTheme> {
        let query_lower = query.to_lowercase();
        self.themes.values()
            .filter(|t| t.name.to_lowercase().contains(&query_lower))
            .collect()
    }
    
    // Stylesheet methods
    
    /// Create stylesheet
    pub fn create_stylesheet(&mut self, name: String) -> String {
        let sheet = Stylesheet::new(&name);
        let id = format!("sheet-{}", self.stylesheets.len() + 1);
        
        let mut sheet = sheet;
        sheet.name = name;
        
        self.stylesheets.insert(id.clone(), sheet);
        self.update_statistics();
        id
    }
    
    /// Get stylesheet
    pub fn get_stylesheet(&self, sheet_id: &str) -> Option<&Stylesheet> {
        self.stylesheets.get(sheet_id)
    }
    
    /// Get stylesheet mutably
    pub fn get_stylesheet_mut(&mut self, sheet_id: &str) -> Option<&mut Stylesheet> {
        self.stylesheets.get_mut(sheet_id)
    }
    
    /// Get all stylesheets
    pub fn list_stylesheets(&self) -> Vec<&Stylesheet> {
        self.stylesheets.values().collect()
    }
    
    /// Delete stylesheet
    pub fn delete_stylesheet(&mut self, sheet_id: &str) -> bool {
        if self.stylesheets.remove(sheet_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Add CSS rule to stylesheet
    pub fn add_css_rule(&mut self, sheet_id: &str, rule: CssRule) -> bool {
        if let Some(sheet) = self.stylesheets.get_mut(sheet_id) {
            sheet.add_rule(rule);
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Generate CSS for theme
    pub fn generate_css(&self, theme_id: &str) -> String {
        if let Some(theme) = self.themes.get(theme_id) {
            let mut css = String::new();
            
            // Window styles
            css.push_str("window {\n");
            css.push_str(&format!("  background-color: {};\n", 
                theme.colors.get_color(&NamedColor::WindowBg)
                    .map(|c| c.to_css())
                    .unwrap_or_else(|| "#ffffff".into())));
            css.push_str(&format!("  color: {};\n", 
                theme.colors.get_color(&NamedColor::WindowFg)
                    .map(|c| c.to_css())
                    .unwrap_or_else(|| "#000000".into())));
            css.push_str("}\n\n");
            
            // Button styles
            css.push_str("button {\n");
            css.push_str(&format!("  background-color: {};\n", 
                theme.colors.get_color(&NamedColor::Accent)
                    .map(|c| c.to_css())
                    .unwrap_or_else(|| "#6366f1".into())));
            css.push_str(&format!("  color: {};\n", 
                theme.colors.get_color(&NamedColor::AccentFg)
                    .map(|c| c.to_css())
                    .unwrap_or_else(|| "#ffffff".into())));
            css.push_str("  border-radius: 6px;\n");
            css.push_str("  padding: 6px 12px;\n");
            css.push_str("}\n\n");
            
            // Entry styles
            css.push_str("entry {\n");
            css.push_str(&format!("  background-color: {};\n", 
                theme.colors.get_color(&NamedColor::ViewBg)
                    .map(|c| c.to_css())
                    .unwrap_or_else(|| "#fafafa".into())));
            css.push_str(&format!("  color: {};\n", 
                theme.colors.get_color(&NamedColor::ViewFg)
                    .map(|c| c.to_css())
                    .unwrap_or_else(|| "#000000".into())));
            css.push_str("  border: 1px solid ");
            css.push_str(&theme.colors.get_color(&NamedColor::Border)
                .map(|c| c.to_css())
                .unwrap_or_else(|| "#e5e7eb".into()));
            css.push_str(";\n");
            css.push_str("  border-radius: 4px;\n");
            css.push_str("  padding: 4px 8px;\n");
            css.push_str("}\n");
            
            // Add CSS overrides
            for override_css in &theme.css_overrides {
                css.push_str("\n");
                css.push_str(override_css);
                css.push('\n');
            }
            
            css
        } else {
            String::new()
        }
    }
    
    /// Get manager uptime
    pub fn get_uptime(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.created_at
    }
    
    /// Update statistics
    fn update_statistics(&mut self) {
        self.statistics.total_themes = self.themes.len();
        self.statistics.active_themes = self.themes.values()
            .filter(|t| t.is_active())
            .count();
        self.statistics.dark_themes = self.themes.values()
            .filter(|t| t.is_dark())
            .count();
        self.statistics.light_themes = self.themes.values()
            .filter(|t| t.is_light())
            .count();
        self.statistics.total_widget_styles = self.themes.values()
            .map(|t| t.get_widget_style_count())
            .sum();
        self.statistics.total_css_rules = self.stylesheets.values()
            .map(|s| s.get_rule_count())
            .sum();
        self.statistics.uptime = self.get_uptime();
    }
    
    /// Get statistics
    pub fn get_statistics(&self) -> &ManagerStatistics {
        &self.statistics
    }
    
    /// Export manager data
    pub fn export_data(&self) -> ManagerExport {
        ManagerExport {
            themes: self.themes.values().cloned().collect(),
            stylesheets: self.stylesheets.values().cloned().collect(),
        }
    }
    
    /// Import manager data
    pub fn import_data(&mut self, export: ManagerExport) {
        self.themes.clear();
        for theme in export.themes {
            self.themes.insert(theme.name.clone(), theme);
        }
        
        self.stylesheets.clear();
        for sheet in export.stylesheets {
            self.stylesheets.insert(sheet.name.clone(), sheet);
        }
        
        self.update_statistics();
    }
}

/// Manager export data
#[derive(Debug, Clone)]
pub struct ManagerExport {
    /// Themes
    pub themes: Vec<AdwaitaTheme>,
    /// Stylesheets
    pub stylesheets: Vec<Stylesheet>,
}

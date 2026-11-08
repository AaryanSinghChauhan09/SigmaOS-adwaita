//! Theme definitions for Adwaita.

use crate::colors::ColorScheme;
use crate::widget::WidgetStyle;
use std::collections::HashMap;

/// Theme variant
#[derive(Debug, Clone, PartialEq)]
pub enum ThemeVariant {
    /// Light variant
    Light,
    /// Dark variant
    Dark,
}

/// Theme state
#[derive(Debug, Clone, PartialEq)]
pub enum ThemeState {
    /// Active theme
    Active,
    /// Preview theme
    Preview,
    /// Disabled theme
    Disabled,
}

/// Adwaita theme
#[derive(Debug, Clone)]
pub struct AdwaitaTheme {
    /// Theme name
    pub name: String,
    /// Theme variant
    pub variant: ThemeVariant,
    /// Theme state
    pub state: ThemeState,
    /// Theme colors
    pub colors: ColorScheme,
    /// Theme widget styles
    pub widget_styles: HashMap<String, WidgetStyle>,
    /// Theme CSS overrides
    pub css_overrides: Vec<String>,
    /// Theme created timestamp
    pub created_at: u64,
    /// Theme modified timestamp
    pub modified_at: u64,
}

impl AdwaitaTheme {
    /// Create new Adwaita theme
    pub fn new(name: String) -> Self {
        Self {
            name,
            variant: ThemeVariant::Light,
            state: ThemeState::Active,
            colors: ColorScheme::default_for_variant(&ThemeVariant::Light),
            widget_styles: HashMap::new(),
            css_overrides: vec![],
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            modified_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Set variant
    pub fn set_variant(&mut self, variant: ThemeVariant) {
        self.variant = variant.clone();
        self.colors = ColorScheme::default_for_variant(&variant);
        self.mark_modified();
    }
    
    /// Set state
    pub fn set_state(&mut self, state: ThemeState) {
        self.state = state;
        self.mark_modified();
    }
    
    /// Add widget style
    pub fn add_widget_style(&mut self, style: WidgetStyle) {
        self.widget_styles.insert(format!("{:?}", style.widget_class), style);
        self.mark_modified();
    }
    
    /// Get widget style
    pub fn get_widget_style(&self, class: &str) -> Option<&WidgetStyle> {
        self.widget_styles.get(class)
    }
    
    /// Add CSS override
    pub fn add_css_override(&mut self, css: String) {
        self.css_overrides.push(css);
        self.mark_modified();
    }
    
    /// Remove CSS override
    pub fn remove_css_override(&mut self, index: usize) -> bool {
        if index < self.css_overrides.len() {
            self.css_overrides.remove(index);
            self.mark_modified();
            return true;
        }
        false
    }
    
    /// Mark as modified
    fn mark_modified(&mut self) {
        self.modified_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    /// Check if theme is active
    pub fn is_active(&self) -> bool {
        self.state == ThemeState::Active
    }
    
    /// Check if theme is dark
    pub fn is_dark(&self) -> bool {
        self.variant == ThemeVariant::Dark
    }
    
    /// Check if theme is light
    pub fn is_light(&self) -> bool {
        self.variant == ThemeVariant::Light
    }
    
    /// Get widget style count
    pub fn get_widget_style_count(&self) -> usize {
        self.widget_styles.len()
    }
    
    /// Get CSS override count
    pub fn get_css_override_count(&self) -> usize {
        self.css_overrides.len()
    }
    
    /// Get theme age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.created_at
    }
    
    /// Get time since modification in seconds
    pub fn get_time_since_modification(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.modified_at
    }
}

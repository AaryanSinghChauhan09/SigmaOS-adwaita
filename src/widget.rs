//! Widget style definitions for Adwaita.

use std::collections::HashMap;

/// Widget class
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum WidgetClass {
    /// Window widget
    Window,
    /// Button widget
    Button,
    /// Entry widget
    Entry,
    /// Label widget
    Label,
    /// List box widget
    ListBox,
    /// Popover widget
    Popover,
    /// Dialog widget
    Dialog,
    /// Header bar widget
    HeaderBar,
    /// Sidebar widget
    Sidebar,
    /// Card widget
    Card,
    /// Switch widget
    Switch,
    /// Slider widget
    Slider,
    /// Progress bar widget
    ProgressBar,
}

/// Style property
#[derive(Debug, Clone, PartialEq)]
pub enum StyleProperty {
    /// Background color
    BackgroundColor(String),
    /// Foreground color
    ForegroundColor(String),
    /// Border color
    BorderColor(String),
    /// Border width
    BorderWidth(u32),
    /// Border radius
    BorderRadius(u32),
    /// Padding
    Padding(u32),
    /// Margin
    Margin(u32),
    /// Font size
    FontSize(u32),
    /// Font weight
    FontWeight(String),
    /// Opacity
    Opacity(f64),
    /// Custom property
    Custom(String, String),
}

/// Widget style
#[derive(Debug, Clone)]
pub struct WidgetStyle {
    /// Widget class
    pub widget_class: WidgetClass,
    /// Style properties
    pub properties: Vec<StyleProperty>,
    /// Pseudo-class states
    pub pseudo_states: Vec<String>,
    /// Created timestamp
    pub created_at: u64,
}

impl WidgetStyle {
    /// Create new widget style
    pub fn new(widget_class: WidgetClass) -> Self {
        Self {
            widget_class,
            properties: vec![],
            pseudo_states: vec![],
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Add property
    pub fn add_property(&mut self, property: StyleProperty) {
        self.properties.push(property);
    }
    
    /// Add pseudo-state
    pub fn add_pseudo_state(&mut self, state: String) {
        if !self.pseudo_states.contains(&state) {
            self.pseudo_states.push(state);
        }
    }
    
    /// Remove pseudo-state
    pub fn remove_pseudo_state(&mut self, state: &str) -> bool {
        if let Some(pos) = self.pseudo_states.iter().position(|s| s == state) {
            self.pseudo_states.remove(pos);
            return true;
        }
        false
    }
    
    /// Get property count
    pub fn get_property_count(&self) -> usize {
        self.properties.len()
    }
    
    /// Check if has properties
    pub fn has_properties(&self) -> bool {
        !self.properties.is_empty()
    }
    
    /// Get widget style age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.created_at
    }
}

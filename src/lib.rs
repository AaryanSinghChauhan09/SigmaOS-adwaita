//! SigmaOS-adwaita — Adwaita GTK theme engine component.
//!
//! Models GTK4/libadwaita theming: color schemes, CSS variants,
//! widget styling, dark/light preference, accent colors, and
//! stylesheet generation for the SigmaOS desktop.

pub mod theme;
pub mod colors;
pub mod widget;
pub mod stylesheet;
pub mod manager;
pub mod options;

pub use theme::{AdwaitaTheme, ThemeVariant, ThemeState};
pub use colors::{ColorScheme, NamedColor, RgbColor, ColorPalette};
pub use widget::{WidgetStyle, WidgetClass, StyleProperty};
pub use stylesheet::{Stylesheet, CssRule, CssSelector, CssDeclaration};
pub use manager::{AdwaitaManager, ManagerOptions, ManagerStatistics};
pub use options::{Mode, Options};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_theme_variant() {
        let variants = vec![
            ThemeVariant::Light,
            ThemeVariant::Dark,
        ];
        assert_eq!(variants.len(), 2);
    }
    
    #[test]
    fn test_theme_state() {
        let states = vec![
            ThemeState::Active,
            ThemeState::Preview,
            ThemeState::Disabled,
        ];
        assert_eq!(states.len(), 3);
    }
    
    #[test]
    fn test_named_color() {
        let colors = vec![
            NamedColor::WindowBg,
            NamedColor::WindowFg,
            NamedColor::Accent,
            NamedColor::Destructive,
            NamedColor::Success,
            NamedColor::Warning,
        ];
        assert_eq!(colors.len(), 6);
    }
    
    #[test]
    fn test_widget_class() {
        let classes = vec![
            WidgetClass::Window,
            WidgetClass::Button,
            WidgetClass::Entry,
            WidgetClass::Label,
            WidgetClass::ListBox,
            WidgetClass::Popover,
            WidgetClass::Dialog,
        ];
        assert_eq!(classes.len(), 7);
    }
    
    #[test]
    fn test_theme_creation() {
        let theme = AdwaitaTheme::new("default".into());
        assert_eq!(theme.name, "default");
        assert_eq!(theme.variant, ThemeVariant::Light);
        assert_eq!(theme.state, ThemeState::Active);
    }
    
    #[test]
    fn test_rgb_color() {
        let color = RgbColor::new(255, 0, 0);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
        assert_eq!(color.to_hex(), "#ff0000");
    }
    
    #[test]
    fn test_color_scheme() {
        let mut scheme = ColorScheme::new();
        scheme.set_color(NamedColor::Accent, RgbColor::new(99, 102, 241));
        
        assert!(scheme.get_color(&NamedColor::Accent).is_some());
    }
    
    #[test]
    fn test_widget_style() {
        let style = WidgetStyle::new(WidgetClass::Button);
        assert_eq!(style.widget_class, WidgetClass::Button);
        assert!(style.properties.is_empty());
    }
    
    #[test]
    fn test_stylesheet_creation() {
        let sheet = Stylesheet::new("main");
        assert_eq!(sheet.name, "main");
        assert!(sheet.rules.is_empty());
    }
    
    #[test]
    fn test_css_rule() {
        let mut rule = CssRule::new();
        rule.selector = CssSelector::Class("button".into());
        rule.add_declaration("background-color".into(), "#ff0000".into());
        
        assert_eq!(rule.declarations.len(), 1);
    }
    
    #[test]
    fn test_manager_creation() {
        let mut manager = AdwaitaManager::new();
        assert!(manager.list_themes().is_empty());
    }
    
    #[test]
    fn test_create_theme() {
        let mut manager = AdwaitaManager::new();
        let theme_id = manager.create_theme("Adwaita Dark".into());
        
        assert!(manager.get_theme(&theme_id).is_some());
        assert_eq!(manager.list_themes().len(), 1);
    }
    
    #[test]
    fn test_set_theme_variant() {
        let mut manager = AdwaitaManager::new();
        let theme_id = manager.create_theme("Test".into());
        
        assert!(manager.set_theme_variant(&theme_id, ThemeVariant::Dark));
        assert_eq!(manager.get_theme(&theme_id).unwrap().variant, ThemeVariant::Dark);
    }
    
    #[test]
    fn test_theme_colors() {
        let mut manager = AdwaitaManager::new();
        let theme_id = manager.create_theme("Test".into());
        
        assert!(manager.set_theme_color(&theme_id, NamedColor::Accent, RgbColor::new(255, 0, 0)));
        assert!(manager.get_theme(&theme_id).unwrap().colors.get_color(&NamedColor::Accent).is_some());
    }
    
    #[test]
    fn test_generate_stylesheet() {
        let mut manager = AdwaitaManager::new();
        let theme_id = manager.create_theme("Test".into());
        manager.set_theme_variant(&theme_id, ThemeVariant::Dark);
        
        let css = manager.generate_css(&theme_id);
        assert!(css.contains("window"));
        assert!(css.contains("background"));
    }
    
    #[test]
    fn test_export_import() {
        let mut manager = AdwaitaManager::new();
        let theme_id = manager.create_theme("Theme1".into());
        manager.set_theme_variant(&theme_id, ThemeVariant::Dark);
        
        let export = manager.export_data();
        
        let mut manager2 = AdwaitaManager::new();
        manager2.import_data(export);
        
        assert_eq!(manager2.list_themes().len(), 1);
    }
    
    #[test]
    fn test_manager_statistics() {
        let mut manager = AdwaitaManager::new();
        let light_id = manager.create_theme("Light".into());
        let dark_id = manager.create_theme("Dark".into());
        manager.set_theme_variant(&dark_id, ThemeVariant::Dark);
        
        let stats = manager.get_statistics();
        assert_eq!(stats.total_themes, 2);
        assert_eq!(stats.dark_themes, 1);
    }
}

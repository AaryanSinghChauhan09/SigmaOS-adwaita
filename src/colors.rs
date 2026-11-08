//! Color definitions for Adwaita theme.

/// Named color
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum NamedColor {
    /// Window background
    WindowBg,
    /// Window foreground
    WindowFg,
    /// View background
    ViewBg,
    /// View foreground
    ViewFg,
    /// Accent color
    Accent,
    /// Accent foreground
    AccentFg,
    /// Destructive color
    Destructive,
    /// Destructive foreground
    DestructiveFg,
    /// Success color
    Success,
    /// Success foreground
    SuccessFg,
    /// Warning color
    Warning,
    /// Warning foreground
    WarningFg,
    /// Card background
    CardBg,
    /// Card foreground
    CardFg,
    /// Popover background
    PopoverBg,
    /// Popover foreground
    PopoverFg,
    /// Border color
    Border,
    /// Selection background
    SelectionBg,
    /// Selection foreground
    SelectionFg,
}

/// RGB color
#[derive(Debug, Clone, PartialEq)]
pub struct RgbColor {
    /// Red component (0-255)
    pub r: u8,
    /// Green component (0-255)
    pub g: u8,
    /// Blue component (0-255)
    pub b: u8,
}

impl RgbColor {
    /// Create new RGB color
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
    
    /// Get hex string
    pub fn to_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
    
    /// Get CSS rgb string
    pub fn to_css(&self) -> String {
        format!("rgb({}, {}, {})", self.r, self.g, self.b)
    }
    
    /// Get CSS rgba string
    pub fn to_css_with_alpha(&self, alpha: f64) -> String {
        format!("rgba({}, {}, {}, {})", self.r, self.g, self.b, alpha)
    }
    
    /// Mix with another color
    pub fn mix(&self, other: &RgbColor, factor: f64) -> RgbColor {
        let factor = factor.clamp(0.0, 1.0);
        RgbColor::new(
            (self.r as f64 * (1.0 - factor) + other.r as f64 * factor) as u8,
            (self.g as f64 * (1.0 - factor) + other.g as f64 * factor) as u8,
            (self.b as f64 * (1.0 - factor) + other.b as f64 * factor) as u8,
        )
    }
    
    /// Lighten color
    pub fn lighten(&self, amount: f64) -> RgbColor {
        let white = RgbColor::new(255, 255, 255);
        self.mix(&white, amount.clamp(0.0, 1.0))
    }
    
    /// Darken color
    pub fn darken(&self, amount: f64) -> RgbColor {
        let black = RgbColor::new(0, 0, 0);
        self.mix(&black, amount.clamp(0.0, 1.0))
    }
}

/// Color scheme
#[derive(Debug, Clone)]
pub struct ColorScheme {
    /// Colors map
    pub colors: std::collections::HashMap<NamedColor, RgbColor>,
    /// Created timestamp
    pub created_at: u64,
}

impl ColorScheme {
    /// Create new color scheme
    pub fn new() -> Self {
        Self {
            colors: std::collections::HashMap::new(),
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Create default color scheme for variant
    pub fn default_for_variant(variant: &crate::theme::ThemeVariant) -> Self {
        let mut scheme = Self::new();
        
        match variant {
            crate::theme::ThemeVariant::Light => {
                scheme.set_color(NamedColor::WindowBg, RgbColor::new(255, 255, 255));
                scheme.set_color(NamedColor::WindowFg, RgbColor::new(0, 0, 0));
                scheme.set_color(NamedColor::ViewBg, RgbColor::new(250, 250, 250));
                scheme.set_color(NamedColor::ViewFg, RgbColor::new(0, 0, 0));
                scheme.set_color(NamedColor::Accent, RgbColor::new(99, 102, 241));
                scheme.set_color(NamedColor::AccentFg, RgbColor::new(255, 255, 255));
                scheme.set_color(NamedColor::Destructive, RgbColor::new(220, 38, 38));
                scheme.set_color(NamedColor::DestructiveFg, RgbColor::new(255, 255, 255));
                scheme.set_color(NamedColor::Border, RgbColor::new(229, 231, 235));
            }
            crate::theme::ThemeVariant::Dark => {
                scheme.set_color(NamedColor::WindowBg, RgbColor::new(30, 30, 30));
                scheme.set_color(NamedColor::WindowFg, RgbColor::new(255, 255, 255));
                scheme.set_color(NamedColor::ViewBg, RgbColor::new(40, 40, 40));
                scheme.set_color(NamedColor::ViewFg, RgbColor::new(255, 255, 255));
                scheme.set_color(NamedColor::Accent, RgbColor::new(129, 140, 248));
                scheme.set_color(NamedColor::AccentFg, RgbColor::new(255, 255, 255));
                scheme.set_color(NamedColor::Destructive, RgbColor::new(248, 113, 113));
                scheme.set_color(NamedColor::DestructiveFg, RgbColor::new(30, 30, 30));
                scheme.set_color(NamedColor::Border, RgbColor::new(55, 55, 55));
            }
        }
        
        scheme
    }
    
    /// Set color
    pub fn set_color(&mut self, name: NamedColor, color: RgbColor) {
        self.colors.insert(name, color);
    }
    
    /// Get color
    pub fn get_color(&self, name: &NamedColor) -> Option<&RgbColor> {
        self.colors.get(name)
    }
    
    /// Remove color
    pub fn remove_color(&mut self, name: &NamedColor) -> bool {
        self.colors.remove(name).is_some()
    }
    
    /// Get color count
    pub fn get_color_count(&self) -> usize {
        self.colors.len()
    }
    
    /// Check if color exists
    pub fn has_color(&self, name: &NamedColor) -> bool {
        self.colors.contains_key(name)
    }
    
    /// Clear all colors
    pub fn clear(&mut self) {
        self.colors.clear();
    }
    
    /// Get scheme age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.created_at
    }
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self::new()
    }
}

/// Color palette
#[derive(Debug, Clone)]
pub struct ColorPalette {
    /// Palette name
    pub name: String,
    /// Palette colors
    pub colors: Vec<RgbColor>,
    /// Palette created timestamp
    pub created_at: u64,
}

impl ColorPalette {
    /// Create new color palette
    pub fn new(name: String) -> Self {
        Self {
            name,
            colors: vec![],
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Add color
    pub fn add_color(&mut self, color: RgbColor) {
        self.colors.push(color);
    }
    
    /// Get color count
    pub fn get_color_count(&self) -> usize {
        self.colors.len()
    }
    
    /// Get color at index
    pub fn get_color(&self, index: usize) -> Option<&RgbColor> {
        self.colors.get(index)
    }
    
    /// Get palette age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.created_at
    }
}

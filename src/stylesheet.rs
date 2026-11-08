//! Stylesheet generation for Adwaita.

/// CSS selector
#[derive(Debug, Clone, PartialEq)]
pub enum CssSelector {
    /// Element selector
    Element(String),
    /// Class selector
    Class(String),
    /// ID selector
    Id(String),
    /// Pseudo-class selector
    PseudoClass(String, String),
    /// Child selector
    Child(Box<CssSelector>, Box<CssSelector>),
    /// Descendant selector
    Descendant(Box<CssSelector>, Box<CssSelector>),
}

impl CssSelector {
    /// Convert to CSS string
    pub fn to_css(&self) -> String {
        match self {
            CssSelector::Element(name) => name.clone(),
            CssSelector::Class(name) => format!(".{}", name),
            CssSelector::Id(name) => format!("#{}", name),
            CssSelector::PseudoClass(class, pseudo) => format!(".{}:{}", class, pseudo),
            CssSelector::Child(parent, child) => format!("{} > {}", parent.to_css(), child.to_css()),
            CssSelector::Descendant(parent, child) => format!("{} {}", parent.to_css(), child.to_css()),
        }
    }
}

/// CSS declaration
#[derive(Debug, Clone, PartialEq)]
pub struct CssDeclaration {
    /// Property name
    pub property: String,
    /// Property value
    pub value: String,
    /// Important flag
    pub important: bool,
}

/// CSS rule
#[derive(Debug, Clone)]
pub struct CssRule {
    /// Rule selector
    pub selector: CssSelector,
    /// Rule declarations
    pub declarations: Vec<CssDeclaration>,
    /// Rule media query
    pub media_query: Option<String>,
}

impl CssRule {
    /// Create new CSS rule
    pub fn new() -> Self {
        Self {
            selector: CssSelector::Element("*".into()),
            declarations: vec![],
            media_query: None,
        }
    }
    
    /// Add declaration
    pub fn add_declaration(&mut self, property: String, value: String) {
        self.declarations.push(CssDeclaration {
            property,
            value,
            important: false,
        });
    }
    
    /// Add important declaration
    pub fn add_important_declaration(&mut self, property: String, value: String) {
        self.declarations.push(CssDeclaration {
            property,
            value,
            important: true,
        });
    }
    
    /// Set media query
    pub fn set_media_query(&mut self, query: String) {
        self.media_query = Some(query);
    }
    
    /// Convert to CSS string
    pub fn to_css(&self) -> String {
        let mut css = String::new();
        
        if let Some(media) = &self.media_query {
            css.push_str(&format!("@media {} {{\n", media));
        }
        
        css.push_str(&format!("{} {{\n", self.selector.to_css()));
        for decl in &self.declarations {
            let important = if decl.important { " !important" } else { "" };
            css.push_str(&format!("  {}: {}{};\n", decl.property, decl.value, important));
        }
        css.push_str("}\n");
        
        if self.media_query.is_some() {
            css.push_str("}\n");
        }
        
        css
    }
}

impl Default for CssRule {
    fn default() -> Self {
        Self::new()
    }
}

/// Stylesheet
#[derive(Debug, Clone)]
pub struct Stylesheet {
    /// Stylesheet name
    pub name: String,
    /// Stylesheet rules
    pub rules: Vec<CssRule>,
    /// Created timestamp
    pub created_at: u64,
    /// Modified timestamp
    pub modified_at: u64,
}

impl Stylesheet {
    /// Create new stylesheet
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            rules: vec![],
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
    
    /// Add rule
    pub fn add_rule(&mut self, rule: CssRule) {
        self.rules.push(rule);
        self.modified_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    /// Remove rule
    pub fn remove_rule(&mut self, index: usize) -> bool {
        if index < self.rules.len() {
            self.rules.remove(index);
            self.modified_at = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            return true;
        }
        false
    }
    
    /// Get rule count
    pub fn get_rule_count(&self) -> usize {
        self.rules.len()
    }
    
    /// Convert to CSS string
    pub fn to_css(&self) -> String {
        let mut css = String::new();
        for rule in &self.rules {
            css.push_str(&rule.to_css());
            css.push('\n');
        }
        css
    }
    
    /// Get stylesheet age in seconds
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

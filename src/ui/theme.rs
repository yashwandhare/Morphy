/// theme.rs - centralized color configuration for the app

/// calm, earthy cli palette (everforest-inspired)
pub struct Theme;

impl Theme {
    pub const HEADER: console::Color = console::Color::Color256(144);  // sage green
    pub const BORDER: console::Color = console::Color::Color256(245);  // muted gray-green
    pub const SUCCESS: console::Color = console::Color::Color256(108); // cool green
    pub const ERROR: console::Color = console::Color::Color256(174);   // soft coral
    pub const INFO: console::Color = console::Color::Color256(109);    // dusty teal
    pub const TEXT: console::Color = console::Color::Color256(187);    // warm cream
    pub const PROMPT: console::Color = console::Color::Color256(180);  // soft gold
    pub const DIM: console::Color = console::Color::Color256(245);     // subtle gray
}

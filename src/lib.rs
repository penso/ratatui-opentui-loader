//! # ratatui-opentui-loader
//!
//! A KITT-style (Knight Rider) scanner/loader widget for
//! [ratatui](https://ratatui.rs), inspired by the opencode/opentui spinner.
//!
//! A bright dot bounces left and right with a fading color trail behind it,
//! using block characters (`■` / `⬝`). During the brief pause at each edge,
//! all inactive dots fade out and then fade back in as the dot resumes — just
//! like the original opencode loader.
//!
//! ## Quick start
//!
//! ```rust,ignore
//! use ratatui_opentui_loader::KittLoader;
//!
//! let mut loader = KittLoader::new();
//! // each tick (~40ms), call loader.tick() then render:
//! loader.tick();
//! frame.render_widget(&loader, area);
//! ```

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

/// Pre-built color themes matching opencode's theme collection.
///
/// Each variant uses the dark-mode primary color from the corresponding
/// opencode theme. Use [`Theme::Custom`] for any color not listed here.
#[derive(Debug, Clone, Copy)]
pub enum Theme {
    Opencode,
    Dracula,
    Gruvbox,
    Catppuccin,
    CatppuccinFrappe,
    CatppuccinMacchiato,
    Nord,
    Tokyonight,
    Solarized,
    Rosepine,
    Ayu,
    Monokai,
    OneDark,
    Kanagawa,
    Material,
    Everforest,
    Github,
    Amoled,
    Aura,
    Carbonfox,
    Cobalt2,
    Cursor,
    Flexoki,
    Matrix,
    Mercury,
    Nightowl,
    Palenight,
    ShadesOfPurple,
    Synthwave84,
    Vesper,
    Zenburn,
    Vercel,
    Orng,
    OsakaJade,
    /// Custom single accent color
    Custom(Color),
}

impl Theme {
    /// Returns all built-in theme variants (excludes `Custom`).
    pub fn all() -> &'static [Theme] {
        &[
            Theme::Opencode,
            Theme::Dracula,
            Theme::Gruvbox,
            Theme::Catppuccin,
            Theme::CatppuccinFrappe,
            Theme::CatppuccinMacchiato,
            Theme::Nord,
            Theme::Tokyonight,
            Theme::Solarized,
            Theme::Rosepine,
            Theme::Ayu,
            Theme::Monokai,
            Theme::OneDark,
            Theme::Kanagawa,
            Theme::Material,
            Theme::Everforest,
            Theme::Github,
            Theme::Amoled,
            Theme::Aura,
            Theme::Carbonfox,
            Theme::Cobalt2,
            Theme::Cursor,
            Theme::Flexoki,
            Theme::Matrix,
            Theme::Mercury,
            Theme::Nightowl,
            Theme::Palenight,
            Theme::ShadesOfPurple,
            Theme::Synthwave84,
            Theme::Vesper,
            Theme::Zenburn,
            Theme::Vercel,
            Theme::Orng,
            Theme::OsakaJade,
        ]
    }

    /// Human-readable name matching the opencode theme id.
    pub fn name(&self) -> &'static str {
        match self {
            Theme::Opencode => "opencode",
            Theme::Dracula => "dracula",
            Theme::Gruvbox => "gruvbox",
            Theme::Catppuccin => "catppuccin",
            Theme::CatppuccinFrappe => "catppuccin-frappe",
            Theme::CatppuccinMacchiato => "catppuccin-macchiato",
            Theme::Nord => "nord",
            Theme::Tokyonight => "tokyonight",
            Theme::Solarized => "solarized",
            Theme::Rosepine => "rosepine",
            Theme::Ayu => "ayu",
            Theme::Monokai => "monokai",
            Theme::OneDark => "one-dark",
            Theme::Kanagawa => "kanagawa",
            Theme::Material => "material",
            Theme::Everforest => "everforest",
            Theme::Github => "github",
            Theme::Amoled => "amoled",
            Theme::Aura => "aura",
            Theme::Carbonfox => "carbonfox",
            Theme::Cobalt2 => "cobalt2",
            Theme::Cursor => "cursor",
            Theme::Flexoki => "flexoki",
            Theme::Matrix => "matrix",
            Theme::Mercury => "mercury",
            Theme::Nightowl => "nightowl",
            Theme::Palenight => "palenight",
            Theme::ShadesOfPurple => "shadesofpurple",
            Theme::Synthwave84 => "synthwave84",
            Theme::Vesper => "vesper",
            Theme::Zenburn => "zenburn",
            Theme::Vercel => "vercel",
            Theme::Orng => "orng",
            Theme::OsakaJade => "osaka-jade",
            Theme::Custom(_) => "custom",
        }
    }

    fn accent(&self) -> Color {
        match self {
            Theme::Opencode => Color::Rgb(0xfa, 0xb2, 0x83),
            Theme::Dracula => Color::Rgb(0xbd, 0x93, 0xf9),
            Theme::Gruvbox => Color::Rgb(0x83, 0xa5, 0x98),
            Theme::Catppuccin => Color::Rgb(0xb4, 0xbe, 0xfe),
            Theme::CatppuccinFrappe => Color::Rgb(0x8d, 0xa4, 0xe2),
            Theme::CatppuccinMacchiato => Color::Rgb(0x8a, 0xad, 0xf4),
            Theme::Nord => Color::Rgb(0x88, 0xc0, 0xd0),
            Theme::Tokyonight => Color::Rgb(0x7a, 0xa2, 0xf7),
            Theme::Solarized => Color::Rgb(0x6c, 0x71, 0xc4),
            Theme::Rosepine => Color::Rgb(0x9c, 0xcf, 0xd8),
            Theme::Ayu => Color::Rgb(0x3f, 0xb7, 0xe3),
            Theme::Monokai => Color::Rgb(0xae, 0x81, 0xff),
            Theme::OneDark => Color::Rgb(0x61, 0xaf, 0xef),
            Theme::Kanagawa => Color::Rgb(0x7e, 0x9c, 0xd8),
            Theme::Material => Color::Rgb(0x82, 0xaa, 0xff),
            Theme::Everforest => Color::Rgb(0xa7, 0xc0, 0x80),
            Theme::Github => Color::Rgb(0x58, 0xa6, 0xff),
            Theme::Amoled => Color::Rgb(0xb3, 0x88, 0xff),
            Theme::Aura => Color::Rgb(0xa2, 0x77, 0xff),
            Theme::Carbonfox => Color::Rgb(0x33, 0xb1, 0xff),
            Theme::Cobalt2 => Color::Rgb(0x00, 0x88, 0xff),
            Theme::Cursor => Color::Rgb(0x88, 0xc0, 0xd0),
            Theme::Flexoki => Color::Rgb(0xda, 0x70, 0x2c),
            Theme::Matrix => Color::Rgb(0x2e, 0xff, 0x6a),
            Theme::Mercury => Color::Rgb(0x8d, 0xa4, 0xf5),
            Theme::Nightowl => Color::Rgb(0x82, 0xaa, 0xff),
            Theme::Palenight => Color::Rgb(0x82, 0xaa, 0xff),
            Theme::ShadesOfPurple => Color::Rgb(0xc7, 0x92, 0xff),
            Theme::Synthwave84 => Color::Rgb(0x36, 0xf9, 0xf6),
            Theme::Vesper => Color::Rgb(0xff, 0xc7, 0x99),
            Theme::Zenburn => Color::Rgb(0x8c, 0xd0, 0xd3),
            Theme::Vercel => Color::Rgb(0x00, 0x70, 0xf3),
            Theme::Orng => Color::Rgb(0xec, 0x5b, 0x2b),
            Theme::OsakaJade => Color::Rgb(0x2d, 0xd5, 0xb7),
            Theme::Custom(c) => *c,
        }
    }
}

fn rgb_components(c: Color) -> (u8, u8, u8) {
    match c {
        Color::Rgb(r, g, b) => (r, g, b),
        _ => (255, 0, 0),
    }
}

/// Derive a trail of colors from a single accent color.
fn derive_trail(accent: Color, steps: usize) -> Vec<Color> {
    let (r, g, b) = rgb_components(accent);
    (0..steps)
        .map(|i| {
            if i == 0 {
                // Head: full brightness
                Color::Rgb(r, g, b)
            } else {
                let factor = 0.65_f64.powi(i as i32);
                Color::Rgb(
                    (r as f64 * factor) as u8,
                    (g as f64 * factor) as u8,
                    (b as f64 * factor) as u8,
                )
            }
        })
        .collect()
}

/// Derive the dim "inactive dot" color, scaled by `factor` (0.0–1.0).
fn derive_inactive(accent: Color, factor: f64) -> Color {
    let (r, g, b) = rgb_components(accent);
    Color::Rgb(
        (r as f64 * factor) as u8,
        (g as f64 * factor) as u8,
        (b as f64 * factor) as u8,
    )
}

/// Snapshot of where the scanner head is on a given frame.
struct ScannerState {
    active_pos: usize,
    is_forward: bool,
    is_holding: bool,
    /// 0.0–1.0 progress through the current hold phase
    hold_progress: f64,
    /// Absolute frame count into the hold (not normalized)
    hold_frame: usize,
}

/// A KITT-style scanner loader widget.
///
/// Call [`tick()`](KittLoader::tick) each frame (~40ms) to advance the
/// animation, then render with `frame.render_widget(&loader, area)`.
#[derive(Debug, Clone)]
pub struct KittLoader {
    width: usize,
    trail_colors: Vec<Color>,
    inactive_color: Color,
    accent: Color,
    inactive_factor: f64,
    /// Minimum brightness multiplier at full fade (0.0 = fully dark, 1.0 = no fade)
    min_fade: f64,
    /// If true, head is darkest and trail gets brighter (for light backgrounds)
    inverted: bool,
    frame_index: usize,
    total_frames: usize,
    hold_start: usize,
    hold_end: usize,
}

impl KittLoader {
    /// Create a loader with default settings (width 8, opencode theme).
    pub fn new() -> Self {
        Self::with_theme(Theme::Opencode)
    }

    /// Create a loader with a specific theme.
    pub fn with_theme(theme: Theme) -> Self {
        Self::with_color(theme.accent())
    }

    /// Create a loader with a custom accent color.
    pub fn with_color(accent: Color) -> Self {
        Self::build(accent, 8, 6, 20, 4, 0.25, 0.55)
    }

    /// Full builder.
    ///
    /// - `accent` – the bright head color
    /// - `width` – number of character cells
    /// - `trail_steps` – length of the fading trail
    /// - `hold_start` – frames to pause at the left edge
    /// - `hold_end` – frames to pause at the right edge
    /// - `inactive_factor` – brightness multiplier for inactive dots (0.0–1.0)
    /// - `min_fade` – minimum brightness at full fade-out (0.0–1.0)
    pub fn build(
        accent: Color,
        width: usize,
        trail_steps: usize,
        hold_start: usize,
        hold_end: usize,
        inactive_factor: f64,
        min_fade: f64,
    ) -> Self {
        let trail_colors = derive_trail(accent, trail_steps);
        let inactive_color = derive_inactive(accent, inactive_factor);
        let total_frames = width + hold_end + (width - 1) + hold_start;

        Self {
            width,
            trail_colors,
            inactive_color,
            accent,
            inactive_factor,
            min_fade,
            inverted: false,
            frame_index: 0,
            total_frames,
            hold_start,
            hold_end,
        }
    }

    /// Change the theme at runtime.
    pub fn set_theme(&mut self, theme: Theme) {
        self.set_color(theme.accent());
    }

    /// Invert the trail gradient (darkest at head, brightest in tail).
    /// Use this on light terminal backgrounds where the accent color
    /// would otherwise blend into the background.
    pub fn inverted(mut self, inv: bool) -> Self {
        self.inverted = inv;
        self
    }

    /// Change the accent color at runtime.
    pub fn set_color(&mut self, accent: Color) {
        self.accent = accent;
        self.trail_colors = derive_trail(accent, self.trail_colors.len());
        self.inactive_color = derive_inactive(accent, self.inactive_factor);
    }

    /// Advance the animation by one frame.
    pub fn tick(&mut self) {
        self.frame_index = (self.frame_index + 1) % self.total_frames;
    }

    fn scanner_state(&self) -> ScannerState {
        let fi = self.frame_index;
        let w = self.width;
        let he = self.hold_end;
        let hs = self.hold_start;
        let backward_frames = w - 1;

        if fi < w {
            ScannerState {
                active_pos: fi,
                is_forward: true,
                is_holding: false,
                hold_progress: 0.0,
                hold_frame: 0,
            }
        } else if fi < w + he {
            let p = fi - w;
            ScannerState {
                active_pos: w - 1,
                is_forward: true,
                is_holding: true,
                hold_progress: if he > 0 { p as f64 / he as f64 } else { 1.0 },
                hold_frame: p,
            }
        } else if fi < w + he + backward_frames {
            let back_i = fi - w - he;
            ScannerState {
                active_pos: w - 2 - back_i,
                is_forward: false,
                is_holding: false,
                hold_progress: 0.0,
                hold_frame: 0,
            }
        } else {
            let p = fi - w - he - backward_frames;
            ScannerState {
                active_pos: 0,
                is_forward: false,
                is_holding: true,
                hold_progress: if hs > 0 { p as f64 / hs as f64 } else { 1.0 },
                hold_frame: p,
            }
        }
    }

    /// Render to a [`Line`] with explicit width.
    pub fn into_line(&self, render_width: usize) -> Line<'static> {
        let w = self.width.min(render_width);
        if w == 0 {
            return Line::default();
        }

        let state = self.scanner_state();

        // Compute the global fade factor for inactive dots.
        // During hold: fade from 1.0 down to min_fade (breathing out).
        // During movement: always full brightness — no slowdown.
        let fade = if state.is_holding {
            let p = state.hold_progress.min(1.0);
            1.0 - p * (1.0 - self.min_fade)
        } else {
            1.0
        };

        // Pre-compute the faded inactive color
        let faded_inactive = self.apply_fade(self.inactive_color, fade);

        let spans: Vec<Span<'static>> = (0..w)
            .map(|i| {
                // Directional distance: positive = trailing behind the head
                let dist = if state.is_forward {
                    state.active_pos as i32 - i as i32
                } else {
                    i as i32 - state.active_pos as i32
                };

                // During hold, shift trail by absolute frame count (1 per frame)
                // so the trail dissolves at the same speed on both edges
                let effective_dist = if state.is_holding {
                    dist + state.hold_frame as i32
                } else {
                    dist
                };

                if effective_dist >= 0 && (effective_dist as usize) < self.trail_colors.len() {
                    let idx = if self.inverted {
                        self.trail_colors.len() - 1 - effective_dist as usize
                    } else {
                        effective_dist as usize
                    };
                    let color = self.trail_colors[idx];
                    Span::styled("■".to_string(), Style::default().fg(color))
                } else {
                    Span::styled("⬝".to_string(), Style::default().fg(faded_inactive))
                }
            })
            .collect();

        Line::from(spans)
    }

    /// Scale a color's brightness by `fade` (0.0–1.0).
    fn apply_fade(&self, color: Color, fade: f64) -> Color {
        let (r, g, b) = rgb_components(color);
        Color::Rgb(
            (r as f64 * fade) as u8,
            (g as f64 * fade) as u8,
            (b as f64 * fade) as u8,
        )
    }
}

impl Default for KittLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for &KittLoader {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(self.into_line(area.width as usize)).render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_creates_8_wide() {
        let loader = KittLoader::new();
        assert_eq!(loader.width, 8);
        assert_eq!(loader.trail_colors.len(), 6);
    }

    #[test]
    fn tick_wraps_around() {
        let mut loader = KittLoader::new();
        for _ in 0..loader.total_frames {
            loader.tick();
        }
        assert_eq!(loader.frame_index, 0);
    }

    #[test]
    fn into_line_correct_width() {
        let loader = KittLoader::new();
        let line = loader.into_line(8);
        assert_eq!(line.spans.len(), 8);
    }

    #[test]
    fn zero_width_line() {
        let loader = KittLoader::new();
        let line = loader.into_line(0);
        assert!(line.spans.is_empty());
    }

    #[test]
    fn theme_changes_color() {
        let mut loader = KittLoader::with_theme(Theme::Dracula);
        assert_eq!(loader.trail_colors[0], Color::Rgb(0xbd, 0x93, 0xf9));
        loader.set_theme(Theme::Matrix);
        assert_eq!(loader.trail_colors[0], Color::Rgb(0x2e, 0xff, 0x6a));
    }

    #[test]
    fn fading_during_hold() {
        let mut loader = KittLoader::new();
        // Advance through forward + hold_end + backward to reach hold-at-start
        let ticks_to_hold_start = loader.width + loader.hold_end + (loader.width - 1);
        for _ in 0..ticks_to_hold_start {
            loader.tick();
        }
        let state = loader.scanner_state();
        assert!(state.is_holding);
        assert_eq!(state.active_pos, 0);
    }

    #[test]
    fn fade_at_hold_produces_dimmer_color() {
        let mut loader = KittLoader::new();
        // Advance to the hold-at-start phase
        let ticks_to_hold_start = loader.width + loader.hold_end + (loader.width - 1);
        for _ in 0..ticks_to_hold_start {
            loader.tick();
        }
        let line_start = loader.into_line(8);
        // Advance near end of hold
        for _ in 0..loader.hold_start - 1 {
            loader.tick();
        }
        let line_end = loader.into_line(8);
        // The inactive dot at position 7 should be dimmer at end of hold
        let start_fg = line_start.spans[7].style.fg.unwrap();
        let end_fg = line_end.spans[7].style.fg.unwrap();
        let (sr, _, _) = rgb_components(start_fg);
        let (er, _, _) = rgb_components(end_fg);
        assert!(er <= sr, "inactive dot should get dimmer during hold");
    }
}

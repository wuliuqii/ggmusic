use gpui::{AppContext, Global, Hsla, SharedString, WindowAppearance, WindowBackgroundAppearance};
use log::error;

fn color_to_hsla(color: catppuccin::Color) -> Hsla {
    Hsla {
        h: color.hsl.h as f32,
        s: color.hsl.s as f32,
        l: color.hsl.l as f32,
        a: 1.0,
    }
}

fn load_fonts(cx: &mut AppContext) -> gpui::Result<()> {
    let font_paths = cx.asset_source().list("fonts")?;
    let mut embedded_fonts = Vec::new();
    for font_path in font_paths {
        if font_path.ends_with(".ttf") {
            let font_bytes = cx.asset_source().load(&font_path)?;
            if let Some(font_bytes) = font_bytes {
                embedded_fonts.push(font_bytes);
            }
        }
    }
    cx.text_system().add_fonts(embedded_fonts)
}

#[derive(Debug, Default, Clone)]
pub enum WindowBackgroundAppearanceContent {
    Blurred {
        opacity: f32,
    },
    Transparent {
        opacity: f32,
    },
    #[default]
    Opaque,
}

impl From<WindowBackgroundAppearanceContent> for WindowBackgroundAppearance {
    fn from(content: WindowBackgroundAppearanceContent) -> Self {
        match content {
            WindowBackgroundAppearanceContent::Blurred { .. } => {
                WindowBackgroundAppearance::Blurred
            }
            WindowBackgroundAppearanceContent::Transparent { .. } => {
                WindowBackgroundAppearance::Transparent
            }
            WindowBackgroundAppearanceContent::Opaque => WindowBackgroundAppearance::Opaque,
        }
    }
}

impl WindowBackgroundAppearanceContent {
    pub fn opacity(&self) -> f32 {
        match self {
            WindowBackgroundAppearanceContent::Blurred { opacity }
            | WindowBackgroundAppearanceContent::Transparent { opacity } => *opacity,
            WindowBackgroundAppearanceContent::Opaque => 1.0,
        }
    }
}

struct ThemeSettings {
    pub light: String,
    pub dark: String,
}

impl Default for ThemeSettings {
    fn default() -> Self {
        Self {
            light: "Catppuccin Latte".into(),
            dark: "Catppuccin Macchiato".into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub name: SharedString,
    pub font_sans: SharedString,
    pub font_mono: SharedString,
    pub window_background: Option<WindowBackgroundAppearanceContent>,
    pub flamingo: Hsla,
    pub pink: Hsla,
    pub mauve: Hsla,
    pub red: Hsla,
    pub maroon: Hsla,
    pub peach: Hsla,
    pub yellow: Hsla,
    pub green: Hsla,
    pub teal: Hsla,
    pub sky: Hsla,
    pub sapphire: Hsla,
    pub blue: Hsla,
    pub lavender: Hsla,
    pub text: Hsla,
    pub subtext1: Hsla,
    pub subtext0: Hsla,
    pub overlay2: Hsla,
    pub overlay1: Hsla,
    pub overlay0: Hsla,
    pub surface2: Hsla,
    pub surface1: Hsla,
    pub surface0: Hsla,
    pub base: Hsla,
    pub mantle: Hsla,
    pub crust: Hsla,
}

impl From<catppuccin::Flavor> for Theme {
    fn from(flavor: catppuccin::Flavor) -> Self {
        let colors = flavor.colors;
        let name = flavor.name;

        Self {
            name: format!("Catppuccin {}", name).into(),
            font_sans: "Inter".into(),
            font_mono: "JetBrains Mono".into(),
            window_background: Some(WindowBackgroundAppearanceContent::Blurred { opacity: 0.9 }),
            flamingo: color_to_hsla(colors.flamingo),
            pink: color_to_hsla(colors.pink),
            mauve: color_to_hsla(colors.mauve),
            red: color_to_hsla(colors.red),
            maroon: color_to_hsla(colors.maroon),
            peach: color_to_hsla(colors.peach),
            yellow: color_to_hsla(colors.yellow),
            green: color_to_hsla(colors.green),
            teal: color_to_hsla(colors.teal),
            sky: color_to_hsla(colors.sky),
            sapphire: color_to_hsla(colors.sapphire),
            blue: color_to_hsla(colors.blue),
            lavender: color_to_hsla(colors.lavender),
            text: color_to_hsla(colors.text),
            subtext1: color_to_hsla(colors.subtext1),
            subtext0: color_to_hsla(colors.subtext0),
            overlay2: color_to_hsla(colors.overlay2),
            overlay1: color_to_hsla(colors.overlay1),
            overlay0: color_to_hsla(colors.overlay0),
            surface2: color_to_hsla(colors.surface2),
            surface1: color_to_hsla(colors.surface1),
            surface0: color_to_hsla(colors.surface0),
            base: color_to_hsla(colors.base),
            mantle: color_to_hsla(colors.mantle),
            crust: color_to_hsla(colors.crust),
        }
    }
}

impl Theme {
    pub fn init(cx: &mut AppContext) {
        load_fonts(cx).expect("Failed to load fonts");
        let appearance = cx.window_appearance();
        let theme = Self::mode(appearance);

        cx.set_global(theme);
    }

    pub fn mode(mode: WindowAppearance) -> Self {
        let list = Self::list();
        let setting = ThemeSettings::default();
        let name = match mode {
            WindowAppearance::Dark | WindowAppearance::VibrantDark => setting.dark,
            WindowAppearance::Light | WindowAppearance::VibrantLight => setting.light,
        };

        list.clone()
            .into_iter()
            .find(|t| t.name == name)
            .unwrap_or_else(|| {
                error!("Theme not found: {}", name);
                list.first().unwrap().clone()
            })
    }

    fn list() -> Vec<Self> {
        // TODO: user themes
        catppuccin::PALETTE
            .all_flavors()
            .into_iter()
            .map(|flavor| Self::from(*flavor))
            .collect()
    }
}

impl Global for Theme {}

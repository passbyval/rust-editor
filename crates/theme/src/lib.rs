use egui::{Color32, Rounding, Vec2};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref GREY: [Color32; 10] = [
        Color32::from_hex("#fafafa").unwrap(),
        Color32::from_hex("#f5f5f5").unwrap(),
        Color32::from_hex("#eeeeee").unwrap(),
        Color32::from_hex("#e0e0e0").unwrap(),
        Color32::from_hex("#bdbdbd").unwrap(),
        Color32::from_hex("#9e9e9e").unwrap(),
        Color32::from_hex("#757575").unwrap(),
        Color32::from_hex("#616161").unwrap(),
        Color32::from_hex("#424242").unwrap(),
        Color32::from_hex("#212121").unwrap(),
    ];
}

pub struct Action {
    pub active: Color32,
    pub hover: Color32,
    pub selected: Color32,
    pub disabled: Color32,
    pub disabled_bg: Color32,
    pub focus: Color32,
}

pub struct Palette {
    pub main: Color32,
    pub light: Color32,
    pub dark: Color32,
    pub contrast_text: Color32,
}

pub struct TextColor {
    pub primary: Color32,
    pub secondary: Color32,
    pub disabled: Color32,
    pub icon: Color32,
}

pub struct Theme {
    pub rounding: Rounding,
    pub padding: Vec2,

    pub primary: Palette,
    pub secondary: Palette,
    pub action: Action,
    pub text_color: TextColor,
    pub bg: Color32,
}

impl Theme {
    pub const ROUNDING: Rounding = Rounding {
        nw: 3.0,
        ne: 3.0,
        sw: 3.0,
        se: 3.0,
    };

    pub const ROUNDING_NONE: Rounding = Rounding {
        nw: 0.0,
        ne: 0.0,
        sw: 0.0,
        se: 0.0,
    };

    pub const PADDING: Vec2 = Vec2 { x: 16.0, y: 6.0 };

    pub fn dark() -> Self {
        Self {
            rounding: Theme::ROUNDING,
            padding: Theme::PADDING,
            bg: Color32::from_hex("#121212").unwrap(),
            primary: Palette {
                main: Color32::from_hex("#90caf9").unwrap(),
                light: Color32::from_hex("#e3f2fd").unwrap(),
                dark: Color32::from_hex("#42a5f5").unwrap(),
                contrast_text: Color32::BLACK.gamma_multiply(0.9),
            },
            secondary: Palette {
                main: Color32::from_hex("#ce93d8").unwrap(),
                light: Color32::from_hex("#f3e5f5").unwrap(),
                dark: Color32::from_hex("#ab47bc").unwrap(),
                contrast_text: Color32::from_hex("#ffffff").unwrap(),
            },
            text_color: TextColor {
                primary: Color32::from_rgb(255, 255, 255).gamma_multiply(0.7),
                secondary: Color32::from_rgb(255, 255, 255).gamma_multiply(0.5),
                disabled: Color32::from_rgb(255, 255, 255).gamma_multiply(0.5),
                icon: Color32::from_rgb(255, 255, 255).gamma_multiply(0.12),
            },
            action: Action {
                active: Color32::from_hex("#ffffff").unwrap(),
                hover: Color32::from_rgb(255, 255, 255).gamma_multiply(0.08),
                selected: Color32::from_rgb(255, 255, 255).gamma_multiply(0.16),
                disabled: Color32::from_rgb(255, 255, 255).gamma_multiply(0.08),
                disabled_bg: Color32::from_rgb(255, 255, 255).gamma_multiply(0.12),
                focus: Color32::from_rgb(255, 255, 255).gamma_multiply(0.12),
            },
        }
    }
}

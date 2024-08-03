use serde::{Deserialize, Serialize};

/// The common version of the project
///
/// I dont bother to update all the cargo files, so this should be considerd the actual version!
pub const VERSION: &str = "0.0.1";

#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn from_hex(hex: &str) -> Option<Self> {
        if hex.len() != 6 || !hex.is_ascii() {
            return None;
        }

        let r = &hex[0..2];
        let g = &hex[2..4];
        let b = &hex[4..6];

        let r = u8::from_str_radix(r, 16).ok()?;
        let g = u8::from_str_radix(g, 16).ok()?;
        let b = u8::from_str_radix(b, 16).ok()?;

        Some(Color { r, g, b })
    }
    pub fn to_hex(&self) -> String {
        format!("{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

impl From<String> for Color {
    fn from(value: String) -> Self {
        Color::from_hex(&value).unwrap_or(Color { r: 0, g: 0, b: 0 })
    }
}

#[derive(Serialize, Deserialize, sqlx::Type, Debug, PartialEq, Eq, Clone, Copy)]
#[sqlx(type_name = "habit_kind", rename_all = "lowercase")]
pub enum HabitKind {
    Habit,
    Addiction,
}

#[derive(Serialize, Deserialize, sqlx::Type, Debug, PartialEq, Eq, Clone, Copy)]
#[sqlx(type_name = "recording_type", rename_all = "lowercase")]
pub enum RecordingType {
    Point,
    Span,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Habit {
    pub id: i32,
    pub name: String,
    pub color: Color,
    pub kind: HabitKind,
    pub recording_type: RecordingType,
    pub every: Option<i32>,
}

impl Habit {
    pub fn as_create(&self) -> api::CreateHabit {
        api::CreateHabit {
            name: self.name.clone(),
            color: self.color,
            kind: self.kind,
            recording_type: self.recording_type,
            every: self.every,
        }
    }
}

pub mod api {

    use super::*;

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
    pub struct CreateHabit {
        pub name: String,
        pub color: Color,
        pub kind: HabitKind,
        pub recording_type: RecordingType,
        pub every: Option<i32>,
    }

    impl Default for CreateHabit {
        fn default() -> Self {
            Self {
                name: String::from("New Habit"),
                color: Color { r: 0, g: 0, b: 255 },
                kind: HabitKind::Habit,
                recording_type: RecordingType::Point,
                every: None,
            }
        }
    }

    impl From<Habit> for CreateHabit {
        fn from(value: Habit) -> Self {
            Self {
                name: value.name,
                color: value.color,
                kind: value.kind,
                recording_type: value.recording_type,
                every: value.every,
            }
        }
    }

    impl CreateHabit {
        pub fn with_id(self, id: i32) -> Habit {
            Habit {
                id,
                name: self.name,
                color: self.color,
                kind: self.kind,
                recording_type: self.recording_type,
                every: self.every,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    #[test]
    fn color_blue_from() {
        assert_eq!(
            Color::from_hex("0000FF"),
            Some(Color { r: 0, g: 0, b: 255 })
        )
    }
    #[test]
    fn color_blue_to() {
        assert_eq!(Color { r: 0, g: 0, b: 255 }.to_hex(), "0000FF");
    }

    proptest! {
        #[test]
        fn color_doesnt_panic(s: String) {
            Color::from_hex(&s);
        }

        #[test]
        fn color_fuzzy(r: u8, g: u8, b: u8) {
            let color = Color { r, g, b };
            let hex = color.to_hex();
            assert_eq!(Color::from_hex(&hex), Some(color));
        }

        #[test]
        fn color_len(r: u8, g: u8, b: u8) {
            let color = Color { r, g, b };
            let hex = color.to_hex();
            assert_eq!(hex.len(), 6);
        }
    }
}

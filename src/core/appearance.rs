// SPDX-License-Identifier: GPL-3.0-or-later

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Theme {
    #[default]
    System,
    Light,
    Dark,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ItemSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl ItemSize {
    pub fn scale(self) -> f64 {
        match self {
            Self::Small => 0.8,
            Self::Medium => 1.0,
            Self::Large => 1.25,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IconSize {
    Small,
    #[default]
    Medium,
    Large,
    ExtraLarge,
}

impl IconSize {
    pub fn scale(self) -> f64 {
        match self {
            Self::Small => 0.75,
            Self::Medium => 1.0,
            Self::Large => 1.25,
            Self::ExtraLarge => 1.5,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortOrder {
    #[default]
    RecentlyUsed,
    AppName,
    GroupedByApp,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Appearance {
    pub theme: Theme,
    pub size: ItemSize,
    pub icon_size: IconSize,
    pub sort: SortOrder,
    pub orientation: Orientation,
    pub item_padding: f64,
    pub item_spacing: f64,
    pub font_family: String,
    pub font_size: f64,
    pub title_lines: u8,
}

impl Default for Appearance {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            size: ItemSize::default(),
            icon_size: IconSize::default(),
            sort: SortOrder::default(),
            orientation: Orientation::default(),
            item_padding: 8.0,
            item_spacing: 8.0,
            font_family: String::new(),
            font_size: 12.0,
            title_lines: 2,
        }
    }
}

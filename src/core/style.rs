// SPDX-License-Identifier: GPL-3.0-or-later

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Style {
    #[default]
    Icons,
    Previews,
    PreviewsWithIcon,
    List,
}

impl Style {
    /// Whether this style shows window previews.
    pub fn uses_previews(self) -> bool {
        matches!(self, Self::Previews | Self::PreviewsWithIcon)
    }
}

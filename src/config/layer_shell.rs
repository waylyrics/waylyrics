use gtk4_layer_shell::Edge::{self};
use gtk4_layer_shell::LayerShell;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, EnumString};

#[derive(Clone, Copy, Default, EnumIter, strum::Display, EnumString, Serialize, Deserialize)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum LayerShellAnchor {
    Center,

    Top,
    #[default]
    Bottom,
    Left,
    Right,

    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl LayerShellAnchor {
    pub(crate) fn setup(self, shell: &impl LayerShell) {
        for &edge in self.edges() {
            shell.set_anchor(edge, true);
        }
    }

    fn edges(self) -> &'static [Edge] {
        match self {
            LayerShellAnchor::Center => &[],
            LayerShellAnchor::Top => &[Edge::Top],
            LayerShellAnchor::Bottom => &[Edge::Bottom],
            LayerShellAnchor::Left => &[Edge::Left],
            LayerShellAnchor::Right => &[Edge::Right],
            LayerShellAnchor::TopLeft => &[Edge::Top, Edge::Left],
            LayerShellAnchor::TopRight => &[Edge::Top, Edge::Right],
            LayerShellAnchor::BottomLeft => &[Edge::Bottom, Edge::Left],
            LayerShellAnchor::BottomRight => &[Edge::Bottom, Edge::Right],
        }
    }
}

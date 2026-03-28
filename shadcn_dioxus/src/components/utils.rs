/// Concatenate CSS class names, filtering out empty strings.
pub fn cn(classes: &[&str]) -> String {
    classes
        .iter()
        .filter(|c| !c.is_empty())
        .copied()
        .collect::<Vec<&str>>()
        .join(" ")
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Side {
    Top,
    #[default]
    Right,
    Bottom,
    Left,
}

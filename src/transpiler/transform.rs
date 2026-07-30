// transformations that can be applied to a program before emission.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Transformation {
    RemoveTypes,
}

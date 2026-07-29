use crate::syntactic_analyzer::Symbol;

// renders a typed list in HDDL syntax, e.g. "?x - block ?y" (untyped symbols print bare)
pub(crate) fn format_typed_list(vars: &[Symbol]) -> String {
    vars.iter()
        .map(|var| match var.symbol_type {
            Some(t) => format!("{} - {}", var.name, t),
            None => var.name.to_string(),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

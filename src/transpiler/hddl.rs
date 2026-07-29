use std::fmt;

use crate::syntactic_analyzer::Symbol;

// space-joins any list of displayable items
pub(crate) fn format_list<T: fmt::Display>(items: &[T]) -> String {
    items.iter()
        .map(|item| item.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

// renders a typed list in HDDL syntax, e.g. "?x - block ?y" (untyped symbols print bare)
pub(crate) fn format_typed_list(vars: &[Symbol]) -> String {
    format_list(vars)
}

// renders a task/predicate invocation, e.g. "(deliver ?p ?l)" or "(noop)"
pub(crate) fn format_call(name: &str, args: &[Symbol]) -> String {
    if args.is_empty() {
        format!("({})", name)
    } else {
        format!("({} {})", name, format_typed_list(args))
    }
}

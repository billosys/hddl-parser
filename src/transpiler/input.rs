// the source formats a program can be built from
pub enum Input<'a> {
    Hddl {
        domain: &'a [u8],
        problem: Option<&'a [u8]>,
    },
    Json(&'a str),
}

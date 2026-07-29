// the source formats a program can be built from
pub enum Input<'a> {
    Hddl {
        domain: &'a Vec<u8>,
        problem: Option<&'a Vec<u8>>,
    },
    Json(&'a str),
}

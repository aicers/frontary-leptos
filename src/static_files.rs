use std::include_bytes;

#[allow(clippy::too_many_lines)]
#[must_use]
pub fn static_files() -> Vec<(&'static str, &'static [u8])> {
    let mut rtn: Vec<(&'static str, &'static [u8])> = Vec::new();

    let file = include_bytes!("../static/tailwind.frontary.theme.json");
    rtn.push(("tailwind.frontary.theme.json", file));

    let file = include_bytes!("../static/tailwind.frontary.safelist.json");
    rtn.push(("tailwind.frontary.safelist.json", file));

    let file = include_bytes!("../static/input.frontary.css");
    rtn.push(("input.frontary.css", file));

    rtn
}

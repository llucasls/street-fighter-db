#[derive(Debug)]
pub struct Fighter {
    pub name: String,
    pub style: String,
    pub nationality: String,
}

impl Fighter {
    pub fn new(name: &str, style: &str, nationality: &str) -> Fighter {
        return Fighter {
            name: name.to_string(),
            style: style.to_string(),
            nationality: nationality.to_string(),
        };
    }
}

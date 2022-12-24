#[derive(Debug)]
struct Fighter {
    name: String,
    style: String,
    nationality: String,
}

impl Fighter {
    fn new(name: &str, style: &str, nationality: &str) -> Fighter {
        return Fighter {
            name: name.to_string(),
            style: style.to_string(),
            nationality: nationality.to_string(),
        };
    }
}

fn main() {
    let ryu = Fighter::new("Ryu", "Karate", "Japanese");

    println!("The chosen fighter is: {:?}", ryu);
}

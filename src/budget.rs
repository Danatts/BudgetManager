#[derive(Debug)]
struct Budget {
    pub name: String,
    pub amount: f64,
}

impl Budget {
    pub fn new(name: &str, amount: f64) -> Budget {
        let name = name.to_string();
        Budget { name, amount }
    }
}

#[cfg(test)]
mod tests {}

use cli::run;

pub mod budget;
pub mod cli;
pub mod database;

fn main() {
    run();
}

#[cfg(test)]
mod tests {

    #[test]
    fn main_ok() {}
}

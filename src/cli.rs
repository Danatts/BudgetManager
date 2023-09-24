use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Name of entity
    #[arg(short, long)]
    pub entity: String,

    /// Name of category
    #[arg(short, long)]
    pub category: String,

    /// Value
    #[arg(short, long, default_value_t = 0.0)]
    pub value: f64,
}

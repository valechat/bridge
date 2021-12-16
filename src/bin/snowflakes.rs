use std::time::Duration;
use vale_bridge::SnowflakeGenerator;

fn main() {
    let gen = SnowflakeGenerator::new(0, 0);

    let snowflake = gen.generate();
    println!("{:?}", snowflake);
}
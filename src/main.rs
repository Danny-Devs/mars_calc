// Physical constants
const EARTH_GRAVITY: f32 = 9.81; // Earth's gravity in m/s²
const MARS_GRAVITY: f32 = 3.711; // Mars' gravity in m/s²
const KG_TO_LBS: f32 = 2.20462; // Conversion factor from kilograms to pounds

use std::io::{self, Write};

fn main() {
    println!("🚀 Mars Weight Calculator");

    loop {
        println!("Enter a weight in lbs, or 'q' to quit");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input.eq_ignore_ascii_case("q") {
            println!("\nGoodbye! 👋");
            break;
        }

        match input.parse::<f32>() {
            Ok(weight) => {
                let mars_weight = calculate_weight_on_mars(weight);
                println!(
                    "🪐 {:.1} lbs. on Earth weighs {:.1} lbs. on Mars\n",
                    weight, mars_weight
                );
            }
            Err(_) => println!("❌ Please enter a valid number or 'q' to quit\n"),
        }
    }
}

/// Calculates equivalent weight on Mars given Earth weight in pounds
///
/// Process:
/// 1. Convert Earth pounds to mass (divide by Earth gravity)
/// 2. Calculate Mars weight (multiply by Mars gravity)
/// 3. Convert back to pounds
fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / EARTH_GRAVITY) * MARS_GRAVITY * KG_TO_LBS
}

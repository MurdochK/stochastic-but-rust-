use rand::Rng;
use sha2::{Sha256, Digest};
use std::fmt::Write;

/// Hash a string using SHA-256
fn hash_identifier(identifier: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(identifier);
    let hash = hasher.finalize();
    let mut hex_string = String::new();
    for byte in hash {
        write!(&mut hex_string, "{:02x}", byte).unwrap();
    }
    hex_string
}

/// Simulate attendance based on probabilities
fn simulate_attendance(probabilities: &[f64], days: usize) -> Vec<Vec<bool>> {
    let mut rng = rand::thread_rng();
    probabilities
        .iter()
        .map(|&prob| {
            (0..days)
                .map(|_| rng.gen_bool(prob)) // Random attendance based on probability
                .collect()
        })
        .collect()
}

fn main() {
    // Constants
    const TOTAL_INDIVIDUALS: usize = 25;
    const DAYS: usize = 7; // Predict attendance for a week

    // Generate unique identifiers for individuals and hash them
    let identifiers: Vec<String> = (1..=TOTAL_INDIVIDUALS)
        .map(|i| format!("Student_{}", i))
        .map(|id| hash_identifier(&id))
        .collect();

    // Assign random attendance probabilities (between 50% and 100%)
    let mut rng = rand::thread_rng();
    let attendance_probabilities: Vec<f64> = (0..TOTAL_INDIVIDUALS)
        .map(|_| rng.gen_range(0.5..1.0)) // Probability between 50% and 100%
        .collect();

    // Simulate attendance over a week
    let attendance = simulate_attendance(&attendance_probabilities, DAYS);

    // Display the results
    println!("Predicted Attendance for {} Days:\n", DAYS);
    for (id, daily_attendance) in identifiers.iter().zip(attendance.iter()) {
        println!("Hashed ID: {}", &id);
        for (day, attended) in daily_attendance.iter().enumerate() {
            println!("  Day {}: {}", day + 1, if *attended { "Present" } else { "Absent" });
        }
        println!();
    }
}

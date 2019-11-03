fn main() {
    let fahrenheit = 70.00; 
    println!("Fahrenheit is {}. Converts to Celsius {}", fahrenheit, fahrenheit_to_celsius(fahrenheit));
}


fn fahrenheit_to_celsius(celsius: f64) -> f64 { 
    // T(°C) = (T(°F) - 32) × 5/9
    (celsius - 32.00) * (5.00 / 9.00)
}
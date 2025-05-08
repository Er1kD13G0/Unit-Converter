pub fn celsius_para_fahnrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

pub fn celsius_para_kelvin(celsius: f64) -> f64 {
    celsius + 273.15
}

pub fn fahnrenheit_para_celsius(fahnrenheit: f64) -> f64 {
    (fahnrenheit - 32.0) * 5.0 / 9.0
}

pub fn fahnrenheit_para_kelvin(fahnrenheit: f64) -> f64 {
    (fahnrenheit - 32.0) * 5.0 / 9.0 + 273.15
}

pub fn kelvin_para_celsius(kelvin: f64) -> f64 {
    kelvin - 273.15
}

pub fn kelvin_para_fahnrenheit(kelvin: f64) -> f64 {
    (kelvin - 273.15) * 9.0 / 5.0 + 32.0
}

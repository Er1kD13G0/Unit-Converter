mod distancia;
mod peso;
mod temperaturas;

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    value: f64,

    #[arg(short, long)]
    from: String,

    #[arg(short, long)]
    to: String,
}

fn main() {
    let args = Args::parse();

    if let Ok(result) = try_convert_temperature(&args) {
        println!("{} {} = {} {}", args.value, args.from, result, args.to);
        return;
    }

    if let Ok(result) = try_convert_distancia(&args) {
        println!("{} {} = {} {}", args.value, args.from, result, args.to);
        return;
    }

    if let Ok(result) = try_convert_peso(&args) {
        println!("{} {} = {} {}", args.value, args.from, result, args.to);
        return;
    }

    eprintln!("Erro de conversão");
    std::process::exit(1);

    fn try_convert_temperature(args: &Args) -> Result<f64, ()> {
        match (
            args.from.to_lowercase().as_str(),
            args.to.to_lowercase().as_str(),
        ) {
            //Conversao de temperaturas
            ("celsius", "fahrenheit") => Ok(temperaturas::celsius_para_fahnrenheit(args.value)),
            ("celsius", "kelvin") => Ok(temperaturas::celsius_para_kelvin(args.value)),
            ("fahrenheit", "celsius") => Ok(temperaturas::fahnrenheit_para_celsius(args.value)),
            ("fahrenheit", "kelvin") => Ok(temperaturas::fahnrenheit_para_kelvin(args.value)),
            ("kelvin", "celsius") => Ok(temperaturas::kelvin_para_celsius(args.value)),
            ("kelvin", "fahrenheit") => Ok(temperaturas::kelvin_para_fahnrenheit(args.value)),
            _ => Err(()),
        }
    }

    fn try_convert_distancia(args: &Args) -> Result<f64, ()> {
        match (
            args.from.to_lowercase().as_str(),
            args.to.to_lowercase().as_str(),
        ) {
            //Conversao de distancia
            ("milimetro", "centimetro") => Ok(distancia::milimetro_para_centimetro(args.value)),
            ("milimetro", "metro") => Ok(distancia::milimetro_para_metro(args.value)),
            ("milimetro", "quilometro") => Ok(distancia::milimetro_para_quilometro(args.value)),
            ("centimetro", "milimetro") => Ok(distancia::centimetro_para_milimetro(args.value)),
            ("centimetro", "metro") => Ok(distancia::centimetro_para_metro(args.value)),
            ("centimetro", "quilometro") => Ok(distancia::centimetro_para_quilometro(args.value)),
            ("metro", "milimetro") => Ok(distancia::metro_para_milimetro(args.value)),
            ("metro", "centimetro") => Ok(distancia::metro_para_centimetro(args.value)),
            ("metro", "quilometro") => Ok(distancia::metro_para_quilometro(args.value)),
            ("metro", "polegada") => Ok(distancia::metro_para_polegada(args.value)),
            ("metro", "pes") => Ok(distancia::metro_para_pe(args.value)),
            ("metro", "jarda") => Ok(distancia::metro_para_jarda(args.value)),
            ("quilometro", "metro") => Ok(distancia::quilometro_para_metro(args.value)),
            ("quilometro", "milha") => Ok(distancia::quilometro_para_milha(args.value)),
            ("polegada", "metro") => Ok(distancia::polegada_para_metro(args.value)),
            ("pes", "metro") => Ok(distancia::pe_para_metro(args.value)),
            ("jarda", "metro") => Ok(distancia::jarda_para_metro(args.value)),
            ("milha", "quilometro") => Ok(distancia::milha_para_quilometro(args.value)),

            _ => Err(()),
        }
    }

    fn try_convert_peso(args: &Args) -> Result<f64, ()> {
        match (
            args.from.to_lowercase().as_str(),
            args.to.to_lowercase().as_str(),
        ) {
            //Conversao de peso
            ("miligrama", "grama") => Ok(peso::miligrama_para_grama(args.value)),
            ("grama", "miligrama") => Ok(peso::grama_para_miligrama(args.value)),
            ("grama", "kilograma") => Ok(peso::grama_para_kilograma(args.value)),
            ("grama", "onca") => Ok(peso::grama_para_onca(args.value)),
            ("kilograma", "grama") => Ok(peso::kilograma_para_grama(args.value)),
            ("kilograma", "libra") => Ok(peso::kilograma_para_libra(args.value)),
            ("libra", "kilograma") => Ok(peso::libra_para_kilograma(args.value)),
            ("onca", "grama") => Ok(peso::onca_para_grama(args.value)),

            _ => Err(()),
        }
    }
}

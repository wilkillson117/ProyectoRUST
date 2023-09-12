extern crate serialport;

use std::io::{BufRead, BufReader};
use std::time::Duration;

pub fn leer_temperatura_y_humedad() -> Result<(f32, f32), Box<dyn std::error::Error>> {
    // Configurar el puerto serial en "COM3" con una velocidad de transmisión de 9600
    let puerto = serialport::new("COM3", 9600)
        .timeout(Duration::from_millis(1000))
        .open()?;

    // Crear un lector con búfer para leer datos del puerto serial de manera eficiente
    let mut lector = BufReader::new(puerto);
    let mut linea = String::new();

    // Leer una línea completa desde el puerto serial (hasta encontrar un carácter de nueva línea)
    lector.read_line(&mut linea)?;

    // Dividir la línea usando la coma como delimitador
    let valores: Vec<&str> = linea.trim().split(',').collect();

    // Analizar los valores de temperatura y humedad como números de coma flotante
    let temperatura: f32 = valores[0].parse()?;
    let humedad: f32 = valores[1].parse()?;

    // Devolver la temperatura y la humedad como un resultado exitoso
    Ok((temperatura, humedad))
}

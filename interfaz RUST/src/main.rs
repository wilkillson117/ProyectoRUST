use eframe::egui;
use egui::TextStyle;
use eframe::egui::plot::{Legend, Line, Plot, PlotPoints};
use eframe::egui::Color32;
use chrono::Local;
use egui_extras::{Size, StripBuilder};
use std::collections::VecDeque;

mod measurements;

struct MyApp {
    mediciones: VecDeque<(f64, f64, String)>, // Cola para contener las últimas 20 mediciones con hora
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        configurar_estilos_texto(&cc.egui_ctx);
        Self {
            mediciones: VecDeque::with_capacity(20),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let resultado = measurements::leer_temperatura_y_humedad();
        if let Ok((temp, hum)) = resultado {
            if self.mediciones.len() >= 20 {
                self.mediciones.pop_front(); // Eliminar el valor más antiguo
            }
            self.mediciones.push_back((temp as f64, hum as f64, Local::now().format("%H:%M:%S").to_string())); // Añadir el valor más nuevo
        }

        egui::CentralPanel::default().show(ctx, |ui| contenido(ui, &self.mediciones));
        ctx.request_repaint();
    }
}

fn configurar_estilos_texto(ctx: &egui::Context) {
    use egui::FontFamily::Proportional;
    let mut estilo = (*ctx.style()).clone();
    estilo.text_styles.insert(TextStyle::Heading, egui::FontId::new(20.0, Proportional));
    ctx.set_style(estilo);
}

fn contenido(ui: &mut egui::Ui, mediciones: &VecDeque<(f64, f64, String)>) {
    let colores = [
        Color32::from_rgb(39, 43, 52),
        Color32::from_rgb(25, 30, 45),
        Color32::from_rgb(30, 35, 55),
        Color32::from_rgb(35, 40, 65),
        Color32::from_rgb(40, 45, 75),
        Color32::from_rgb(45, 50, 85),
    ];

    // Código para construir la interfaz gráfica
    StripBuilder::new(ui)
        .size(Size::relative(0.15))
        .size(Size::relative(0.15))
        .size(Size::relative(0.35))
        .size(Size::relative(0.35))
        .vertical(|mut strip| {
            strip.cell(|ui| {
                let rect = ui.available_rect_before_wrap();
                ui.painter().rect_filled(rect, 0.0, colores[0]);
                // Mostrar la hora local
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(Local::now().format("%A, %d of %B %Y, %H:%M:%S").to_string());
                    });
                });
                ui.heading("C.A.S.A. - Control Atmosférico y Sistema Ambiental");
                ui.separator();
                ui.label("Recibe datos de un sensor DHT11 de temperatura y humedad mediante serial desde un ESP32");
            });
            // Celda 2 con 3 divisiones
            strip.cell(|ui| {
                StripBuilder::new(ui)
                    .size(Size::relative(0.3))
                    .size(Size::relative(0.3))
                    .size(Size::relative(0.4))
                    .horizontal(|mut celda2| {
                        celda2.cell(|ui| {
                            let rect = ui.available_rect_before_wrap();
                            ui.painter().rect_filled(rect, 0.0, colores[1]);
                            ui.label("Temperatura.");
                            if let Some((temperatura, _, _)) = mediciones.back() {
                                ui.label(format!("Última Temperatura: {:.2}°C", temperatura));
                            }
                        });
                        celda2.cell(|ui| {
                            let rect = ui.available_rect_before_wrap();
                            ui.painter().rect_filled(rect, 0.0, colores[2]);
                            ui.label("Humedad");
                            if let Some((_, humedad, _)) = mediciones.back() {
                                ui.label(format!("Última Humedad: {:.2}°C", humedad));
                            }
                        });
                        celda2.cell(|ui| {
                            let rect = ui.available_rect_before_wrap();
                            ui.painter().rect_filled(rect, 0.0, colores[3]);
                            ui.label("Sensación Térmica");
                            if let Some((temperatura, humedad, _)) = mediciones.back() {
                                // Cálculo de ejemplo para la temperatura aparente
                                let sensacion_termica = temperatura + (0.33 * humedad) - 0.70; // Ajustar esta fórmula según sea necesario
                                // Mostrar la sensación térmica
                                ui.label(format!("Sensación Térmica: {:.2}°C", sensacion_termica));
                            }   
                        });
                    });
            });

            let temperatura_plot = Plot::new("Temperatura vs Tiempo")
                .legend(Legend::default());

            let humedad_plot = Plot::new("Humedad vs Tiempo")
                .legend(Legend::default());

            // Código para construir gráficos de temperatura y humedad
            let grafico_temperatura: Vec<[f64; 2]> = mediciones
                .iter()
                .enumerate()
                .map(|(i, (temperatura, _, _))| [i as f64, *temperatura])
                .collect();

            let grafico_humedad: Vec<[f64; 2]> = mediciones
                .iter()
                .enumerate()
                .map(|(i, (_, humedad, _))| [i as f64, *humedad])
                .collect();

            strip.cell(|ui| {
                StripBuilder::new(ui)
                    .size(Size::relative(0.80))
                    .size(Size::relative(0.20))
                    .horizontal(|mut celda3| {
                        celda3.cell(|ui| {
                            let rect = ui.available_rect_before_wrap();
                            ui.painter().rect_filled(rect, 0.0, colores[1]);
                            ui.label("Gráfico.");

                            temperatura_plot.show(ui, |plot_ui| {
                                plot_ui.line(Line::new(PlotPoints::from(grafico_temperatura)).name("Temperatura"));
                            });
                            
                        });
                        celda3.cell(|ui| {
                            egui::ScrollArea::horizontal().show(ui, |ui| {
                                use egui_extras::{Column, TableBuilder};
                        
                                let tabla_temperatura = TableBuilder::new(ui)
                                    .striped(true)
                                    .resizable(true)
                                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                                    .column(Column::auto())
                                    .column(Column::initial(100.0).range(40.0..=300.0))
                                    .column(Column::remainder())
                                    .min_scrolled_height(0.0);
                        
                                tabla_temperatura
                                    .header(20.0, |mut header| {
                                        header.col(|ui| { ui.strong("Hora"); });
                                        header.col(|ui| { ui.strong("Temperatura"); });
                                    })
                                    .body(|mut body| {
                                        for (_, (temperatura, _, hora)) in mediciones.iter().enumerate() {
                                            body.row(18.0, |mut row| {
                                                row.col(|ui| { ui.label(hora); });
                                                row.col(|ui| { ui.label(format!("{:.2}°C", temperatura)); });
                                            });
                                        }
                                        
                                    });
                            });
                        });
                        
                        
                    });
            });

            // Más código para construir la interfaz gráfica
            strip.cell(|ui| {
                StripBuilder::new(ui)
                    .size(Size::relative(0.80))
                    .size(Size::relative(0.20))
                    .horizontal(|mut celda4| {
                        celda4.cell(|ui| {
                            let rect = ui.available_rect_before_wrap();
                            ui.painter().rect_filled(rect, 0.0, colores[1]);
                            ui.label("Gráfico.");

                            humedad_plot.show(ui, |plot_ui| {
                                plot_ui.line(Line::new(PlotPoints::from(grafico_humedad)).name("Humedad"));
                            });
                        });
                        celda4.cell(|ui| {
                            egui::ScrollArea::horizontal().show(ui, |ui| {
                                use egui_extras::{Column, TableBuilder};
                        
                                let tabla_humedad = TableBuilder::new(ui)
                                    .striped(true)
                                    .resizable(true)
                                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                                    .column(Column::auto())
                                    .column(Column::initial(100.0).range(40.0..=300.0))
                                    .column(Column::remainder())
                                    .min_scrolled_height(0.0);
                        
                                tabla_humedad
                                    .header(20.0, |mut header| {
                                        header.col(|ui| { ui.strong("Hora"); });
                                        header.col(|ui| { ui.strong("Humedad"); });
                                    })
                                    .body(|mut body| {
                                        for (_, (_, humedad, hora)) in mediciones.iter().enumerate() {
                                            body.row(18.0, |mut row| {
                                                row.col(|ui| { ui.label(hora); });
                                                row.col(|ui| { ui.label(format!("{:.2}%", humedad)); });
                                            });
                                        }
                                    });
                            });
                        });
                    });
            });
        });
}

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    let opciones = eframe::NativeOptions::default();
    eframe::run_native("Sistema de Monitoreo de la Casa", opciones, Box::new(|cc| Box::new(MyApp::new(cc))))
}

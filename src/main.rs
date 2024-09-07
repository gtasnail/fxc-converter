#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use rfd::FileDialog;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::ops::Not;
use std::time::Instant;
use eframe::egui::{FontDefinitions, FontFamily, Context, RichText, Color32};
use eframe::Frame;

fn load_custom_font(ctx: &Context) {
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "PixelFont".to_owned(),
        egui::FontData::from_static(include_bytes!("pixel.ttf")),
    );
    fonts.families.entry(FontFamily::Proportional).or_default().insert(0, "PixelFont".to_owned());
    ctx.set_fonts(fonts);
}


fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        ..Default::default() 
    };

    eframe::run_native(
        "FXC Converter",
        options,
        Box::new(|_cc| Ok(Box::new(FxcDecompiler::default()))),
    )
}

#[derive(Default)]
struct FxcDecompiler {
    fxc_file: String,
    output_folder: String,
    patch_file: String,
    offset: String,
    result: String,
    pack_output_folder: String,
    log_buffer: String,
    dark_mode: bool,  
}

impl eframe::App for FxcDecompiler {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {

        load_custom_font(ctx);

        if self.dark_mode.not() {
            let mut visuals = egui::Visuals::dark(); 
            visuals.window_rounding = 10.0.into();
            visuals.override_text_color = Some(Color32::WHITE); 
            ctx.set_visuals(visuals);
        } else {
            let mut visuals = egui::Visuals::light(); 
            visuals.window_rounding = 10.0.into();
            visuals.override_text_color = Some(Color32::from_rgb(50, 50, 50));
            ctx.set_visuals(visuals);
        }
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading(egui::RichText::new("FXC Converter").heading().strong().size(25.0));
                ui.label(RichText::new("Tool to unpack .fxc files").code());
                ui.hyperlink_to("GitHub", "https://www.github.com/gtasnail/fxc-converter/");
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.add(egui::Button::new(if self.dark_mode.not() { "🌙 Dark" } else { "🌞 Light" })).clicked() {
                        self.dark_mode = !self.dark_mode; 
                    }
                });
            });
        });


        egui::CentralPanel::default().show(ctx, |ui| { 
            egui::CollapsingHeader::new("Unpack FXC File")
                .default_open(true)
                .show(ui, |ui| {
                    ui.add_space(10.0);
                    egui::Grid::new("unpack_grid")
                        .spacing([10.0, 10.0]) 
                        .show(ui, |ui| {
                            ui.label("FXC file:");
                            if ui.add(egui::Button::new("📁 Select FXC File")).clicked() {
                                if let Some(file) = FileDialog::new().pick_file() {
                                    self.fxc_file = file.display().to_string();
                                }
                            }
                            ui.label(&self.fxc_file);
                            ui.end_row();

                            ui.label("Output folder:");
                            if ui.add(egui::Button::new("📂 Select Output Folder")).clicked() {
                                if let Some(folder) = FileDialog::new().pick_folder() {
                                    self.output_folder = folder.display().to_string();
                                }
                            }
                            ui.label(&self.output_folder);
                            ui.end_row();
                        });

                    ui.add_space(20.0);
                    if ui.add(egui::Button::new("🚀 Unpack").fill(Color32::from_rgb(100, 200, 255)).frame(true)).clicked() {
                        self.result = if !self.fxc_file.is_empty() && !self.output_folder.is_empty() {
                            match unpack(&self.fxc_file, &self.output_folder, &mut self.log_buffer) {
                                Ok(_) => "Unpacking completed 🎉".to_string(),
                                Err(e) => format!("Error: {:?}", e),
                            }
                        } else {
                            "Please select both an FXC file and an output folder.".to_string()
                        };
                    }
                });
            ui.add_space(15.0);
            ui.separator();
            ui.add_space(15.0);
            egui::CollapsingHeader::new("Pack FXC File")
                .default_open(true)
                .show(ui, |ui| {
                    ui.add_space(10.0);
                    egui::Grid::new("pack_grid")
                        .spacing([10.0, 10.0])
                        .show(ui, |ui| {
                            ui.label("FXC file:");
                            if ui.add(egui::Button::new("📁 Select FXC File")).clicked() {
                                if let Some(file) = FileDialog::new().pick_file() {
                                    self.fxc_file = file.display().to_string();
                                }
                            }
                            ui.label(&self.fxc_file);
                            ui.end_row();

                            ui.label("Patch file (.cso):");
                            if ui.add(egui::Button::new("📁 Select Patch File")).clicked() {
                                if let Some(file) = FileDialog::new().pick_file() {
                                    self.patch_file = file.display().to_string();
                                }
                            }
                            ui.label(&self.patch_file);
                            ui.end_row();

                            ui.label("Pack output folder:");
                            if ui.add(egui::Button::new("📂 Select Output Folder")).clicked() {
                                if let Some(folder) = FileDialog::new().pick_folder() {
                                    self.pack_output_folder = folder.display().to_string();
                                }
                            }
                            ui.label(&self.pack_output_folder);
                            ui.end_row();

                            ui.label("Offset:");
                            ui.text_edit_singleline(&mut self.offset);
                            ui.end_row();
                        });

                    ui.add_space(20.0);
                    if ui.add(egui::Button::new("📦 Pack").fill(Color32::from_rgb(255, 180, 100))).clicked() {
                        self.result = if !self.fxc_file.is_empty()
                            && !self.patch_file.is_empty()
                            && !self.offset.is_empty()
                            && !self.pack_output_folder.is_empty()
                        {
                            let offset = self.offset.parse().unwrap_or(0);
                            match pack(
                                &self.fxc_file,
                                offset,
                                &self.patch_file,
                                &self.pack_output_folder,
                                &mut self.log_buffer,
                            ) {
                                Ok(_) => "Packing completed 🎉".to_string(),
                                Err(e) => format!("Error: {:?}", e),
                            }
                        } else {
                            "Please provide FXC file, patch file, offset, and output folder.".to_string()
                        };
                    }
                });

            ui.add_space(20.0);
            ui.label(egui::RichText::new("Result:").strong());
            ui.label(&self.result); // revert (probably meh)
            ui.separator();

            ui.label(egui::RichText::new("Log:").strong());
            egui::ScrollArea::vertical().max_height(150.0).show(ui, |ui| {
                ui.label(&self.log_buffer);
            });
        });
    }
}



fn unpack(fxc_file: &str, output_folder: &str, log_buffer: &mut String) -> std::io::Result<()> {
    let convert_dir = Path::new(fxc_file);
    let converted_dir = Path::new(output_folder);
    fs::create_dir_all(converted_dir)?;

    let start_time = Instant::now();
    let mut shader_count = 0;
    let file_content = fs::read(&convert_dir)?;

    let output_path = converted_dir.join(convert_dir.file_name().unwrap());
    let output_path = output_path.with_extension("o");

    for (i, window) in file_content.windows(4).enumerate() {
        if window == b"DXBC" {
            let shader_size =
                u32::from_le_bytes(file_content[i + 24..i + 28].try_into().unwrap()) as usize;
            log_buffer.push_str(&format!("Found shader in: {:?}\n", convert_dir));
            log_buffer.push_str(&format!("Shader is: {} bytes long\n", shader_size));

            let shader_data = &file_content[i..i + shader_size];
            let mut output_file = File::create(output_path.with_extension(format!("o{}", i)))?;
            output_file.write_all(shader_data)?;
            shader_count += 1;
        }
    }

    let duration = start_time.elapsed();
    log_buffer.push_str(&format!(
        "Found {} shaders in this file. This operation took {:?}\n",
        shader_count, duration
    ));
    Ok(())
}

fn pack(
    fxc_file: &str,
    offset: u64,
    patch_file: &str,
    output_folder: &str,
    log_buffer: &mut String,
) -> std::io::Result<()> {
    let convert_dir = Path::new(fxc_file);
    let patch_content = fs::read(patch_file)?;
    let file_content = fs::read(convert_dir)?;

    let output_dir = Path::new(output_folder);
    fs::create_dir_all(output_dir)?;

    let output_filename = convert_dir.file_name().unwrap().to_str().unwrap();
    let output_path = output_dir.join(format!("new_{}", output_filename));

    let start = offset as usize;

    if &file_content[start..start + 4] == b"DXBC" {
        let shader_size =
            u32::from_le_bytes(file_content[start + 24..start + 28].try_into().unwrap()) as usize;
        log_buffer.push_str(&format!(
            "Found shader at offset: {} (Expected: {})\n",
            start, offset
        ));
        log_buffer.push_str(&format!("Shader was: {} bytes long\n", shader_size));

        let mut new_content = Vec::new();
        new_content.extend_from_slice(&file_content[..start - 4]);
        new_content.extend_from_slice(&patch_content[24..28]);
        new_content.extend_from_slice(&patch_content);
        new_content.extend_from_slice(&file_content[start + shader_size..]);

        let mut output_file = File::create(&output_path)?;
        output_file.write_all(&new_content)?;
        log_buffer.push_str(&format!(
            "Packing completed. New file created at: {:?}\n",
            output_path
        ));
    } else {
        log_buffer.push_str("Could not find valid DXBC at or near the offset.\n");
    }
    Ok(())
}

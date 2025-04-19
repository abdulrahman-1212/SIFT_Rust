use eframe::egui;
use image::{DynamicImage, GrayImage, Rgb, RgbImage, GenericImage};
use imageproc::drawing::{draw_cross_mut, draw_line_segment_mut};

pub fn show_matches(
    image1: &GrayImage,
    image2: &GrayImage,
    keypoints1: &[KeyPoint],
    keypoints2: &[KeyPoint],
    matches: &[(usize, usize)],
) -> Result<(), eframe::Error> {
    // Prepare images with keypoints
    let mut img1 = DynamicImage::ImageLuma8(image1.clone()).to_rgb8();
    let mut img2 = DynamicImage::ImageLuma8(image2.clone()).to_rgb8();
    
    // Draw keypoints (red crosses)
    let red = Rgb([255, 0, 0]);
    for kp in keypoints1 {
        draw_cross_mut(&mut img1, red, kp.x as i32, kp.y as i32);
    }
    for kp in keypoints2 {
        draw_cross_mut(&mut img2, red, kp.x as i32, kp.y as i32);
    }

    // Create composite image with matches
    let (w1, h1) = img1.dimensions();
    let (w2, _) = img2.dimensions();
    let mut composite = RgbImage::new(w1 + w2, h1);
    composite.copy_from(&img1, 0, 0)?;
    composite.copy_from(&img2, w1, 0)?;
    
    // Draw matches (green lines)
    let green = Rgb([0, 255, 0]);
    for &(i, j) in matches {
        let kp1 = &keypoints1[i];
        let kp2 = &keypoints2[j];
        draw_line_segment_mut(
            &mut composite,
            (kp1.x as f32, kp1.y as f32),
            (w1 as f32 + kp2.x as f32, kp2.y as f32),
            green,
        );
    }

    // Convert to egui-compatible format
    let composite_rgba = DynamicImage::ImageRgb8(composite).to_rgba8();
    let size = [composite_rgba.width() as usize, composite_rgba.height() as usize];
    let pixels = composite_rgba.into_raw();
    let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);

    // Launch native window
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };
    
    eframe::run_native(
        "SIFT Feature Matches", 
        options, 
        Box::new(|_cc| Box::new(ImageApp::new(color_image)))
    )
}

struct ImageApp {
    texture: Option<egui::TextureHandle>,
    image: egui::ColorImage,
}

impl ImageApp {
    fn new(image: egui::ColorImage) -> Self {
        Self { 
            texture: None,
            image 
        }
    }
}

impl eframe::App for ImageApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let texture = self.texture.get_or_insert_with(|| {
                ui.ctx().load_texture(
                    "matches",
                    self.image.clone(),
                    egui::TextureOptions::default()
                )
            });
            
            ui.image(texture, texture.size_vec2());
        });
    }
}
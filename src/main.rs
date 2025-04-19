use show_image::{create_window, event, ImageView, WindowOptions};
use image::{GrayImage, Rgb, RgbImage};
use imageproc::drawing::{draw_cross_mut, draw_line_segment_mut};

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load images
    let img1 = image::open("image1.jpg")?.to_luma8();
    let img2 = image::open("image2.jpg")?.to_luma8();

    // Mock keypoints (replace with actual SIFT implementation)
    let kps1 = vec![(100.0, 100.0), (150.0, 120.0)];
    let kps2 = vec![(110.0, 105.0), (160.0, 125.0)];
    let matches = vec![(0, 0), (1, 1)];

    // Create visualization
    let composite = create_composite(&img1, &img2, &kps1, &kps2, &matches);
    
    // Convert to show-image format
    let image_view = ImageView::new(
        show_image::ImageInfo::rgb8(composite.width(), composite.height()),
        composite.into_raw(),
    );

    // Create window and display
    let window = create_window("SIFT Matches", WindowOptions::default())?;
    window.set_image("matches", image_view)?;

    // Event loop
    for event in window.event_channel()? {
        if let event::WindowEvent::KeyboardInput(event) = event {
            if event.input.key_code == Some(event::VirtualKeyCode::Escape) {
                break;
            }
        }
    }

    Ok(())
}

fn create_composite(
    img1: &GrayImage,
    img2: &GrayImage,
    kps1: &[(f32, f32)],
    kps2: &[(f32, f32)],
    matches: &[(usize, usize)],
) -> RgbImage {
    // Convert to RGB and mark keypoints
    let mut img1_rgb = GrayImage::to_rgb(img1);
    let mut img2_rgb = GrayImage::to_rgb(img2);
    
    let red = Rgb([255, 0, 0]);
    for &(x, y) in kps1 {
        draw_cross_mut(&mut img1_rgb, red, x as i32, y as i32);
    }
    for &(x, y) in kps2 {
        draw_cross_mut(&mut img2_rgb, red, x as i32, y as i32);
    }

    // Create composite image
    let (w1, h1) = img1_rgb.dimensions();
    let (w2, h2) = img2_rgb.dimensions();
    let mut composite = RgbImage::new(w1 + w2, h1.max(h2));
    
    composite.copy_from(&img1_rgb, 0, 0).unwrap();
    composite.copy_from(&img2_rgb, w1, 0).unwrap();

    // Draw matches
    let green = Rgb([0, 255, 0]);
    for &(i, j) in matches {
        let (x1, y1) = kps1[i];
        let (x2, y2) = kps2[j];
        draw_line_segment_mut(
            &mut composite,
            (x1 as f32, y1 as f32),
            (w1 as f32 + x2 as f32, y2 as f32),
            green,
        );
    }

    composite
}
use image::{DynamicImage, GrayImage, Rgb, RgbImage, GenericImage};
use imageproc::drawing::{draw_cross_mut, draw_line_segment_mut};
use show_image::{Image, ImageView, WindowOptions};

// Make sure KeyPoint is defined somewhere in your code
#[derive(Debug, Clone)]
pub struct KeyPoint {
    pub x: f32,
    pub y: f32,
    pub octave: usize,
    pub scale: usize,
    pub orientation: f32,
    pub descriptor: [f32; 128],
}

pub fn visualize_sift_matches(
    image1: &GrayImage,
    image2: &GrayImage,
    keypoints1: &[KeyPoint],
    keypoints2: &[KeyPoint],
    matches: &[(usize, usize)],
) -> Result<(), Box<dyn std::error::Error>> {
    // Convert images to color
    let mut img1 = DynamicImage::ImageLuma8(image1.clone()).to_rgb8();
    let mut img2 = DynamicImage::ImageLuma8(image2.clone()).to_rgb8();

    // Create composite image
    let (w1, h1) = img1.dimensions();
    let (w2, h2) = img2.dimensions();
    let mut composite = RgbImage::new(w1 + w2, h1.max(h2));
    composite.copy_from(&img1, 0, 0)?;
    composite.copy_from(&img2, w1, 0)?;

    // Draw keypoints and matches
    let green = Rgb([0, 255, 0]);
    let red = Rgb([255, 0, 0]);

    // Draw keypoints on individual images
    for kp in keypoints1 {
        draw_cross_mut(&mut img1, red, kp.x as i32, kp.y as i32);
    }
    for kp in keypoints2 {
        draw_cross_mut(&mut img2, red, kp.x as i32, kp.y as i32);
    }

    // Draw matches on composite
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

    // Create windows
    let options = WindowOptions::default();
    
    // Convert images to ImageView
    let img1_view = ImageView::new(Image::from(img1));
    let img2_view = ImageView::new(Image::from(img2));
    let comp_view = ImageView::new(Image::from(composite));

    // Display images
    let window1 = show_image::window("Image 1", options.clone())?;
    window1.set_image("img1", img1_view)?;

    let window2 = show_image::window("Image 2", options.clone())?;
    window2.set_image("img2", img2_view)?;

    let window_comp = show_image::window("Matches", options)?;
    window_comp.set_image("matches", comp_view)?;

    // Keep windows open
    window1.wait_until_destroyed()?;
    Ok(())
}
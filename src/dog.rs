// src/dog.rs
use ndarray::Array2;

pub struct DoGPyramid {
    pub octaves: Vec<Vec<Array2<f32>>>,
}

impl DoGPyramid {
    pub fn from_gaussian(gaussian: &GaussianPyramid) -> Self {
        let mut dog = Vec::with_capacity(gaussian.octaves.len());
        
        for octave in &gaussian.octaves {
            let mut dog_octave = Vec::with_capacity(octave.len() - 1);
            for i in 0..octave.len() - 1 {
                dog_octave.push(octave[i+1].clone() - octave[i].clone());
            }
            dog.push(dog_octave);
        }
        
        DoGPyramid { octaves: dog }
    }
    
    pub fn find_keypoints(&self) -> Vec<KeyPoint> {
        // Extremum detection in 3D scale-space
        unimplemented!()
    }
}
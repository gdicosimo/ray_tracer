use std::{env, fmt, io, path::PathBuf};

use image::ImageError as GenericImageError;

use crate::math::{color_to_bytes, Color};

#[derive(Debug)]
pub enum ImageError {
    NotFound(String),
    LoadError(String),
    IOError(io::Error),
    ImageCrateError(GenericImageError),
    InvalidPath(String),
}

pub struct RtwImage {
    width: u32,
    height: u32,
    byte_data: Vec<u8>,
}

impl RtwImage {
    pub fn try_new(filename: &str) -> Result<Self, ImageError> {
        let file_path = Self::try_find_image_file(filename)?;
        Self::try_load_from_file(&file_path)
    }

    fn try_find_image_file(filename: &str) -> Result<PathBuf, ImageError> {
        if filename.is_empty() || filename.contains('\0') {
            return Err(ImageError::InvalidPath(filename.to_string()));
        }

        let image_dir_env = env::var("RTW_IMAGES").ok();

        let paths_to_check = vec![
            image_dir_env.map(|dir| PathBuf::from(format!("{}/{}", dir, filename))),
            Some(PathBuf::from(filename)),
            Some(PathBuf::from(format!("images/{}", filename))),
            Some(PathBuf::from(format!("assets/images/{}", filename))),
            Some(PathBuf::from(format!("../images/{}", filename))),
            Some(PathBuf::from(format!("../../images/{}", filename))),
            Some(PathBuf::from(format!("../../assets/images/{}", filename))),
        ]
        .into_iter()
        .flatten();

        for path in paths_to_check {
            if path.exists() {
                return Ok(path);
            }
        }

        Err(ImageError::NotFound(filename.to_string()))
    }

    fn try_load_from_file(file_path: &PathBuf) -> Result<Self, ImageError> {
        match image::open(file_path) {
            Ok(dyn_image) => {
                let rgb_image = dyn_image.to_rgb8();
                let width = rgb_image.width();
                let height = rgb_image.height();

                let byte_data: Vec<u8> = rgb_image.to_vec();

                Ok(Self {
                    width,
                    height,
                    byte_data,
                })
            }
            Err(error) => Err(ImageError::LoadError(format!(
                "Error loading image from path {:?}: {}",
                file_path, error
            ))),
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn pixel_data(&self, x: u32, y: u32) -> &[u8] {
        static MAGENTA: [u8; 3] = [255, 0, 255];
        if self.byte_data.is_empty() {
            return &MAGENTA;
        }

        let clamped_x = x.clamp(0, self.width.saturating_sub(1));
        let clamped_y = y.clamp(0, self.height.saturating_sub(1));
        let index = (clamped_y * self.width + clamped_x) * 3;
        &self.byte_data[(index as usize)..(index as usize + 3)]
    }
}

impl From<io::Error> for ImageError {
    fn from(error: io::Error) -> Self {
        ImageError::IOError(error)
    }
}

impl From<GenericImageError> for ImageError {
    fn from(error: GenericImageError) -> Self {
        ImageError::ImageCrateError(error)
    }
}

impl fmt::Display for ImageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImageError::NotFound(s) => write!(f, "Image not found: {}", s),
            ImageError::LoadError(s) => write!(f, "Load error: {}", s),
            ImageError::IOError(err) => write!(f, "IO error: {}", err),
            ImageError::ImageCrateError(err) => write!(f, "Image crate error: {}", err),
            ImageError::InvalidPath(s) => write!(f, "Invalid path: {}", s),
        }
    }
}

pub fn save_as_png(
    width: u32,
    height: u32,
    pixels: &[u8],
    filename: &str,
) -> Result<(), ImageError> {
    let img = image::RgbImage::from_raw(width, height, pixels.to_vec())
        .ok_or_else(|| ImageError::LoadError("Failed to create image from raw data".to_string()))?;

    img.save(filename).map_err(ImageError::ImageCrateError)
}

pub fn save_as_ppm_from_floats(
    width: u32,
    height: u32,
    pixels: &[f32],
    filename: &str,
) -> Result<(), ImageError> {
    use std::fs::File;
    use std::io::Write;

    let mut file = File::create(filename)?;
    writeln!(file, "P3")?;
    writeln!(file, "{} {}", width, height)?;
    writeln!(file, "255")?;

    for chunk in pixels.chunks(3) {
        let col = Color::new(chunk[0], chunk[1], chunk[2]);
        let [r, g, b] = color_to_bytes(col);
        writeln!(file, "{} {} {}", r, g, b)?;
    }
    Ok(())
}

pub fn save_as_png_from_floats(
    width: u32,
    height: u32,
    pixels: &[f32],
    filename: &str,
) -> Result<(), ImageError> {
    let byte_data: Vec<u8> = pixels
        .chunks(3)
        .flat_map(|chunk| {
            let col = Color::new(chunk[0], chunk[1], chunk[2]);
            color_to_bytes(col).to_vec()
        })
        .collect();

    save_as_png(width, height, &byte_data, filename)
}

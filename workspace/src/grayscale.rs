use image::DynamicImage;

pub trait GrayScale {
    fn grayscale(img: DynamicImage) -> DynamicImage;
}

pub struct ProcessImage {

}

impl GrayScale for ProcessImage {
    fn grayscale(img: DynamicImage) -> DynamicImage {
        return img.grayscale()
    }
}
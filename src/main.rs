mod kernels;
use kernels::*;
mod fourier;

use image::{GenericImageView, GrayImage};
fn main() 
{
    let img_name = "helena.jpg";
    let kernel = Kernel::gaussian(5, 2.0);

}

fn image_to_vector(path: &str) -> Vec<Vec<f64>>
{
    todo!();
}

fn vector_as_image(image: &Vec<Vec<f64>>, path: &str)
{
    todo!();
}
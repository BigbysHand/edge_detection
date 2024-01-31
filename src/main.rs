mod kernels;
use kernels::*;
mod fourier;
mod convolution;
//use convolution;

use image::{GenericImageView, GrayImage};
fn main()
{
    let img_name = "helena.jpg";
    let kernel = Kernel::edge_x();

    println!("image loaded");
    let image = image_to_vector(std::format!("C:/dev/rust/edge_detection/input_images/{}", img_name));
    println!("image conv");
    let conv_img = convolution::conv_2d(&image, &kernel);
    println!("image saved");
    vector_as_image(&conv_img, std::format!("C:/dev/rust/edge_detection/output_images/{}", img_name));

}

//conv img to vector 
fn image_to_vector(path: String) -> Vec<Vec<f64>>
{
    //get image + dimensions
    let img = image::open(path).unwrap();
    let (width, height) = img.dimensions();

    //define empty vector of required size
    let mut img_vec = vec![vec![0.0; (width as usize)]; (height as usize)];

    //set greyscaled pixel values of image
    for y in 0..height
    {
        for x in 0..width
        {
            let this_pix = img.get_pixel(x, y);
            let curr_pix = this_pix.0;
            let intensity = (0.33 * curr_pix[0] as f64 + 0.33 * curr_pix[1] as f64 + 0.33 * curr_pix[2] as f64) / (255.0);
            img_vec[y as usize][x as usize] = intensity;
        }
    }

    return img_vec;

    
}

//conv vector to img
fn vector_as_image(image: &Vec<Vec<f64>>, path: String)
{
    // convert to grey image
    let mut ouput = GrayImage::new(image[0].len() as u32, image.len() as u32);
    for (x, y, pixel) in ouput.enumerate_pixels_mut() 
    {
        let gray_value = (image[x as usize][y as usize] * 255.0) as u8;
        *pixel = image::Luma([gray_value]);
    }
    // save the image
    ouput.save(path).unwrap();
}
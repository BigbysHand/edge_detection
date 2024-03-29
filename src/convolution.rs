//mod fourier;
use crate::fourier;
use crate::padding;

use num::complex::Complex;

pub fn conv_2d(input: &Vec<Vec<f64>>, kernel: &Vec<Vec<f64>>) -> Vec<Vec<f64>>
{
    //get img dimenstions
    let width = input[0].len();
    let height = input.len();


    //flatten both image and kernel
    let image = input
        .to_owned()
        .into_iter()
        .flatten()
        .collect::<Vec<f64>>();

    let flat_kernel = kernel
        .to_owned()
        .into_iter()
        .flatten()
        .collect::<Vec<f64>>();

    //make padded kernel
    let mut padded_kernel = vec![0.0; image.len()];
    for i in 0..flat_kernel.len()
    {
        padded_kernel[i] = flat_kernel[i];
    }

    //perform fft
    let image_fft = fourier::fft(&image);
    let kernel_fft = fourier::fft(&padded_kernel);

    //perfrom element wise multiplication
    let filtered_img: Vec<Complex<f64>> = image_fft
        .iter()
        .zip(kernel_fft.iter())
        .map(|(a, b)| a * b)
        .collect();

    //ifft
    let image_ifft = fourier::ifft(&filtered_img);

    //reconstruct 2d image
    let mut result_image = vec![vec![0.0; width]; height];
    for y in 0..height
    {
        for x in 0..width
        {
            result_image[y][x] = image_ifft[y as usize * width + x as usize];
        }
    }
    return result_image;
}

pub fn conv_padded_2d(input: &Vec<Vec<f64>>, kernel: &Vec<Vec<f64>>) -> Vec<Vec<f64>>
{
    //get img dimenstions
    let width = input[0].len();
    let height = input.len();

    let padded_input = padding::basic_padding(&input, kernel.len());

    //flatten both image and kernel
    let image = padded_input
        .to_owned()
        .into_iter()
        .flatten()
        .collect::<Vec<f64>>();

    let flat_kernel = kernel
        .to_owned()
        .into_iter()
        .flatten()
        .collect::<Vec<f64>>();

    //make padded kernel
    let mut padded_kernel = vec![0.0; image.len()];
    for i in 0..flat_kernel.len()
    {
        padded_kernel[i] = flat_kernel[i];
    }


    //perform fft
    let image_fft = fourier::fft(&image);
    let kernel_fft = fourier::fft(&padded_kernel);

    //perfrom element wise multiplication
    let filtered_img: Vec<Complex<f64>> = image_fft
        .iter()
        .zip(kernel_fft.iter())
        .map(|(a, b)| a * b)
        .collect();

    //ifft
    let image_ifft = fourier::ifft(&filtered_img);

    //reconstruct 2d image
    let mut result_image = vec![vec![0.0; width]; height];
    for y in 0..height
    {
        for x in 0..width
        {
            result_image[y][x] = image_ifft[y as usize * width + x as usize];
        }
    }
    return result_image;
}
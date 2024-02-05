use std::f64::consts::PI;

use num::{complex::Complex, Zero};

pub fn fft(x: &Vec<f64>) ->Vec<Complex<f64>>
{
    //make complex
    let mut input = x
    .iter()
    .map(|v| Complex::new(*v, 0.0))
    .collect::<Vec<Complex<f64>>>();

    //pad the image to be an even number
    input.resize(x.len().next_power_of_two(), Complex::zero());

    //perform fft
    let input = post_fft(&input);

    return input;
}

fn post_fft(x: &Vec<Complex<f64>>) -> Vec<Complex<f64>>
{
    //Cooley-Tuckey alg
    let l = x.len();

    if l == 1
    {
        return x.to_owned();
    }

    let even: Vec<Complex<f64>> = x.iter().step_by(2).cloned().collect();
    let odd: Vec<Complex<f64>> = x.iter().skip(1).step_by(2).cloned().collect();
    
    let even = post_fft(&even);
    let odd = post_fft(&odd);

    let mut dft = vec![Complex {im: 0.0, re: 0.0}; l];
    let c = Complex {im: -2.0, re: 0.0};

    for k in 0..(l/2)
    {
        let t = odd[k] * (c * PI * k as f64 / l as f64).exp();
        dft[k] = even[k] + t;
        dft[k + l / 2] = even[k] - t;
    }

    return dft;


}

pub fn ifft(x: &Vec<Complex<f64>>) -> Vec<f64>
{

    if !x.len().is_power_of_two()
    {
        panic!("Not even");
    } else 
    {
        let out = post_ifft(x);
        return out
            .to_owned()
            .into_iter()
            .map(|v| v.norm())
            .collect::<Vec<f64>>();
    }

}

fn post_ifft(x: &Vec<Complex<f64>>) -> Vec<Complex<f64>>
{
    let l = x.len();

    if l == 1
    {
        return x.to_owned();
    }

    let even: Vec<Complex<f64>> = x.iter().step_by(2).cloned().collect();
    let odd: Vec<Complex<f64>> = x.iter().skip(1).step_by(2).cloned().collect();

    let even = post_ifft(&even);
    let odd = post_ifft(&odd);

    let mut idft = vec![Complex {im: 0.0, re: 0.0}; l];
    let c = Complex {im: -2.0, re: 0.0};

    for k in 0..(l / 2)
    {
        let t = odd[k] * (c * PI * k as f64 / l as f64).exp();
        idft[k] = (even[k] + t ) / 2.0;
        idft[k + l/2] = (even[k] - t) / 2.0;
    }

    return idft;
}
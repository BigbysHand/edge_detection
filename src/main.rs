mod kernels;
use kernels::*;
mod fourier;
fn main() 
{
    let kernel = Kernel::gaussian(5, 2.0);

}

/*
to-do list:

Build a fft alg
Build a convolution alg
*/
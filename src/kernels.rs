pub type Kernel = Vec<Vec<f64>>;

pub trait KernelConstruct
{
    fn identity() -> Self;
    fn gaussian(size: usize, sigma: f64) -> Self;
    fn edge_x() -> Self;
    fn edge_y() -> Self;
    fn edge_all() -> Self;
    fn edge_enhance() -> Self;
}

impl KernelConstruct for Kernel
{
    //identity of image (same as input image)
    fn identity() -> Self
    {
        vec!
        [
            vec![0.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0],
            vec![0.0, 0.0, 0.0]
        ]
    }
    
    fn gaussian(size: usize, sigma: f64) -> Self
    {
        let mid = (size / 2) as f64;
        let var = sigma * sigma;

        let mut kernel: Vec<Vec<f64>> = vec!
        [
            vec!
            [
                0.0;
                size
            ];
            size
        ];

        //define generic unnormalised gaussian
        for i in 0..size
        {
            for j in 0..size
            {
                let x = (i as f64) - mid;
                let y = (j as f64) - mid;

                kernel[i][j] = 1.0/(2.0*std::f64::consts::PI * var) *
                 (-1.0*((x*x + y*y) / 2.0*var)).exp();
            }
        }
        //normalise to regulate intensity
        let sum: f64 = kernel.iter().flatten().sum();
        for i in 0..size
        {
            for j in 0..size
            {
                kernel[i][j] /= sum;
            }
        }


        kernel


    }
    //detects vertical lines
    fn edge_x() -> Self
    {
        vec!
        [
            vec![-1.0, 0.0, 1.0],
            vec![-2.0, 0.0, 1.0],
            vec![-1.0, 0.0, 1.0]
        ]
    }
    //detects horizontal lines
    fn edge_y() -> Self
    {
        vec!
        [
            vec![-1.0, -2.0, -1.0],
            vec![ 0.0,  0.0,  0.0],
            vec![ 1.0,  2.0,  1.0]
        ]
    }

    fn edge_all() -> Self
    {
        todo!()
    }

    fn edge_enhance() -> Self
    {
        todo!()
    }

}
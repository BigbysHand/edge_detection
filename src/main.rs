pub fn conv(_input: Vec<Vec<f32>>, _kernel: Vec<Vec<f32>>, stride: usize)->Vec<Vec<f32>>
{
    let output: Vec<Vec<f32>> = Vec::new();

    let row: usize = _input.len();
    let col: usize =  _input[0].len();

    let k_row: usize = _kernel.len();
    let k_col: usize = _kernel[0].len();
    
    //defin the bounds of the iteration
    //number of kernel values in_row - ker-row / stride
    let out_rows: usize = (row - k_row) / stride;
    let out_col: usize = (col - k_col) / stride;




    



    return output;
}

fn main() 
{

    let mut a: Vec<Vec<f32>> = Vec::new();

    

}
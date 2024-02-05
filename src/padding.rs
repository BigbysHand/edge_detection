pub fn basic_padding(input: &Vec<Vec<f64>>, kernel_width: usize) -> Vec<Vec<f64>>
{
    let mut padded: Vec<Vec<f64>> = vec![vec![0.0; input[0].len() + kernel_width - 1]; input.len() + kernel_width - 1];
    for y in 0..input.len()
    {
        for x in 0..input[0].len()
        {
            padded[y + kernel_width / 2][x + kernel_width / 2] = input[y][x];
        }
    }
    return padded;
}
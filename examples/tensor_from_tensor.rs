// For python the data type is automatically inferred from the data
// In Rust, we need to specify the data type

// Pytorch equivalent:
// zero_tensor = torch.zeros_like(tensor)
// ones_tensor = torch.ones_like(tensor)
// random_tensor = torch.rand_like(tensor)

use anyhow::Result;
use candle_core::{DType, Device, Tensor};

fn main() -> Result<()> {
    let data: [u32; 3] = [1, 2, 3];
    let tensor = Tensor::new(&data, &Device::Cpu)?;

    let zero_tensor = tensor.zeros_like()?;
    println!("zero_tensor: {:?}", zero_tensor.to_vec1::<u32>()?);

    let ones_tensor = tensor.ones_like()?;
    println!("ones_tensor: {:?}", ones_tensor.to_vec1::<u32>()?);

    // TODO: Need to update this in tutorial
    let random_tensor = tensor.to_dtype(DType::F64)?.rand_like(0.0, 1.0)?;
    println!("random_tensor: {:?}", random_tensor.to_vec1::<f64>()?);

    Ok(())
}

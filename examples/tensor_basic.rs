// For python the data type is automatically inferred from the data
// In Rust, we need to specify the data type

// Pytorch equivalent:
// import torch
// from typing import List

// data: List = [1, 2, 3]
// tensor = torch.tensor(data)
// print(tensor)

// nested_data = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
// nested_tensor = torch.tensor(nested_data)
// print(nested_tensor)

use anyhow::Result;
use candle_core::{Device, Tensor};

fn main() -> Result<()> {
    let data: [u32; 3] = [1u32, 2, 3];
    let tensor = Tensor::new(&data, &Device::Cpu)?;
    println!("tensor: {:?}", tensor.to_vec1::<u32>()?);

    let nested_data: [[u32; 3]; 3] = [[1u32, 2, 3], [4, 5, 6], [7, 8, 9]];
    let nested_tensor = Tensor::new(&nested_data, &Device::Cpu)?;
    println!("nested_tensor: {:?}", nested_tensor.to_vec2::<u32>()?);

    Ok(())
}

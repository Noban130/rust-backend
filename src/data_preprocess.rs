use rand::Rng; // Add rand crate to your Cargo.toml
use std::iter::repeat_with;

fn split_data_randomly<T>(data: &[T], min_chunk_size: usize, max_chunk_size: usize) -> Vec<Vec<T>>
where
    T: Clone,
{
    let mut rng = rand::thread_rng();
    let mut chunks = Vec::new();
    let mut start_index = 0;

    while start_index < data.len() {
        let remaining_len = data.len() - start_index;
        let chunk_size = rng.gen_range(min_chunk_size..=remaining_len.min(max_chunk_size));
        let end_index = start_index + chunk_size;
        chunks.push(data[start_index..end_index].to_vec());
        start_index = end_index;
    }

    chunks
}
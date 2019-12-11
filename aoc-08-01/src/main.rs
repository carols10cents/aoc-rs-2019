use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

struct SpaceImage {
    layers: Vec<Vec<u32>>,
}

impl SpaceImage {
    fn new(width: usize, height: usize, data: &str) -> SpaceImage {
        let digits: Vec<_> = data.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let layers: Vec<_> = digits.chunks(width * height).map(|s| s.to_vec()).collect();

        SpaceImage { layers }
    }

    fn digits_per_layer(&self) -> Vec<HashMap<u32, usize>> {
        self.layers.iter().map(|_| HashMap::new()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_data_into_images() {
        let data = "123456789012";
        let sample = SpaceImage::new(3, 2, data);

        assert_eq!(sample.layers.len(), 2);
        assert_eq!(sample.layers[0], vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(sample.layers[1], vec![7, 8, 9, 0, 1, 2]);
    }

    #[test]
    fn counting_digits_per_layer() {
        let data = "113456789012";
        let sample = SpaceImage::new(3, 2, data);

        let digits_per_layer = sample.digits_per_layer();
        assert_eq!(digits_per_layer[0].get(&1), Some(2));
        assert_eq!(digits_per_layer[1].get(&1), Some(1));
    }
}

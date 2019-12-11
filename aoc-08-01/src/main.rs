fn main() {
    println!("Hello, world!");
}

struct SpaceImage {
    layers: Vec<Vec<i32>>,
}

impl SpaceImage {
    fn new(width: usize, height: usize, data: &str) -> SpaceImage {
        let digits: Vec<_> = data.chars().map(|c| c.parse().unwrap()).collect();
        let layers: Vec<_> = digits.chunks(width * height).collect();

        SpaceImage { layers }
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
}

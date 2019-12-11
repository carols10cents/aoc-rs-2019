use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");
    let image = SpaceImage::new(25, 6, input);

    let digits_per_layer = image.digits_per_layer();

    let layer_with_fewest_0s = digits_per_layer
        .iter()
        .min_by_key(|counts| counts.get(&0).copied().unwrap_or(0))
        .expect("Image should not be empty");

    println!(
        "{}",
        layer_with_fewest_0s.get(&1).copied().unwrap_or(0)
            * layer_with_fewest_0s.get(&2).copied().unwrap_or(0)
    );
}

struct SpaceImage {
    layers: Vec<Vec<u32>>,
}

impl SpaceImage {
    fn new(width: usize, height: usize, data: &str) -> SpaceImage {
        let digits: Vec<_> = data.trim().chars().map(|c| c.to_digit(10).unwrap()).collect();
        let layers: Vec<_> = digits.chunks(width * height).map(|s| s.to_vec()).collect();

        SpaceImage { layers }
    }

    fn digits_per_layer(&self) -> Vec<HashMap<u32, usize>> {
        self.layers
            .iter()
            .map(|layer| {
                let mut counts = HashMap::new();

                for &digit in layer {
                    *counts.entry(digit).or_insert(0) += 1;
                }

                counts
            })
            .collect()
    }

    fn flatten(&self()) -> Vec<u32> {
        self.layers[0].cloned()
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
        assert_eq!(digits_per_layer[0].get(&1), Some(&2));
        assert_eq!(digits_per_layer[1].get(&1), Some(&1));
    }

    #[test]
    fn flatten_layers() {
        let data = "0222112222120000";
        let sample = SpaceImage::new(2, 2, data);

        let flat = sample.flatten();
        assert_eq!(flat, vec![0, 1, 1, 0]);
    }
}

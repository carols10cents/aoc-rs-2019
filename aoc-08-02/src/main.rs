use std::collections::HashMap;
use std::fmt;

fn main() {
    let input = include_str!("../input");
    let image = SpaceImage::new(25, 6, input);

    println!("{}", image);
}

struct SpaceImage {
    width: usize,
    layers: Vec<Vec<u32>>,
}

impl fmt::Display for SpaceImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.flatten()
                .chunks(self.width)
                .map(|row| row.iter().map(|pixel| pixel.to_string()).collect::<Vec<_>>().join(""))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

impl SpaceImage {
    fn new(width: usize, height: usize, data: &str) -> SpaceImage {
        let digits: Vec<_> = data
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        let layers: Vec<_> = digits.chunks(width * height).map(|s| s.to_vec()).collect();

        SpaceImage { width, layers }
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

    fn flatten(&self) -> Vec<u32> {
        (0..self.layers[0].len())
            .into_iter()
            .map(|i| {
                self.layers
                    .iter()
                    .map(|layer| layer[i])
                    .skip_while(|&pixel| pixel == 2)
                    .next()
                    .expect("All locations should have a color pixel in some layer")
            })
            .collect()
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

    #[test]
    fn display_flattened_image_according_to_width_and_height() {
        let data = "0222112222120000";
        let sample = SpaceImage::new(2, 2, data);

        assert_eq!(sample.to_string(), "01\n10");
    }
}

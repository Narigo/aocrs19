use itertools::Itertools;
use std::fs;

type ProgramInput = String;
type Color = u8;
type LayerNumber = usize;
type Layer = (LayerNumber, Vec<Color>);
type LayerChecksum = (LayerNumber, u32);
type Height = u8;
type Width = u8;

pub fn space_image_format() -> u32 {
  let filename = "./src/aoc08/input.txt";
  let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
  let image = Image::new(6, 25, &contents);
  image.checksum()
}

pub fn draw_image() -> String {
  let filename = "./src/aoc08/input.txt";
  let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
  let image = Image::new(6, 25, &contents);
  image.draw()
}

struct Image {
  dimensions: (Height, Width),
  layers: Vec<Layer>,
}

impl Image {
  pub fn new(height: Height, width: Width, input: &String) -> Image {
    let length = input.len();
    let expected_length: usize = (height * width) as usize;
    assert_eq!(0, length % expected_length);
    let mut layers = Vec::new();
    let mut layer_number: usize = 0;
    for pixels_in_layer in input.chars().chunks(expected_length).into_iter() {
      let pixels: Vec<u8> = pixels_in_layer
        .map(|digit| {
          digit
            .to_string()
            .parse::<u8>()
            .expect("should always have digits")
        })
        .collect();
      layers.push((layer_number, pixels));
      layer_number = layer_number + 1;
    }
    Image {
      dimensions: (height, width),
      layers,
    }
  }

  fn find_layer_with_fewest_zero_digits(self: &Image) -> LayerChecksum {
    let layer_info: (usize, u32) = self.layers.iter().fold(
      (0, std::u32::MAX),
      |(number_of_layer_with_max_zeros, min_number_of_zeros), (layer_number, points)| {
        let number_of_zeros_in_current_layer = points.iter().fold(0, |zeros, point| {
          zeros
            + (match point {
              0 => 1,
              _ => 0,
            })
        });
        if number_of_zeros_in_current_layer < min_number_of_zeros {
          (*layer_number, number_of_zeros_in_current_layer)
        } else {
          (number_of_layer_with_max_zeros, min_number_of_zeros)
        }
      },
    );
    layer_info
  }
  fn checksum(self: &Image) -> u32 {
    let layer = self.find_layer_with_fewest_zero_digits();
    let one_digits = self.layers[layer.0].1.iter().fold(0, |sum, digit| {
      sum
        + match digit {
          1 => 1,
          _ => 0,
        }
    });
    let two_digits = self.layers[layer.0].1.iter().fold(0, |sum, digit| {
      sum
        + match digit {
          2 => 1,
          _ => 0,
        }
    });
    one_digits * two_digits
  }
  fn find_color_in_layers_at(self: &Image, index: usize) -> &'static str {
    for (_, pixels) in self.layers.iter() {
      match pixels[index] {
        0 => return " ",
        1 => return "#",
        _ => continue,
      }
    }
    return "X";
  }
  fn draw(self: &Image) -> String {
    let length = self.layers[0].1.len() + self.dimensions.0 as usize;
    let mut result: Vec<&'static str> = Vec::with_capacity(length);
    let mut index: usize = 0;
    for num in self.layers[0].1.iter() {
      print!("{}", num);
      result.push(self.find_color_in_layers_at(index));
      index = index + 1;
      if (index % self.dimensions.1 as usize) == 0 {
        print!("\n");
        result.push("\n");
      }
    }
    result.iter().map(|s| s.to_owned()).collect()
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn example_layer() {
    let input = "123456789012";
    let width = 3;
    let height = 2;
    let expected = 1;
    let image = Image::new(height, width, &input.to_owned());
    let (layer_number, number_of_zeros) = image.find_layer_with_fewest_zero_digits();
    assert_eq!(0, layer_number);
    assert_eq!(0, number_of_zeros);
    let result = image.checksum();
    assert_eq!(expected, result);
  }
}

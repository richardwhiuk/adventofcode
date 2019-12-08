pub fn run_a() {
    let data = std::fs::read_to_string("8.txt").expect("Unable to read file");
    println!("8a: {}", get_result(data, 25, 6));
}

pub fn run_b() {
    let data = std::fs::read_to_string("8.txt").expect("Unable to read file");
    let image = get_image(data, 25, 6);
    println!("8b:");
    for n in 0..6 {
        println!("{}", &image[(n * 25)..((n + 1) * 25)]);
    }
}

pub fn get_image(data: String, width: usize, height: usize) -> String {
    let data: Vec<u32> = data.chars().filter_map(|c| c.to_digit(10)).collect();
    let layers = split_into_layers(data, width * height);

    let mut image = vec![];
    let mut first = true;

    for layer in layers {
        let mut n = 0;
        for pixel in layer {
            if first {
                image.push(pixel);
            } else {
                if image[n] == 2 {
                    image[n] = pixel;
                }
            }
            n += 1;
        }
        first = false;
    }

    let image: Vec<String> = image
        .iter()
        .map(|c| (if *c == 0 { " " } else { "#" }).to_string())
        .collect();
    image.join("")
}

pub fn get_result(data: String, width: usize, height: usize) -> u32 {
    let data: Vec<u32> = data.chars().filter_map(|c| c.to_digit(10)).collect();

    let layers = split_into_layers(data, width * height);
    let layer = least_layer(layers);

    let result = get_n_in_layer(&layer, 1) * get_n_in_layer(&layer, 2);

    result
}

fn split_into_layers(data: Vec<u32>, size: usize) -> Vec<Vec<u32>> {
    let mut layers = vec![];
    let mut n = 0;

    while n < data.len() {
        layers.push(data[n..(n + size)].to_vec());
        n += size;
    }

    layers
}

fn least_layer(data: Vec<Vec<u32>>) -> Vec<u32> {
    let mut best_layer = None;
    let mut best_zeros = std::u32::MAX;

    for layer in data {
        let zeros = get_n_in_layer(&layer, 0);

        if zeros < best_zeros {
            best_layer = Some(layer);
            best_zeros = zeros;
        }
    }

    best_layer.expect("No layer found!")
}

fn get_n_in_layer(layer: &Vec<u32>, n: u32) -> u32 {
    layer
        .iter()
        .fold(0, |total, i| total + if *i == n { 1 } else { 0 })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(get_result("123456789012".to_string(), 3, 2), 1);
    }

    #[test]
    fn test_b() {
        assert_eq!(get_image("0222112222120000".to_string(), 2, 2), "0110");
    }
}

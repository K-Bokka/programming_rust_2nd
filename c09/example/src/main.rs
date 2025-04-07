fn main() {
    let width = 1024;
    let height = 576;
    let image = GrayscaleMap {
        pixels: vec![0; width * height],
        size: (width, height),
    };

    assert_eq!(image.size, (1024, 576));
    assert_eq!(image.pixels.len(), 1024 * 765);

    let image2 = new_map((1024, 576), vec![0; 1024 * 576]);

    println!(
        "pixel len is {}, image is {:?}",
        image2.pixels.len(),
        image2.size
    );

    let hokey = Broom {
        name: "Hokey".to_string(),
        height: 60,
        health: 100,
        position: (100.0, 200.0, 0.0),
        intent: BroomIntent::FetchWater,
    };
    let (hokey1, hokey2) = chop(hokey);
    assert_eq!(hokey1.name, "Hokey I");
    assert_eq!(hokey1.height, 30);
    assert_eq!(hokey1.health, 100);
    assert_eq!(hokey2.name, "Hokey II");
    assert_eq!(hokey2.height, 30);
    assert_eq!(hokey2.health, 100);
}
#[derive(Debug)]
struct GrayscaleMap {
    pixels: Vec<u8>,
    size: (usize, usize),
}

fn new_map(size: (usize, usize), pixels: Vec<u8>) -> GrayscaleMap {
    assert_eq!(pixels.len(), size.0 * size.1);
    GrayscaleMap { pixels, size }
}

struct Broom {
    name: String,
    height: u32,
    health: u32,
    #[allow(dead_code)]
    position: (f32, f32, f32),
    #[allow(dead_code)]
    intent: BroomIntent,
}

#[derive(Copy, Clone)]
enum BroomIntent {
    FetchWater,
    #[allow(dead_code)]
    DumpWater,
}

fn chop(b: Broom) -> (Broom, Broom) {
    let mut broom1 = Broom {
        height: b.height / 2,
        ..b
    };
    let mut broom2 = Broom {
        name: broom1.name.clone(),
        ..broom1
    };
    broom1.name.push_str(" I");
    broom2.name.push_str(" II");
    (broom1, broom2)
}

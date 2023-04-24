fn main() {
    // Set up variables
    const IMAGE_WIDTH: i16 = 256;
    const IMAGE_HEIGHT: i16 = 256;

    let mut r:i32;
    let mut g:i32;
    let mut b:i32;

    // Set up PPM file
    // Output image using cargo run > image.ppm
    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n 255");

    // Render
    for j in (0..IMAGE_HEIGHT-1).rev(){
        // Progress Indicator
        eprintln!("{}%", ((j as f32 / IMAGE_HEIGHT as f32)*100.0) as i8);
        for i in 0..IMAGE_WIDTH{
            r = (255.99 * (i as f32) / (IMAGE_WIDTH-1) as f32) as i32;
            g = (255.99 * (j as f32) / (IMAGE_HEIGHT-1) as f32) as i32;
            b = (255.99 * 0.25) as i32;
            println!("{r} {g} {b}");
        }
    }
}

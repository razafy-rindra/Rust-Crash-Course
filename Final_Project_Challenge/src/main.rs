


// **SUPER CHALLENGE FOR LATER** - Let's face it, you don't have time for this during class.
//
// Make all of the subcommands stackable!
//
// For example, if you run:
//
//   cargo run infile.png outfile.png blur 2.5 invert rotate 180 brighten 10
//
// ...then your program would:
// - read infile.png
// - apply a blur of 2.5
// - invert the colors
// - rotate the image 180 degrees clockwise
// - brighten the image by 10
// - and write the result to outfile.png
//
// Good luck!


fn main() {
    // 1. First, you need to implement some basic command-line argument handling
    // so you can make your program do different things.  Here's a little bit
    // to get you started doing manual parsing.
    //
    // Challenge: If you're feeling really ambitious, you could delete this code
    // and use the "clap" library instead: https://docs.rs/clap/2.32.0/clap/
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        print_usage_and_exit();
    }
    let mut infile = args.remove(0);
    let outfile = args.remove(0);
    while !args.is_empty(){
   //     println!("vec: {:?}", args);
        let subcommand = args.remove(0);
        match subcommand.as_str() {
            "blur" => {
                if args.len() < 1 {
                    print_usage_and_exit();
                }
                let amount: f32 = args.remove(0).parse().expect("Failed to parse a number");
                // **OPTION**
                // Improve the blur implementation -- see the blur() function below
                blur(&infile, &outfile, amount);
            }
            "brighten" => {
                if args.len() < 1 {
                    print_usage_and_exit();
                }
                let amount: i32 = args.remove(0).parse().expect("Failed to parse a number");
                brighten(&infile, &outfile,amount);
            }    
            "crop" => {
                if args.len() < 4 {
                    print_usage_and_exit();
                }
                let x = args.remove(0).parse().expect("Failed to parse a number");
                let y = args.remove(0).parse().expect("Failed to parse a number");
                let width = args.remove(0).parse().expect("Failed to parse a number");
                let height = args.remove(0).parse().expect("Failed to parse a number");
                crop(&infile, &outfile, x, y, width, height);
            }
            "rotate" => {
                if args.len() < 1 {
                    print_usage_and_exit();
                }
                let angle: u32 = args.remove(0).parse().expect("Failed to parse a number");
                rotate(&infile, &outfile, angle);
            }
    
            "invert" => {
                invert(&infile, &outfile);
            }

            "grayscale" => {
                grayscale(&infile, &outfile);
    
            }
    
    
            // For everything else...
            _ => {
                print_usage_and_exit();
            }
        }
        infile = outfile.clone();
    }
}

fn print_usage_and_exit() {
    println!("USAGE (when in doubt, use a .png extension on your filenames)");
    println!("INFILE OUTFILE");
    println!("blur AMOUNT");
    println!("brighten AMOUNT");
    println!("crop x y WIDTH HEIGHT");
    println!("rotate");
    println!("invert");
    println!("grayscale");
    // **OPTION**
    // Print useful information about what subcommands and arguments you can use
    // println!("...");
    std::process::exit(-1);
}

fn blur(infile: &String, outfile: &String, amount: f32) {
    // Here's how you open an existing image file
    let c_infile = infile.clone();
    let c_outfile = outfile.clone();
    let img = image::open(c_infile).expect("Failed to open INFILE.");
    // **OPTION**
    // Parse the blur amount (an f32) from the command-line and pass it through
    // to this function, instead of hard-coding it to 2.0.
    let img2 = img.blur(amount);
    // Here's how you save an image to a file.
    img2.save(c_outfile).expect("Failed writing OUTFILE.");
}

fn brighten(infile: &String, outfile: &String, amount: i32) {
    // See blur() for an example of how to open / save an image.
    let c_infile = infile.clone();
    let c_outfile = outfile.clone();
    let img = image::open(c_infile).expect("Failed to open INFILE.");

    // .brighten() takes one argument, an i32.  Positive numbers brighten the
    // image. Negative numbers darken it.  It returns a new image.

    // Challenge: parse the brightness amount from the command-line and pass it
    // through to this function.

    let img2 = img.brighten(amount);
    img2.save(c_outfile).expect("Failed writing OUTFILE.");
}

// Good testing ocation is 500 350 200 200
fn crop(infile: &String, outfile: &String, x: u32, y: u32, width: u32, height: u32) {
    // See blur() for an example of how to open an image.
    let c_infile = infile.clone();
    let c_outfile = outfile.clone();

    let mut img = image::open(c_infile).expect("Failed to open INFILE.");

    // .crop() takes four arguments: x: u32, y: u32, width: u32, height: u32
    // You may hard-code them, if you like.  It returns a new image.

    let img2 = img.crop(x,y,width,height);

    // Challenge: parse the four values from the command-line and pass them
    // through to this function.

    // See blur() for an example of how to save the image.
    img2.save(c_outfile).expect("Failed writing OUTFILE.");
}

fn rotate(infile: &String, outfile: &String, angle: u32) {
    let c_infile = infile.clone();
    let c_outfile = outfile.clone();


    // See blur() for an example of how to open an image.
    let img = image::open(c_infile).expect("Failed to open INFILE.");

    // There are 3 rotate functions to choose from (all clockwise):
    //   .rotate90()
    //   .rotate180()
    //   .rotate270()
    // All three methods return a new image.  Pick one and use it!

    // Challenge: parse the rotation amount from the command-line, pass it
    // through to this function to select which method to call.

    let img2 = match angle{
        90 => img.rotate90(),
        180 => img.rotate180(),
        270 => img.rotate270(),
        _ => panic!("Angle must be 90,180 or 270"),
    };

    // See blur() for an example of how to save the image.

    img2.save(c_outfile).expect("Failed writing OUTFILE.");
}

fn invert(infile: &String, outfile: &String) {
    let c_infile = infile.clone();
    let c_outfile = outfile.clone();

    // See blur() for an example of how to open an image.
    let mut img = image::open(c_infile).expect("Failed to open INFILE.");

    // .invert() takes no arguments and converts the image in-place, so you
    // will use the same image to save out to a different file.
    img.invert();

    // See blur() for an example of how to save the image.
    img.save(c_outfile).expect("Failed writing OUTFILE.");
}

fn grayscale(infile: &String, outfile: &String) {
    let c_infile = infile.clone();
    let c_outfile = outfile.clone();

    // See blur() for an example of how to open an image.
    let img = image::open(c_infile).expect("Failed to open INFILE.");

    // .invert() takes no arguments and converts the image in-place, so you
    // will use the same image to save out to a different file.
    let img2 = img.grayscale();

    // See blur() for an example of how to save the image.
    img2.save(c_outfile).expect("Failed writing OUTFILE.");
}

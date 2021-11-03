extern crate image;
extern crate clap;

//use image::GenericImageView;
use std::env;
use std::path::Path;
use clap::{Arg, App};
use image::{GenericImage, GenericImageView};
use image::math::Rect;

fn main() {
    let matches = App::new("Image Condenser")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Turtleb01")
        .about("Removes useless rows and columns from a screenshot")
        .arg(Arg::with_name("reverse")
             .short("r")
             .long("reverse")
             .multiple(false)
             .help("Reverses changes made by imagecondenser"))
        .arg(Arg::with_name("threshold")
             .short("t")
             .long("threshold")
             .multiple(false)
             .takes_value(true)
             .default_value("1")
             .help("Sets the minimum amount of rows or columns to start reducing"))
        .arg(Arg::with_name("INPUT")
             .help("Input file")
             .required(true)
             .index(1))
        .arg(Arg::with_name("OUTPUT")
             .help("Output file (will override input if not set)")
             .index(2))
        .get_matches();

    if matches.is_present("reverse") {
        eprintln!("Reverse mode not implemented");
        return;
    }
    let inputfile = Path::new(matches.value_of("INPUT").unwrap());
    let outputfile = Path::new(
        if matches.is_present("OUTPUT") {
              matches.value_of("OUTPUT").unwrap()
        } else {
              matches.value_of("INPUT").unwrap()
        }
    );

    println!("Input: {:?} Output: {:?}", inputfile, outputfile);
    if !inputfile.exists() {
        eprintln!("Input file does not exist!");
        return;
    }

    let threshold = matches.value_of("threshold").unwrap().parse().unwrap_or(0);
    if threshold <= 0 {
        eprintln!("Threshold must be a positive non-zero integer!");
        return;
    }

    let mut img = image::open(inputfile).unwrap();
    let (imgx, imgy) = img.dimensions();
    let (mut remx, mut remy): (u32, u32) = (0, 0); //removed lines counters for crop at the end

    let mut remw: u32 = 0; //width of the identical zone
    let mut refimg = img.view(0,0,1,imgy).to_image(); //reference image
    let mut curimg; //current image

    let mut i: u32 = 0;
    while i < (imgx - remx) { //unlike iterator, this reads remx every time and actually stops when the output has become smaller
        curimg = img.view(i,0,1,imgy).to_image();
        if refimg == curimg {
            remw += 1;
        } else {
            refimg = curimg;
            if remw >= threshold {
                remw-=threshold-1;
                remx+=remw;
                img.copy_within(
                  Rect { x: i, y: 0, width: imgx-i, height: imgy },
                  i-remw, 0);
            }
            i-=remw;
            remw = 0;
        }
        i+=1;
    }

    //condition for when the file ends
    if remw >= threshold {
      remx+=remw-(threshold-1);
    }

    remw = 0;
    refimg = img.view(0,0,imgx,1).to_image();
    //todo: do this with scopes, much cooler

    i = 0;
    while i < (imgy - remy) {
        curimg = img.view(0,i,imgx,1).to_image();
        if refimg == curimg {
            remw += 1;
        } else {
            refimg = curimg;
            if remw >= threshold {
                remw-=threshold-1;
                remy+=remw;
                img.copy_within(
                  Rect { x: 0, y: i, width: imgx, height: imgy-i },
                  0, i-remw);
            }
            i-=remw;
            remw = 0;
        }
        i+=1;
    }

    //condition for when the file ends
    if remw >= threshold {
      remy+=remw-(threshold-1);
    }

    //actually remove removed area
    img.crop_imm(0,0,imgx-remx,imgy-remy).save(outputfile).unwrap();
}

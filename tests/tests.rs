use std::collections::HashSet;
use std::path::Path;

use png_reader::chunks::{bKGD_Greyscale, cHRM, eXIf, FromChunk, iCCP, IHDR, iTXt, PLTE, tEXt, tIME, tRNS_Indexed, zTXt};
use png_reader::png::PNG;

#[test]
fn wormomancer() {
    let png_path = "/pictures/wormomancer.png";

    let png = PNG::open(png_path);
    println!("{:?}", png);
    assert_eq!(png.get_image_data().len(), 756501);
}

#[test]
fn caught() {
    let png_path = r"D:\Pictures\caught.png";

    let png = PNG::open(png_path);
    println!("{:?}", png);

    println!("{:?}", IHDR::from_chunk(&png.chunks[0]));
    println!("{:?}", eXIf::from_chunk(&png.chunks[4]));

    assert_eq!(png.get_image_data().len(), 124928);
}

#[test]
fn broad_bolts() {
    let png_path = r"D:\Pictures\broad bolts.png";

    let png = PNG::open(png_path);
    println!("{:?}", png);

    println!("{:?}", IHDR::from_chunk(&png.chunks[0]));

    assert_eq!(png.get_image_data().len(), 2256000);
}

#[test]
fn gasm() {
    let png_path = r"D:\\Pictures\\145-1455614_captain-gachi-gasm.png";

    let png = PNG::open(png_path);
    println!("{:?}", png);
    png.chunks.iter().enumerate().for_each(|(i, c)| println!("{i} {} {}", c, c.data.len()));

    println!("{:?}", IHDR::from_chunk(&png.chunks[0]));
    println!("{:?}", cHRM::from_chunk(&png.chunks[3]));
    println!("{:?}", bKGD_Greyscale::from_chunk(&png.chunks[4]));
    println!("{:?}", tEXt::from_chunk(&png.chunks[11]));

    assert_eq!(png.get_image_data().len(), 481636);
}

#[test]
fn four_set() {
    let png_path = r"D:\\Pictures\\4set.png";

    let png = PNG::open(png_path);
    println!("{:?}", png);
    png.chunks.iter().enumerate().for_each(|(i, c)| println!("{i} {} {}", c, c.data.len()));

    println!("{:?}", IHDR::from_chunk(&png.chunks[0]));
    println!("{:?}", iCCP::from_chunk(&png.chunks[1]));
    println!("{:?}", tIME::from_chunk(&png.chunks[4]));

    assert_eq!(png.get_image_data().len(), 2359296);
}

#[test]
fn blaspgemy() {
    let png_path = r"D:\\Pictures\\blaspgemy.png";

    let png = PNG::open(png_path);
    println!("{:?}", png);
    png.chunks.iter().enumerate().for_each(|(i, c)| println!("{i} {}", c));

    println!("{:?}", IHDR::from_chunk(&png.chunks[0]));
    println!("{:?}", zTXt::from_chunk(&png.chunks[1]));
    println!("{:?}", iCCP::from_chunk(&png.chunks[2]));
    println!("{:?}", tIME::from_chunk(&png.chunks[4]));

    assert_eq!(png.get_image_data().len(), 12589056);
}

#[test]
fn covfefe() {
    let png_path = r"D:\\Pictures\\covfefe.png";

    let png = PNG::open(png_path);
    println!("{:?}", png);
    png.chunks.iter().enumerate().for_each(|(i, c)| println!("{i} {}", c));

    println!("{:?}", IHDR::from_chunk(&png.chunks[0]));
    println!("{:?}", PLTE::from_chunk(&png.chunks[3]));
    println!("{:?}", tRNS_Indexed::from_chunk(&png.chunks[4]));

    assert_eq!(png.get_image_data().len(), 5796000);
}

#[test]
fn crazy_champ() {
    let png_path = r"D:\\Pictures\\crazyChamp.png";

    let png = PNG::open(png_path);
    println!("{:?}", png);
    png.chunks.iter().enumerate().for_each(|(i, c)| println!("{i} {}", c));

    println!("{:?}", IHDR::from_chunk(&png.chunks[0]));
    println!("{:?}", cHRM::from_chunk(&png.chunks[1]));
    println!("{:?}", iTXt::from_chunk(&png.chunks[3]));

    assert_eq!(png.get_image_data().len(), 50176);
}

#[test]
fn crawl() {
    let folder = Path::new(r"D:\Pictures");

    let mut all_tags = HashSet::new();
    for item in folder.read_dir().expect("Read failed") {
        if let Ok(item) = item {
            match item.path().extension() {
                Some(ext) => {
                    if ext.to_str().unwrap() == "png" {
                        println!("{:?}", item.path());

                        let png = PNG::open(item.path().to_str().unwrap());
                        let chunk_types = png.chunks.iter().map(|c| c.chunk_type.as_str())
                            .collect::<Vec<_>>();

                        for c in chunk_types {
                            if !all_tags.contains(c) {
                                println!("{:?}", c);
                                all_tags.insert(String::from(c));
                            }
                        }
                    }
                }
                None => {}
            }
        }
    }
}
#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use std::fs::File;
    use test::Bencher;

    use png::Decoder;

    use png_reader::png::PNG;

    use super::*;

    #[bench]
    fn bench_init_wormomancer(b: &mut Bencher) {
        b.iter(|| {
            let png_path = "/pictures/wormomancer.png";
            let png = PNG::open(png_path);
        });
    }

    #[bench]
    fn bench_load_data_wormomancer(b: &mut Bencher) {
        b.iter(|| {
            let png_path = "/pictures/wormomancer.png";
            let png = PNG::open(png_path);
            let data = png.get_image_data();
        });
    }

    #[bench]
    fn bench_init_covfefe(b: &mut Bencher) {
        b.iter(|| {
            let png_path = "/pictures/covfefe.png";
            let png = PNG::open(png_path);
        });
    }

    #[bench]
    fn bench_load_data_covfefe(b: &mut Bencher) {
        b.iter(|| {
            let png_path = "/pictures/covfefe.png";
            let png = PNG::open(png_path);
            let data = png.get_image_data();
        });
    }

    #[bench]
    fn bench_load_data_big_white(b: &mut Bencher) {
        b.iter(|| {
            let png_path = "/pictures/big_white.png";
            let png = PNG::open(png_path);
            let data = png.get_image_data();

            assert_eq!(data.len(), 192000000);
        });
    }

    #[bench]
    fn bench_load_data_big_white_png(b: &mut Bencher) {
        b.iter(|| {
            let png_path = "/pictures/big_white.png";
            let decoder = Decoder::new(File::open(png_path).unwrap());
            let mut reader = decoder.read_info().unwrap();

            // Allocate the output buffer.
            let mut buf = vec![0; reader.output_buffer_size()];
            // Read the next frame. An APNG might contain multiple frames.
            let info = reader.next_frame(&mut buf).unwrap();
            // Grab the bytes of the image.
            let bytes = &buf[..info.buffer_size()];

            assert_eq!(bytes.len(), 192000000);
        });
    }
}
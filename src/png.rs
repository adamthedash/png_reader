use std::cmp::min;
use std::fmt::{Debug, Display, Formatter};
use std::fs;
use std::io::Read;

use compress::zlib;
use rayon::prelude::*;

use crate::chunks::{FilterType, FromChunk, IHDR};
use crate::utils;

pub struct Chunk {
    length: u32,
    pub chunk_type: String,
    pub data: Vec<u8>,
    crc: Vec<u8>,
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Chunk (type: {}, length: {}, critical: {})", self.chunk_type, self.length, self.is_critical())
    }
}

impl Debug for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Chunk {
    pub fn from_byte_stream(stream: &mut &[u8]) -> Chunk {
        // Chunk length
        let chunk_length = utils::read_be_u32_mut(stream);

        // Chunk type
        let chunk_type = utils::read_str_mut(stream, 4);

        // Data
        let (data, rest) = stream.split_at(chunk_length as usize);
        *stream = rest;

        // CRC
        let (crc, rest) = stream.split_at(4);
        *stream = rest;

        Chunk {
            length: chunk_length,
            chunk_type,
            data: Vec::from(data),
            crc: Vec::from(crc),
        }
    }

    pub fn is_critical(&self) -> bool {
        self.chunk_type.chars().next().unwrap().is_ascii_uppercase()
    }
}


#[derive(Debug)]
pub struct PNG {
    pub chunks: Vec<Chunk>,
}


impl PNG {
    pub fn open(path: &str) -> PNG {
        let contents = fs::read(path).expect("Error opening file.");
        let header = &contents[..8];

        if header != vec![137, 80, 78, 71, 13, 10, 26, 10] {
            panic!("Invalid PNG header: {:?}", header.iter().map(|x| format!("{x:X}")).collect::<Vec<String>>())
        }

        // Generate chunks
        let mut body = &contents[8..];
        let mut chunks = Vec::new();
        while !body.is_empty() {
            chunks.push(Chunk::from_byte_stream(&mut body));
        }


        PNG {
            chunks
        }
    }

    /// https://www.w3.org/TR/PNG-Filters.html
    fn paeth(a: u8, b: u8, c: u8) -> u8 {
        let aa = a as i32;
        let bb = b as i32;
        let cc = c as i32;

        let p = aa + bb - cc;
        let pa = aa.abs_diff(p);
        let pb = bb.abs_diff(p);
        let pc = cc.abs_diff(p);

        if pa <= pb && pa <= pc { a } else if pb <= pc { b } else { c }
    }

    fn apply_filter_none(x: u8, l: u8, u: u8, ul: u8) -> u8 { x }
    fn apply_filter_sub(x: u8, l: u8, u: u8, ul: u8) -> u8 { x.wrapping_add(l) }
    fn apply_filter_up(x: u8, l: u8, u: u8, ul: u8) -> u8 { x.wrapping_add(u) }
    fn apply_filter_average(x: u8, l: u8, u: u8, ul: u8) -> u8 { x.wrapping_add(min(l, u) + l.abs_diff(u) / 2) }
    fn apply_filter_paeth(x: u8, l: u8, u: u8, ul: u8) -> u8 { x.wrapping_add(PNG::paeth(l, u, ul)) }


    /// Applies a filter to a single scanline.
    fn apply_filter_scanlines(current_scanline: &[u8], prior_scanline: &[u8]) -> Vec<u8> {
        let filter_type = FilterType::try_from(current_scanline[0]).expect("Invalid filter type");

        // Select filter function f(x, l, u, ul) -> y
        let filter_func = match filter_type {
            FilterType::None => { PNG::apply_filter_none }
            FilterType::Sub => { PNG::apply_filter_sub }
            FilterType::Up => { PNG::apply_filter_up }
            FilterType::Average => { PNG::apply_filter_average }
            FilterType::Paeth => { PNG::apply_filter_paeth }
        };

        // Apply filter
        (1..current_scanline.len())
            .map(|i| filter_func(
                current_scanline[i], // x
                if i == 1 { 0 } else { current_scanline[i - 1] }, // l
                prior_scanline[i], // u
                if i == 1 { 0 } else { prior_scanline[i - 1] }, // ul
            ))
            .collect()
    }

    /// Applies a filter to all scanlines.
    fn filter(bytes: &[u8], ihdr: &IHDR) -> Vec<u8> {
        let scanline_length = ((ihdr.width * ihdr.color_type.bytes_per_pixel()) + 1) as usize;
        let scanlines = bytes
            .chunks(scanline_length)
            .collect::<Vec<_>>();

        // Add dummy scanline as filters require prior scanline
        let dummy_scanline = vec![0_u8; scanline_length];


        let row_indices = (0..scanlines.len()).collect::<Vec<usize>>();
        let filtered = row_indices
            .iter()
            // .par_iter()
            .map(|&i| {
                let prior_scanline = if i == 0 { &dummy_scanline } else { scanlines[i - 1] };
                PNG::apply_filter_scanlines(scanlines[i], prior_scanline)
            })
            .flatten()
            .collect();


        filtered
    }


    pub fn get_image_data(&self) -> Vec<u8> {
        // Get IDHR chunk
        let ihdr = IHDR::from_chunk(&self.chunks[0]);

        // Filter chunks
        let data_chunks = self.chunks.iter()
            .filter(|c| c.chunk_type == "IDAT")
            .collect::<Vec<&Chunk>>();

        // Concatenate chunk data
        let mut all_chunks: Vec<u8> = vec![];
        for chunk in data_chunks {
            all_chunks.extend(&chunk.data)
        }

        // Decompress
        let mut decompressed = vec![];
        zlib::Decoder::new(all_chunks.as_slice()).read_to_end(&mut decompressed)
            .expect("Failed to decompress byte stream");


        // Filter
        PNG::filter(&decompressed, &ihdr)
    }
}

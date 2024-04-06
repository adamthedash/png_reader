use std::fmt::{Display, Formatter};
use std::io::Read;

use compress::zlib;

use crate::u8_enum;
use crate::png::Chunk;
use crate::utils::{read_be_u16, read_be_u32, read_until_null};

pub trait FromChunk {
    fn from_chunk(chunk: &Chunk) -> Self;
}

#[derive(Debug)]
pub struct IHDR {
    pub width: u32,
    pub height: u32,
    bit_depth: u8,
    pub color_type: ColorType,
    compression_method: u8,
    filter_method: u8,
    interlace_method: InterlaceMethod,
}

impl FromChunk for IHDR {
    fn from_chunk(chunk: &Chunk) -> Self {
        Self {
            width: read_be_u32(&chunk.data[..4]),
            height: read_be_u32(&chunk.data[4..8]),
            bit_depth: chunk.data[8],
            color_type: chunk.data[9].try_into().unwrap(),
            compression_method: chunk.data[10],
            filter_method: chunk.data[11],
            interlace_method: chunk.data[12].try_into().unwrap(),
        }
    }
}

u8_enum! {
    #[derive(Debug)]
    pub enum ColorType {
        Greyscale = 0,
        TrueColor = 2,
        IndexedColor = 3,
        GreyscaleAlpha = 4,
        TrueColorAlpha = 6,
    }
}

impl ColorType {
    pub fn bytes_per_pixel(&self) -> u32 {
        match self {
            ColorType::Greyscale => { 1 }
            ColorType::TrueColor => { 3 }
            ColorType::IndexedColor => { 1 }
            ColorType::GreyscaleAlpha => { 2 }
            ColorType::TrueColorAlpha => { 4 }
        }
    }
}



u8_enum! {
    #[derive(Debug)]
    pub enum InterlaceMethod {
        None = 0,
        Adam7 = 1,
    }
}

#[derive(Debug)]
pub struct sRGB {
    intent: RenderingIntent,
}

impl FromChunk for sRGB {
    fn from_chunk(chunk: &Chunk) -> Self {
        Self {
            intent: chunk.data[0].try_into().unwrap()
        }
    }
}

u8_enum! {
    #[derive(Debug)]
    enum RenderingIntent {
        Perceptual = 0,
        RelativeColorimetric = 1,
        Saturation = 2,
        AbsoluteColorimetric = 3,
    }
}


#[derive(Debug)]
pub struct gAMA {
    gamma: f32,
}

impl FromChunk for gAMA {
    fn from_chunk(chunk: &Chunk) -> Self {
        Self {
            gamma: read_be_u32(&chunk.data) as f32 / 100000.
        }
    }
}


#[derive(Debug)]
pub struct pHYs {
    pixels_per_unit_x: u32,
    pixels_per_unit_y: u32,
    unit_specifier: PixelUnit,
}

impl FromChunk for pHYs {
    fn from_chunk(chunk: &Chunk) -> Self {
        Self {
            pixels_per_unit_x: read_be_u32(&chunk.data[..4]),
            pixels_per_unit_y: read_be_u32(&chunk.data[4..8]),
            unit_specifier: chunk.data[8].try_into().unwrap(),
        }
    }
}



u8_enum! {
    #[derive(Debug)]
    enum PixelUnit {
        Unknown = 0,
        Meter = 1,
    }
}

u8_enum! {
    #[derive(Debug)]
    pub enum FilterType {
        None = 0,
        Sub = 1,
        Up = 2,
        Average = 3,
        Paeth = 4,
    }
}


#[derive(Debug)]
pub enum ByteAlign {
    Intel,
    Motorola,
}

impl TryFrom<(u8, u8)> for ByteAlign {
    type Error = ();

    fn try_from(value: (u8, u8)) -> Result<Self, Self::Error> {
        match value {
            (77, 77) => { Ok(Self::Motorola) }
            (73, 73) => { Ok(Self::Intel) }
            _ => { Err(()) }
        }
    }
}

#[derive(Debug)]
pub struct eXIf {
    byte_align: ByteAlign,
    idfs: Vec<IDF>,
}

#[derive(Debug)]
pub struct IDF {
    entries: Vec<Vec<u8>>,
}

impl FromChunk for eXIf {
    fn from_chunk(chunk: &Chunk) -> Self {
        let mut offset = read_be_u32(&chunk.data[4..8]) as usize;

        let mut idfs = vec![];
        while offset > 0 {
            let idf = IDF::parse_idf(&chunk.data[offset..]);

            offset += 2 + idf.entries.len() * 12; // Offset to next offset
            idfs.push(idf);

            offset = read_be_u32(&chunk.data[offset..offset + 4]) as usize;
        }


        Self {
            byte_align: ByteAlign::try_from((chunk.data[0], chunk.data[1])).unwrap(),
            idfs,
        }
    }
}

impl IDF {
    fn parse_idf(bytes: &[u8]) -> IDF {
        // Consumes a folder off the top of the bytestream
        let num_entries = u16::from_be_bytes([bytes[0], bytes[1]]);

        // Each entry is 12 bytes
        let entries = bytes[2..2 + (num_entries as usize) * 12]
            .chunks(12)
            .map(|e| e.to_vec())
            .collect::<Vec<_>>();

        IDF { entries }
    }
}


#[derive(Debug)]
pub struct cHRM {
    white_x: u32,
    white_y: u32,
    red_x: u32,
    red_y: u32,
    green_x: u32,
    green_y: u32,
    blue_x: u32,
    blue_y: u32,
}

impl FromChunk for cHRM {
    fn from_chunk(chunk: &Chunk) -> Self {
        Self {
            white_x: read_be_u32(&chunk.data[..4]),
            white_y: read_be_u32(&chunk.data[4..8]),
            red_x: read_be_u32(&chunk.data[8..12]),
            red_y: read_be_u32(&chunk.data[12..16]),
            green_x: read_be_u32(&chunk.data[16..20]),
            green_y: read_be_u32(&chunk.data[20..24]),
            blue_x: read_be_u32(&chunk.data[24..28]),
            blue_y: read_be_u32(&chunk.data[28..32]),
        }
    }
}

#[derive(Debug)]
pub struct bKGD_Greyscale {
    value: u16,
}

impl FromChunk for bKGD_Greyscale {
    fn from_chunk(chunk: &Chunk) -> Self {
        Self {
            value: read_be_u16(&chunk.data[..2]),
        }
    }
}

#[derive(Debug)]
pub struct bKGD_TrueColor {
    red: u16,
    green: u16,
    blue: u16,
}

impl FromChunk for bKGD_TrueColor {
    fn from_chunk(chunk: &Chunk) -> Self {
        Self {
            red: read_be_u16(&chunk.data[..2]),
            green: read_be_u16(&chunk.data[2..4]),
            blue: read_be_u16(&chunk.data[4..6]),
        }
    }
}

#[derive(Debug)]
pub struct bKGD_Indexed {
    index: u8,
}

impl FromChunk for bKGD_Indexed {
    fn from_chunk(chunk: &Chunk) -> Self {
        Self {
            index: chunk.data[0],
        }
    }
}

#[derive(Debug)]
pub struct tEXt {
    keyword: String,
    text: String,
}

impl FromChunk for tEXt {
    fn from_chunk(chunk: &Chunk) -> Self {
        // Split on first null byte
        let split = chunk.data.splitn(2, |&x| x == 0)
            .collect::<Vec<_>>();

        Self {
            keyword: String::from_utf8(split[0].to_vec()).unwrap(),
            text: String::from_utf8(split[1].to_vec()).unwrap(),
        }
    }
}


#[derive(Debug)]
pub struct iCCP {
    profile_name: String,
    compression_method: CompressionMethod,
    profile: Vec<u8>,
}


u8_enum! {
    #[derive(Debug)]
    enum CompressionMethod {
        ZLIB = 0,
    }
}

impl FromChunk for iCCP {
    fn from_chunk(chunk: &Chunk) -> Self {
        let name = read_until_null(&chunk.data);
        let name_len = name.len();

        // Decompress todo
        // let mut decompressed = vec![];
        // zlib::Decoder::new(&chunk.data[name_len + 2..])
        //     .read_to_end(&mut decompressed)
        //     .expect("Failed to decompress byte stream");


        Self {
            profile_name: String::from_utf8(name).unwrap(),
            compression_method: chunk.data[name_len + 1].try_into().unwrap(),
            profile: chunk.data[name_len + 2..].to_vec(),
        }
    }
}


#[derive(Debug)]
pub struct tIME {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
}

impl FromChunk for tIME {
    fn from_chunk(chunk: &Chunk) -> Self {
        Self {
            year: read_be_u16(&chunk.data[..2]),
            month: chunk.data[2],
            day: chunk.data[3],
            hour: chunk.data[4],
            minute: chunk.data[5],
            second: chunk.data[6],
        }
    }
}


#[derive(Debug)]
pub struct zTXt {
    keyword: String,
    compression_method: CompressionMethod,
    text: String,
}

impl Display for zTXt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n----------------------------------\n{}", self.keyword, self.text)
    }
}

impl FromChunk for zTXt {
    fn from_chunk(chunk: &Chunk) -> Self {
        let name = read_until_null(&chunk.data);

        let name_len = name.len();

        // Decompress
        let mut decompressed = vec![];
        zlib::Decoder::new(&chunk.data[name_len + 2..])
            .read_to_end(&mut decompressed)
            .expect("Failed to decompress byte stream");

        Self {
            keyword: String::from_utf8(name).unwrap(),
            compression_method: chunk.data[name_len + 1].try_into().unwrap(),
            text: String::from_utf8(decompressed).unwrap(),
        }
    }
}


#[derive(Debug)]
pub struct PLTE {
    palette: Vec<Vec<u8>>,
}

impl FromChunk for PLTE {
    fn from_chunk(chunk: &Chunk) -> Self {
        Self {
            palette: chunk.data.chunks(3)
                .map(|c| c.to_vec())
                .collect()
        }
    }
}

#[derive(Debug)]
pub struct tRNS_Greyscale {
    value: u16,
}

impl FromChunk for tRNS_Greyscale {
    fn from_chunk(chunk: &Chunk) -> Self {
        Self {
            value: read_be_u16(&chunk.data[..2]),
        }
    }
}


#[derive(Debug)]
pub struct tRNS_TrueColor {
    red: u16,
    blue: u16,
    green: u16,
}

impl FromChunk for tRNS_TrueColor {
    fn from_chunk(chunk: &Chunk) -> Self {
        Self {
            red: read_be_u16(&chunk.data[..2]),
            blue: read_be_u16(&chunk.data[2..4]),
            green: read_be_u16(&chunk.data[4..6]),
        }
    }
}


#[derive(Debug)]
pub struct tRNS_Indexed {
    values: Vec<u8>,
}

impl FromChunk for tRNS_Indexed {
    fn from_chunk(chunk: &Chunk) -> Self {
        Self {
            values: chunk.data.to_vec()
        }
    }
}


#[derive(Debug)]
pub struct iTXt {
    keyword: String,
    is_compressed: bool,
    compression_method: CompressionMethod,
    lang_tag: Option<String>,
    translated_keyword: Option<String>,
    text: String,
}

impl FromChunk for iTXt {
    fn from_chunk(chunk: &Chunk) -> Self {
        let keyword = read_until_null(&chunk.data);
        let mut offset = keyword.len() + 1;

        let is_compressed = chunk.data[offset] == 1;
        let compression_method = CompressionMethod::try_from(chunk.data[offset + 1]).unwrap();
        offset += 2;

        // Language tag
        let lang_tag = read_until_null(&chunk.data[offset..]);
        let lang_tag = if lang_tag.is_empty() {
            None
        } else {
            offset += lang_tag.len();
            Some(String::from_utf8(lang_tag).unwrap())
        };
        offset += 1;

        // Translated keyword
        let translated_keyword = match lang_tag {
            None => { None }
            Some(_) => {
                let bytes = read_until_null(&chunk.data[offset..]);
                offset += bytes.len();

                Some(String::from_utf8(bytes).unwrap())
            }
        };


        offset += 1;

        let text = read_until_null(&chunk.data[offset..]);

        Self {
            keyword: String::from_utf8(keyword).unwrap(),
            is_compressed,
            compression_method,
            lang_tag,
            translated_keyword,
            text: String::from_utf8(text).unwrap(),
        }
    }
}
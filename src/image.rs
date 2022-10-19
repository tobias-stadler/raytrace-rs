use std::{
    fs,
    io::{self, Write},
    ops::{Add, Mul},
};

#[allow(non_camel_case_types)]
pub type fCol = f32;

#[derive(Debug)]
pub struct Image {
    width: usize,
    height: usize,
    pixels: Vec<Pixel>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Color {
    pub r: fCol,
    pub g: fCol,
    pub b: fCol,
}

impl Color {
    #[inline]
    pub fn black() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    #[inline]
    pub fn white() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
    }

    #[inline]
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: r as fCol / u8::MAX as fCol,
            g: g as fCol / u8::MAX as fCol,
            b: b as fCol / u8::MAX as fCol,
        }
    }

    #[inline]
    pub fn new(r: fCol, g: fCol, b: fCol) -> Self {
        Self { r: r, g: g, b: b }
    }

    #[inline]
    pub fn gamma2(self) -> Self {
        Self {
            r: self.r.sqrt(),
            g: self.g.sqrt(),
            b: self.b.sqrt(),
        }
    }
}

impl Add for Color {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Mul for Color {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl Mul<fCol> for Color {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: fCol) -> Self::Output {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Pixel {
    fn write_to_buf_bgr(&self, buf: &mut Vec<u8>) {
        buf.push(self.b);
        buf.push(self.g);
        buf.push(self.r);
    }
}

impl From<Color> for Pixel {
    #[inline]
    fn from(col: Color) -> Self {
        Pixel {
            r: (col.r * 255.) as u8,
            g: (col.g * 255.) as u8,
            b: (col.b * 255.) as u8,
        }
    }
}

struct BmpHeader {
    magic: u16,
    size: u32,
    offset: u32,
}

impl BmpHeader {
    pub fn write_to_buf(&self, buf: &mut Vec<u8>) {
        buf.write_all(&self.magic.to_be_bytes()).unwrap();
        buf.write_all(&self.size.to_le_bytes()).unwrap();
        buf.write(&[0; 4]).unwrap();
        buf.write_all(&self.offset.to_le_bytes()).unwrap();
    }
}

struct BmpInfo {
    size: u32,
    width: i32,
    height: i32,
    planes: u16,
    bit_count: u16,
    compression: u32,
    size_image: u32,
    x_dpmeter: i32,
    y_dpmeter: i32,
    clr_used: u32,
    clr_important: u32,
}

impl BmpInfo {
    pub fn new_bgr(width: usize, height: usize, num_pixels: usize) -> BmpInfo {
        BmpInfo {
            size: 40,
            width: width as i32,
            height: -(height as i32),
            planes: 1,
            bit_count: 24,
            compression: 0,
            size_image: (num_pixels as u32) * 3,
            x_dpmeter: 0,
            y_dpmeter: 0,
            clr_used: 0,
            clr_important: 0,
        }
    }

    pub fn write_to_buf(&self, buf: &mut Vec<u8>) {
        buf.write_all(&self.size.to_le_bytes()).unwrap();
        buf.write_all(&self.width.to_le_bytes()).unwrap();
        buf.write_all(&self.height.to_le_bytes()).unwrap();
        buf.write_all(&self.planes.to_le_bytes()).unwrap();
        buf.write_all(&self.bit_count.to_le_bytes()).unwrap();
        buf.write_all(&self.compression.to_le_bytes()).unwrap();
        buf.write_all(&self.size_image.to_le_bytes()).unwrap();
        buf.write_all(&self.x_dpmeter.to_le_bytes()).unwrap();
        buf.write_all(&self.y_dpmeter.to_le_bytes()).unwrap();
        buf.write_all(&self.clr_used.to_le_bytes()).unwrap();
        buf.write_all(&self.clr_important.to_le_bytes()).unwrap();
    }
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        Image {
            width: width,
            height: height,
            pixels: vec![Color::black().into(); width * height],
        }
    }

    #[inline]
    pub fn px_mut(&mut self, x: usize, y: usize) -> Option<&mut Pixel> {
        self.enforce(x, y)?;
        self.pixels.get_mut(y * self.width + x)
    }

    #[inline]
    fn enforce(&self, x: usize, y: usize) -> Option<()> {
        if x < self.width && y < self.height {
            Some(())
        } else {
            None
        }
    }

    #[inline]
    pub fn px(&self, x: usize, y: usize) -> Option<&Pixel> {
        self.enforce(x, y)?;
        self.pixels.get(y * self.width + x)
    }

    pub fn save_bmp(&self, path: &str) -> io::Result<()> {
        let mut file = fs::File::create(path)?;
        let mut out: Vec<u8> = Vec::with_capacity(self.pixels.len() * 3);

        BmpHeader {
            magic: 0x424D,
            size: 0,
            offset: 64,
        }
        .write_to_buf(&mut out);
        BmpInfo::new_bgr(self.width, self.height, self.pixels.len()).write_to_buf(&mut out);

        let offset = out.len();
        let padding = (4 - (3 * self.width) % 4) % 4;
        //println!("Offset: {}", offset);
        for _ in offset..64 {
            out.push(0xCC)
        }
        for (i, px) in self.pixels.iter().enumerate() {
            px.write_to_buf_bgr(&mut out);
            if i % self.width == self.width - 1 {
                for _ in 0..padding {
                    out.push(0);
                }
            }
        }

        file.write_all(&out)?;
        Ok(())
    }
}

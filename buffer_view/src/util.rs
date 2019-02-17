#![allow(dead_code)]

#[derive(Clone, Debug)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u32>,
}

impl std::ops::Index<(usize, usize)> for Image {
    type Output = u32;

    fn index(&self, (x, y): (usize, usize)) -> &u32 {
        assert!(x < self.width);
        assert!(y < self.height);
        &self.data[x + y * self.width]
    }
}
impl std::ops::IndexMut<(usize, usize)> for Image {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut u32 {
        assert!(x < self.width);
        assert!(y < self.height);
        &mut self.data[x + y * self.width]
    }
}

impl Image {
    pub fn dump(&self) -> ! {
        println!("P3");
        println!("{} {}", self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                let v = self[(x, y)];
                let b = v & 0xFF;
                let g = (v >> 8) & 0xFF;
                let r = (v >> 16) & 0xFF;
                print!("{} {} {} ", r, g, b);
            }
            println!("");
        }
        panic!();
    }

    pub fn new(width: usize, height: usize, fill: u32) -> Image {
        let mut data = Vec::with_capacity(width * height);
        data.resize(width * height, fill);
        Image {
            width,
            height,
            data,
        }
    }

    pub fn pad(&self, new_width: usize, new_height: usize, fill: u32) -> Image {
        assert!(new_width >= self.width);
        assert!(new_height >= self.height);
        if new_width == self.width && new_height == self.height {
            return self.clone();
        }

        let mut out = Image::new(new_width, new_height, fill);

        for y in 0..self.height {
            for x in 0..self.width {
                out[(x, y)] = self[(x, y)];
            }
        }

        out
    }

    pub fn stack_hor(&self, other: &Image, fill: u32) -> Image {
        let new_width = self.width + other.width;
        let new_height = std::cmp::max(self.height, other.height);
        let mut out = Image::new(new_width, new_height, fill);

        for y in 0..self.height {
            for x in 0..self.width {
                out[(x, y)] = self[(x, y)];
            }
        }

        for y in 0..other.height {
            for x in 0..other.width {
                out[(x + self.width, y)] = other[(x, y)];
            }
        }

        out
    }

    pub fn stack_vert(&self, other: &Image, fill: u32) -> Image {
        let new_width = std::cmp::max(self.width, other.width);
        let new_height = self.height + other.height;
        let mut out = Image::new(new_width, new_height, fill);

        for y in 0..self.height {
            for x in 0..self.width {
                out[(x, y)] = self[(x, y)];
            }
        }

        for y in 0..other.height {
            for x in 0..other.width {
                out[(x, y + self.height)] = other[(x, y)];
            }
        }

        out
    }

    pub fn pixelize(&self) -> Image {
        let mut out = Image::new(self.width * 2, self.height * 2, 0);

        for y in 0..self.height {
            for x in 0..self.width {
                let v = self[(x, y)];
                out[(x * 2 + 0, y * 2 + 0)] = v;
                out[(x * 2 + 1, y * 2 + 0)] = v;
                out[(x * 2 + 0, y * 2 + 1)] = v;
                out[(x * 2 + 1, y * 2 + 1)] = v;
            }
        }

        out
    }
}

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>, // Cambiado a u32 para representar colores
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let buffer = vec![0; width * height];
        Framebuffer { width, height, buffer }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: u32) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            self.buffer[index] = value;
        }
    }

    pub fn get_buffer(&self) -> &Vec<u32> {
        &self.buffer
    }

    pub fn clear(&mut self) {
        self.buffer.iter_mut().for_each(|pixel| *pixel = 0);
    }
}

use super::colour::Colour;

use ::util::vector::Vector2D;

pub struct Renderer {
    size: Vector2D<u8>


}

impl Renderer {
    pub fn new(x_size: u8, y_size: u8) -> Renderer {
        let mut renderer = Renderer {
            size: Vector2D::new(x_size, y_size)
        };
        renderer.create_border();

        renderer
    }

    fn create_border(&mut self) {
        Renderer::set_bg_colour(&Colour::new(0, 0, 0));
        for x in 0..self.size.x + 2 {
            Renderer::set_cursor_location(x, 0);
            print!(" ");
            Renderer::set_cursor_location(x, self.size.y + 1);
            print!(" ");
        }
        for y in 0..self.size.y + 2 {
            Renderer::set_cursor_location(0, y);
            print!(" ");
            Renderer::set_cursor_location(self.size.x + 1, y);
            print!(" ");
        }
    }

    /**
     * Colour functions for changing text colour in the terminal
     */
    pub fn set_text_colour(colour: &Colour) {
        Renderer::set_colour(38, &colour);
    }
    
    pub fn set_bg_colour(colour: &Colour) {
        Renderer::set_colour(48, &colour);
    }

    pub fn set_colour(ansi: u8, colour: &Colour) {
        print!("\x1b[{};2;{};{};{}m", 
            ansi, colour.r, colour.g, colour.b);
    }

    /**
     * Misc ANSI commands
     */
    pub fn set_cursor_location(x: u8, y: u8) {
        print!("\x1b[{};{}H", y + 1, x + 1);
    }
}
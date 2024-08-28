use std::{collections::HashMap, fs::read, io::Bytes, rc::Rc};

use macroquad::{prelude::ImageFormat, texture::Texture2D};

use crate::model::Texture;


pub struct TextureManager{
    textures: HashMap<Texture, Texture2D>
}
impl TextureManager {
    fn create_default_texture() -> Texture2D {
        let width = 32;
        let height = 32;
        let pixels: Vec<_> = (0..(width*height))
            .flat_map(|i| if (i % width > width/2) != (i > width*height/2) { [255, 0, 255, 255] } else {[255, 255, 255, 255]})
            .collect();

        Texture2D::from_rgba8(width, height, &pixels)
    }

    fn load_texture(textures: &mut HashMap<Texture, Texture2D>, texture: Texture, path: &str) {
        let result = read(path);
        if result.is_err() {
            println!("Error loading file '{}': {}", path, result.err().unwrap());
            return;
        }
        let bytes = result.unwrap();

        textures.insert(texture, Texture2D::from_file_with_format(&bytes, None));
    }

    pub fn load() -> Rc<Self> {
        let mut textures = HashMap::new();
        
        textures.insert(Texture::Debug, Self::create_default_texture());
        Self::load_texture(&mut textures, Texture::Stone, "assets/stone.png");
        Rc::new(Self { textures })
    }
    
    pub fn get_texture(&self, texture: Texture) -> &Texture2D {
        let texture_2d = self.textures.get(&texture);
        if texture_2d.is_some() {
            return texture_2d.unwrap();
        }

        self.textures.get(&Texture::Debug).expect("Debug texture not found")
    }
}
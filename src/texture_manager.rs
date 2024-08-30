use std::{collections::HashMap, fs::read};

use macroquad::texture::{FilterMode, Texture2D};

use crate::model::Texture;

pub struct TextureManager {
    textures: HashMap<Texture, Texture2D>,
}
impl TextureManager {
    fn create_default_texture() -> Texture2D {
        let width = 32;
        let height = 32;
        let pixels: Vec<_> = (0..(width * height))
            .flat_map(|i| {
                if (i % width > width / 2) != (i > width * height / 2) {
                    [255, 0, 255, 255]
                } else {
                    [255, 255, 255, 255]
                }
            })
            .collect();

        Texture2D::from_rgba8(width, height, &pixels)
    }

    fn load_texture(textures: &mut HashMap<Texture, Texture2D>, texture: Texture, path: &str) {
        let result = read(path);
        if result.is_err() {
            println!("Error loading file '{path}': {}", result.err().unwrap());
            return;
        }
        let bytes = result.unwrap();

        let texture_2d = Texture2D::from_file_with_format(&bytes, None);
        texture_2d.set_filter(FilterMode::Linear);
        textures.insert(texture, texture_2d);
    }

    pub fn load() -> Self {
        let mut textures = HashMap::new();

        textures.insert(Texture::Debug, Self::create_default_texture());
        Self::load_texture(&mut textures, Texture::Stone, "assets/stone.png");
        Self::load_texture(&mut textures, Texture::Key1, "assets/key1.png");
        Self::load_texture(&mut textures, Texture::Key2, "assets/key2.png");
        Self { textures }
    }

    pub fn get_texture(&self, texture: Texture) -> &Texture2D {
        let texture_2d = self.textures.get(&texture);
        if let Some(unwraped) = texture_2d {
            return unwraped;
        }

        self.textures
            .get(&Texture::Debug)
            .expect("Debug texture not found")
    }
}

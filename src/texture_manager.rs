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

    fn load_texture_with_filter(
        textures: &mut HashMap<Texture, Texture2D>,
        texture: Texture,
        path: &str,
        filter_mode: FilterMode,
    ) {
        let result = read(path);
        if result.is_err() {
            println!("Error loading file '{path}': {}", result.err().unwrap());
            return;
        }
        let bytes = result.unwrap();

        let texture_2d = Texture2D::from_file_with_format(&bytes, None);
        texture_2d.set_filter(filter_mode);
        textures.insert(texture, texture_2d);
    }

    fn load_texture(textures: &mut HashMap<Texture, Texture2D>, texture: Texture, path: &str) {
        Self::load_texture_with_filter(textures, texture, path, FilterMode::Nearest);
    }

    pub fn load() -> Self {
        let mut textures = HashMap::new();

        textures.insert(Texture::Debug, Self::create_default_texture());
        Self::load_texture_with_filter(
            &mut textures,
            Texture::Stone,
            "assets/stone.png",
            FilterMode::Linear,
        );

        Self::load_texture(&mut textures, Texture::Key1, "assets/key/key1.png");
        Self::load_texture(&mut textures, Texture::Key2, "assets/key/key2.png");
        Self::load_texture(&mut textures, Texture::Skull, "assets/skull.png");

        Self::load_texture(&mut textures, Texture::Gun1, "assets/gun/FAMAS_00.png");
        Self::load_texture(&mut textures, Texture::Gun2, "assets/gun/FAMAS_03.png");
        Self::load_texture(&mut textures, Texture::Gun3, "assets/gun/FAMAS_04.png");
        Self::load_texture(&mut textures, Texture::Gun4, "assets/gun/FAMAS_05.png");
        Self::load_texture(&mut textures, Texture::Gun5, "assets/gun/FAMAS_06.png");
        Self::load_texture(&mut textures, Texture::Gun6, "assets/gun/FAMAS_07.png");
        Self::load_texture(&mut textures, Texture::Gun7, "assets/gun/FAMAS_08.png");
        Self::load_texture(&mut textures, Texture::Gun8, "assets/gun/FAMAS_09.png");

        Self::load_texture(&mut textures, Texture::Enemy1, "assets/enemy/enemy1.png");
        Self::load_texture(&mut textures, Texture::Enemy2, "assets/enemy/enemy2.png");
        Self::load_texture(&mut textures, Texture::Enemy3, "assets/enemy/enemy3.png");
        Self::load_texture(&mut textures, Texture::Enemy4, "assets/enemy/enemy4.png");
        Self::load_texture(&mut textures, Texture::Enemy5, "assets/enemy/enemy5.png");
        Self::load_texture(&mut textures, Texture::Enemy6, "assets/enemy/enemy6.png");
        Self::load_texture(&mut textures, Texture::Enemy7, "assets/enemy/enemy7.png");
        Self::load_texture(&mut textures, Texture::Enemy8, "assets/enemy/enemy8.png");

        Self::load_texture(
            &mut textures,
            Texture::Explostion1,
            "assets/explosion/1.png",
        );
        Self::load_texture(
            &mut textures,
            Texture::Explostion2,
            "assets/explosion/2.png",
        );
        Self::load_texture(
            &mut textures,
            Texture::Explostion3,
            "assets/explosion/3.png",
        );
        Self::load_texture(
            &mut textures,
            Texture::Explostion4,
            "assets/explosion/4.png",
        );
        Self::load_texture(
            &mut textures,
            Texture::Explostion5,
            "assets/explosion/5.png",
        );
        Self::load_texture(
            &mut textures,
            Texture::Explostion6,
            "assets/explosion/6.png",
        );
        Self::load_texture(
            &mut textures,
            Texture::Explostion7,
            "assets/explosion/7.png",
        );
        Self::load_texture(
            &mut textures,
            Texture::Explostion8,
            "assets/explosion/8.png",
        );
        Self::load_texture(
            &mut textures,
            Texture::Explostion9,
            "assets/explosion/9.png",
        );

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

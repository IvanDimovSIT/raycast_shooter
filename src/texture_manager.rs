use std::{collections::HashMap, fs::read};

use macroquad::texture::{FilterMode, Texture2D};

use crate::{constants::TEXTURE_PATH, model::Texture};

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
        let full_path = TEXTURE_PATH.to_string() + path;
        let result = read(full_path);
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
            "stone.png",
            FilterMode::Linear,
        );

        Self::load_texture_with_filter(
            &mut textures,
            Texture::Metal,
            "metal.png",
            FilterMode::Linear,
        );

        Self::load_texture(&mut textures, Texture::Key1, "key/key1.png");
        Self::load_texture(&mut textures, Texture::Key2, "key/key2.png");
        Self::load_texture(&mut textures, Texture::Skull, "skull.png");

        Self::load_texture(&mut textures, Texture::Gun1, "gun/FAMAS_00.png");
        Self::load_texture(&mut textures, Texture::Gun2, "gun/FAMAS_03.png");
        Self::load_texture(&mut textures, Texture::Gun3, "gun/FAMAS_04.png");
        Self::load_texture(&mut textures, Texture::Gun4, "gun/FAMAS_05.png");
        Self::load_texture(&mut textures, Texture::Gun5, "gun/FAMAS_06.png");
        Self::load_texture(&mut textures, Texture::Gun6, "gun/FAMAS_07.png");
        Self::load_texture(&mut textures, Texture::Gun7, "gun/FAMAS_08.png");
        Self::load_texture(&mut textures, Texture::Gun8, "gun/FAMAS_09.png");

        Self::load_texture(&mut textures, Texture::Enemy1, "enemy/melee/enemy1.png");
        Self::load_texture(&mut textures, Texture::Enemy2, "enemy/melee/enemy2.png");
        Self::load_texture(&mut textures, Texture::Enemy3, "enemy/melee/enemy3.png");
        Self::load_texture(&mut textures, Texture::Enemy4, "enemy/melee/enemy4.png");
        Self::load_texture(&mut textures, Texture::Enemy5, "enemy/melee/enemy5.png");
        Self::load_texture(&mut textures, Texture::Enemy6, "enemy/melee/enemy6.png");
        Self::load_texture(&mut textures, Texture::Enemy7, "enemy/melee/enemy7.png");
        Self::load_texture(&mut textures, Texture::Enemy8, "enemy/melee/enemy8.png");

        Self::load_texture(
            &mut textures,
            Texture::RangedEnemy1,
            "enemy/ranged/enemy1.png",
        );

        Self::load_texture(
            &mut textures,
            Texture::RangedEnemy2,
            "enemy/ranged/enemy2.png",
        );

        Self::load_texture(
            &mut textures,
            Texture::RangedEnemy3,
            "enemy/ranged/enemy3.png",
        );

        Self::load_texture(
            &mut textures,
            Texture::RangedEnemy4,
            "enemy/ranged/enemy4.png",
        );

        Self::load_texture(
            &mut textures,
            Texture::RangedEnemy5,
            "enemy/ranged/enemy5.png",
        );

        Self::load_texture(
            &mut textures,
            Texture::RangedEnemy6,
            "enemy/ranged/enemy6.png",
        );

        Self::load_texture(
            &mut textures,
            Texture::RangedEnemy7,
            "enemy/ranged/enemy7.png",
        );

        Self::load_texture(
            &mut textures,
            Texture::RangedEnemy8,
            "enemy/ranged/enemy8.png",
        );

        Self::load_texture(&mut textures, Texture::Explostion1, "explosion/1.png");
        Self::load_texture(&mut textures, Texture::Explostion2, "explosion/2.png");
        Self::load_texture(&mut textures, Texture::Explostion3, "explosion/3.png");
        Self::load_texture(&mut textures, Texture::Explostion4, "explosion/4.png");
        Self::load_texture(&mut textures, Texture::Explostion5, "explosion/5.png");
        Self::load_texture(&mut textures, Texture::Explostion6, "explosion/6.png");
        Self::load_texture(&mut textures, Texture::Explostion7, "explosion/7.png");
        Self::load_texture(&mut textures, Texture::Explostion8, "explosion/8.png");
        Self::load_texture(&mut textures, Texture::Explostion9, "explosion/9.png");

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

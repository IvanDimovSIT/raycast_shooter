use std::{collections::HashMap, fs::read};

use macroquad::texture::{FilterMode, Texture2D};

use crate::{constants::TEXTURE_PATH, model::Texture};

const TEXTURE_PATHS: [(Texture, &str); 48] = [
    (Texture::Stone, "stone.png"),
    (Texture::Metal, "metal.png"),
    (Texture::Key1, "key/key1.png"),
    (Texture::Key2, "key/key1.png"),
    (Texture::Gun1, "gun/FAMAS_00.png"),
    (Texture::Gun2, "gun/FAMAS_03.png"),
    (Texture::Gun3, "gun/FAMAS_04.png"),
    (Texture::Gun4, "gun/FAMAS_05.png"),
    (Texture::Gun5, "gun/FAMAS_06.png"),
    (Texture::Gun6, "gun/FAMAS_07.png"),
    (Texture::Gun7, "gun/FAMAS_08.png"),
    (Texture::Gun8, "gun/FAMAS_09.png"),
    (Texture::Enemy1, "enemy/melee/enemy1.png"),
    (Texture::Enemy2, "enemy/melee/enemy2.png"),
    (Texture::Enemy3, "enemy/melee/enemy3.png"),
    (Texture::Enemy4, "enemy/melee/enemy4.png"),
    (Texture::Enemy5, "enemy/melee/enemy5.png"),
    (Texture::Enemy6, "enemy/melee/enemy6.png"),
    (Texture::Enemy7, "enemy/melee/enemy7.png"),
    (Texture::Enemy8, "enemy/melee/enemy8.png"),
    (Texture::RangedEnemy1, "enemy/ranged/enemy1.png"),
    (Texture::RangedEnemy2, "enemy/ranged/enemy2.png"),
    (Texture::RangedEnemy3, "enemy/ranged/enemy3.png"),
    (Texture::RangedEnemy4, "enemy/ranged/enemy4.png"),
    (Texture::RangedEnemy5, "enemy/ranged/enemy5.png"),
    (Texture::RangedEnemy6, "enemy/ranged/enemy6.png"),
    (Texture::RangedEnemy7, "enemy/ranged/enemy7.png"),
    (Texture::RangedEnemy8, "enemy/ranged/enemy8.png"),
    (Texture::MeleeSlowEnemy1, "enemy/meleeSlow/enemy1.png"),
    (Texture::MeleeSlowEnemy2, "enemy/meleeSlow/enemy2.png"),
    (Texture::MeleeSlowEnemy3, "enemy/meleeSlow/enemy3.png"),
    (Texture::MeleeSlowEnemy4, "enemy/meleeSlow/enemy4.png"),
    (Texture::MeleeSlowEnemy5, "enemy/meleeSlow/enemy5.png"),
    (Texture::MeleeSlowEnemy6, "enemy/meleeSlow/enemy6.png"),
    (Texture::MeleeSlowEnemy7, "enemy/meleeSlow/enemy7.png"),
    (Texture::MeleeSlowEnemy8, "enemy/meleeSlow/enemy8.png"),
    (Texture::Explostion1, "explosion/1.png"),
    (Texture::Explostion2, "explosion/2.png"),
    (Texture::Explostion3, "explosion/3.png"),
    (Texture::Explostion4, "explosion/4.png"),
    (Texture::Explostion5, "explosion/5.png"),
    (Texture::Explostion6, "explosion/6.png"),
    (Texture::Explostion7, "explosion/7.png"),
    (Texture::Explostion8, "explosion/8.png"),
    (Texture::Explostion9, "explosion/9.png"),
    (Texture::Skull, "skull.png"),
    (Texture::Projectile, "projectile.png"),
    (Texture::TextFindTheKeys, "text/find_exit.png"),
];

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

    fn load_multiple_textures(
        textures: &mut HashMap<Texture, Texture2D>,
        filter_mode: FilterMode,
        paths: &[(Texture, &str)],
    ) {
        for (texture, path) in paths {
            Self::load_texture_with_filter(textures, *texture, path, filter_mode);
        }
    }

    pub fn load() -> Self {
        let mut textures = HashMap::new();
        textures.insert(Texture::Debug, Self::create_default_texture());
        Self::load_multiple_textures(&mut textures, FilterMode::Nearest, &TEXTURE_PATHS);

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

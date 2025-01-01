use std::{collections::HashMap, fs::read};

use macroquad::texture::{FilterMode, Texture2D};

use crate::{constants::TEXTURE_PATH, model::TextureId};

const TEXTURE_PATHS: [(TextureId, &str); 49] = [
    (TextureId::Stone, "stone.png"),
    (TextureId::Metal, "metal.png"),
    (TextureId::Door, "door.png"),
    (TextureId::Key1, "key/key1.png"),
    (TextureId::Key2, "key/key2.png"),
    (TextureId::Gun1, "gun/FAMAS_00.png"),
    (TextureId::Gun2, "gun/FAMAS_03.png"),
    (TextureId::Gun3, "gun/FAMAS_04.png"),
    (TextureId::Gun4, "gun/FAMAS_05.png"),
    (TextureId::Gun5, "gun/FAMAS_06.png"),
    (TextureId::Gun6, "gun/FAMAS_07.png"),
    (TextureId::Gun7, "gun/FAMAS_08.png"),
    (TextureId::Gun8, "gun/FAMAS_09.png"),
    (TextureId::Enemy1, "enemy/melee/enemy1.png"),
    (TextureId::Enemy2, "enemy/melee/enemy2.png"),
    (TextureId::Enemy3, "enemy/melee/enemy3.png"),
    (TextureId::Enemy4, "enemy/melee/enemy4.png"),
    (TextureId::Enemy5, "enemy/melee/enemy5.png"),
    (TextureId::Enemy6, "enemy/melee/enemy6.png"),
    (TextureId::Enemy7, "enemy/melee/enemy7.png"),
    (TextureId::Enemy8, "enemy/melee/enemy8.png"),
    (TextureId::RangedEnemy1, "enemy/ranged/enemy1.png"),
    (TextureId::RangedEnemy2, "enemy/ranged/enemy2.png"),
    (TextureId::RangedEnemy3, "enemy/ranged/enemy3.png"),
    (TextureId::RangedEnemy4, "enemy/ranged/enemy4.png"),
    (TextureId::RangedEnemy5, "enemy/ranged/enemy5.png"),
    (TextureId::RangedEnemy6, "enemy/ranged/enemy6.png"),
    (TextureId::RangedEnemy7, "enemy/ranged/enemy7.png"),
    (TextureId::RangedEnemy8, "enemy/ranged/enemy8.png"),
    (TextureId::MeleeSlowEnemy1, "enemy/meleeSlow/enemy1.png"),
    (TextureId::MeleeSlowEnemy2, "enemy/meleeSlow/enemy2.png"),
    (TextureId::MeleeSlowEnemy3, "enemy/meleeSlow/enemy3.png"),
    (TextureId::MeleeSlowEnemy4, "enemy/meleeSlow/enemy4.png"),
    (TextureId::MeleeSlowEnemy5, "enemy/meleeSlow/enemy5.png"),
    (TextureId::MeleeSlowEnemy6, "enemy/meleeSlow/enemy6.png"),
    (TextureId::MeleeSlowEnemy7, "enemy/meleeSlow/enemy7.png"),
    (TextureId::MeleeSlowEnemy8, "enemy/meleeSlow/enemy8.png"),
    (TextureId::Explostion1, "explosion/1.png"),
    (TextureId::Explostion2, "explosion/2.png"),
    (TextureId::Explostion3, "explosion/3.png"),
    (TextureId::Explostion4, "explosion/4.png"),
    (TextureId::Explostion5, "explosion/5.png"),
    (TextureId::Explostion6, "explosion/6.png"),
    (TextureId::Explostion7, "explosion/7.png"),
    (TextureId::Explostion8, "explosion/8.png"),
    (TextureId::Explostion9, "explosion/9.png"),
    (TextureId::Skull, "skull.png"),
    (TextureId::Projectile, "projectile.png"),
    (TextureId::TextFindTheKeys, "text/find_exit.png"),
];

pub struct TextureManager {
    textures: HashMap<TextureId, Texture2D>,
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
        textures: &mut HashMap<TextureId, Texture2D>,
        texture: TextureId,
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
        textures: &mut HashMap<TextureId, Texture2D>,
        filter_mode: FilterMode,
        paths: &[(TextureId, &str)],
    ) {
        for (texture, path) in paths {
            Self::load_texture_with_filter(textures, *texture, path, filter_mode);
        }
    }

    pub fn load() -> Self {
        let mut textures = HashMap::new();
        textures.insert(TextureId::Debug, Self::create_default_texture());
        Self::load_multiple_textures(&mut textures, FilterMode::Nearest, &TEXTURE_PATHS);

        Self { textures }
    }

    pub fn get_texture(&self, texture: TextureId) -> &Texture2D {
        let texture_2d = self.textures.get(&texture);
        if let Some(unwraped) = texture_2d {
            return unwraped;
        }

        self.textures
            .get(&TextureId::Debug)
            .expect("Debug texture not found")
    }
}

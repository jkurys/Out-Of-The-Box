/** TEXTURES */
pub const PLAYER_TEXTURES: [&str; 3] = [
    "textures/player.png",
    "textures/white_sprite.png",
    "textures/black_sprite.png",
];
pub const TILE_TEXTURE: &str = "textures/tile.png";
pub const WALL_TEXTURE: &str = "textures/wall.png";
pub const BOX_TEXTURE: &str = "textures/box.png";
pub const GOAL_TEXTURE: &str = "textures/goal.png";
pub const BOX_ON_GOAL_TEXTURE: &str = "textures/box_on_goal.png";
pub const ICE_TEXTURE: &str = "textures/ice.png";
pub const WARP_TEXTURE: &str = "textures/warp.png";
pub const HIDDEN_WALL_TEXTURE: &str = "textures/hidden_wall.png";
pub const SHOWN_HIDDEN_WALL_TEXTURE: &str = "textures/hidden_wall_shown.png";
pub const BUTTON_TEXTURE: &str = "textures/button.png";
pub const PLUS_TEXTURE: &str = "textures/plus.png";
pub const HOVERED_PLUS_TEXTURE: &str = "textures/hovered-plus.png";
/** FONTS */

pub const MAIN_MENU_FONT: &str = "fonts/square-deal.ttf";
pub const LEVEL_FONT: &str = "fonts/pixel NES.otf";

/** MAPS */
// pub const MAP_NAMES: [&str; LEVEL_AMOUNT as usize] = [
//     "assets/maps/1.txt",
//     "assets/maps/2.txt",
//     "assets/maps/3.txt",
//     "assets/maps/4.txt",
//     "assets/maps/5.txt",
//     "assets/maps/6.txt",
//     "assets/maps/7.txt",
//     "assets/maps/8.txt",
//     "assets/maps/9.txt",
// ];

/** SAVES */
pub const PLAYER_TEXTURE_SAVE: &str = "assets/saves/player.txt";
pub const LEVEL_SAVE: &str = "assets/saves/level.txt";
//for sure this can be done in a better way, I just haven't found it yet

/** DIMENSIONS */
pub const TILE_SIZE: f32 = 50.;
pub const IMAGE_SIZE: f32 = 16.;
pub const MOVE_ANIMATION_TIME: f32 = 0.2;
pub const MAX_WIDTH: u32 = 20;
pub const MAX_HEIGHT: u32 = 20;

/** Z_INDICES */
pub const OBJECT_Z_INDEX: f32 = 2.0;
pub const FLOOR_Z_INDEX: f32 = 1.0;

/** MISCELLANEOUS */
// pub const LEVEL_AMOUNT: usize = 9;
pub const MAX_MAPS: usize = 10;
pub const INITIAL_MAP: usize = 0;

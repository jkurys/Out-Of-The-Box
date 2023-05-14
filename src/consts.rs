/** TEXTURES */
pub const PLAYER_TEXTURES: [&str; 3] = [
    "textures/player.png",
    "textures/white_sprite.png",
    "textures/black_sprite.png",
];

pub const LOWER_PLAYER_TEXTURES: [&str; 3] = [
    "textures/player_lower.png",
    "textures/player_lower.png",
    "textures/player_lower.png",
];

pub const TILE_TEXTURE: &str = "textures/tile.png";
pub const LOWER_WALL_TEXTURE: &str = "textures/wall_lower.png";
pub const HIGHER_WALL_TEXTURE: &str = "textures/wall_higher.png";
pub const HIGHER_BOX_TEXTURE: &str = "textures/box.png";
pub const LOWER_BOX_TEXTURE: &str = "textures/box_lower.png";
pub const GOAL_TEXTURE: &str = "textures/goal.png";
pub const HIGHER_BOX_ON_GOAL_TEXTURE: &str = "textures/box_on_goal.png";
pub const ICE_TEXTURE: &str = "textures/ice.png";
pub const WARP_TEXTURE: &str = "textures/warp.png";
pub const HIDDEN_WALL_TEXTURE: &str = "textures/hidden_wall.png";
pub const LOWER_SHOWN_HIDDEN_WALL_TEXTURE: &str = "textures/hidden_wall_lower.png";
pub const HIGHER_SHOWN_HIDDEN_WALL_TEXTURE: &str = "textures/hidden_wall_higher.png";
pub const BUTTON_TEXTURE: &str = "textures/button.png";
pub const PLUS_TEXTURE: &str = "textures/plus.png";
pub const HOVERED_PLUS_TEXTURE: &str = "textures/hovered-plus.png";
/** FONTS */

pub const MAIN_MENU_FONT: &str = "fonts/square-deal.ttf";
pub const LEVEL_FONT: &str = "fonts/pixel NES.otf";

/** SAVES */
pub const PLAYER_TEXTURE_SAVE: &str = "assets/saves/player.txt";
pub const LEVEL_SAVE: &str = "assets/saves/level.txt";

/** DIMENSIONS */
pub const TILE_SIZE: f32 = 50.;
pub const IMAGE_SIZE: f32 = 16.;
pub const MOVE_ANIMATION_TIME: f32 = 0.2;
pub const MAX_WIDTH: u32 = 20;
pub const MAX_HEIGHT: u32 = 20;

/** Z_INDICES */
pub const LOWER_HALF_OBJECT_Z_INDEX: f32 = 1.5;
pub const UPPER_HALF_OBJECT_Z_INDEX: f32 = 3.0;
pub const FLOOR_Z_INDEX: f32 = 1.0;

/** MISCELLANEOUS */
pub const MAX_MAPS: usize = 10;
pub const INITIAL_MAP: usize = 0;

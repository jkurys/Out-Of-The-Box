/** TEXTURES */
pub const PLAYER_TEXTURES: [&str; 3] = [
    "textures/player.png",
    "textures/white_sprite.png",
    "textures/black_sprite.png",
];

pub const TILE_TEXTURE: &str = "textures/tile.png";
pub const WALL_TEXTURE: &str = "textures/wall_higher.png";
pub const WALL_ATLAS: &str = "textures/wall_atlas.png";
pub const TURTLE_TEXTURES: [&str; 4] = [
    "textures/turtle_left.png",
    "textures/turtle_right.png",
    "textures/turtle_down.png",
    "textures/turtle_up.png",
];
pub const STICKER_TEXTURES: [&str; 3] = [
    "textures/turtle_red_sticker.png",
    "textures/turtle_blue_sticker.png",
    "textures/turtle_green_sticker.png",
];
pub const GLUE_ATLAS: &str = "textures/glue_atlas.png";
pub const BOX_GLUE_ATLAS: &str = "textures/box_glue_atlas.png";
pub const TURTLE_ATLAS: &str = "textures/turtle_atlas.png";
pub const PLAYER_ATLAS: &str = "textures/player_atlas.png";
pub const BOX_ATLAS: &str = "textures/box_atlas.png";
pub const BOX_TEXTURE: &str = "textures/box2.png";
pub const GOAL_TEXTURE: &str = "textures/goal.png";
pub const ICE_TEXTURE: &str = "textures/ice.png";
pub const WARP_TEXTURE: &str = "textures/warp.png";
pub const HIDDEN_WALL_TEXTURES: [&str; 3] = [
    "textures/hidden_wall_red.png",
    "textures/hidden_wall_blue.png",
    "textures/hidden_wall_green.png",
];
pub const HIDING_WALL_ATLAS: &str = "textures/hiding_wall_atlas.png";
pub const SHOWN_HIDDEN_WALL_TEXTURES: [&str; 3] = [
    "textures/hidden_wall_higher.png",
    "textures/hidden_wall_higher_blue.png",
    "textures/hidden_wall_higher_green.png",
];
pub const BUTTON_TEXTURES: [&str; 3] = [
    "textures/button_red.png",
    "textures/button_blue.png",
    "textures/button_green.png",
];
pub const PLUS_TEXTURE: &str = "textures/plus.png";
pub const HOVERED_PLUS_TEXTURE: &str = "textures/hovered-plus.png";
pub const BACKGROUND_TEXTURE: &str = "textures/background.png";
/** FONTS */

pub const MAIN_MENU_FONT: &str = "fonts/njal_bold.otf";
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
pub const LOWER_HALF_OBJECT_Z_INDEX: f32 = 2.5;
pub const UPPER_HALF_OBJECT_Z_INDEX: f32 = 3.0;
pub const UPPER_HALF_STICKER_Z_INDEX: f32 = 3.5;
pub const CORNER_STICKER_Z_INDEX: f32 = 4.0;
pub const FLOOR_Z_INDEX: f32 = 1.0;
pub const FLOOR_STICKER_Z_INDEX: f32 = 1.5;

/** MISCELLANEOUS */
pub const MAX_MAPS: usize = 10;
pub const INITIAL_MAP: usize = 0;

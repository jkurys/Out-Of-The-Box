/** TEXTURES */
pub const PLAYER_TEXTURES: [&str; 3] = [
    "textures/player.png",
    "textures/white_sprite.png",
    "textures/black_sprite.png",
];

pub const TILE_TEXTURE: &str = "textures/tile_big.png";
pub const WALL_TEXTURE: &str = "textures/wall_higher.png";
pub const WALL_ATLAS: &str = "textures/wall_atlas_big.png";
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
pub const GLUE_ATLAS: &str = "textures/glue_atlas_big.png";
pub const DIRT_TEXTURE: &str = "textures/dirt.png";
pub const WATER_TEXTURE: &str = "textures/water_big.png";
pub const BOX_GLUE_ATLAS: &str = "textures/box_glue_atlas.png";
pub const TURTLE_ATLAS: &str = "textures/turtle_atlas_big.png";
pub const PLAYER_ATLAS: &str = "textures/player_atlas_big.png";
pub const BOX_ATLAS: &str = "textures/box_atlas_big.png";
pub const TELEBOX_TEXTURE: &str = "textures/tele_box.png";
pub const BOX_TEXTURE: &str = "textures/box2.png";
pub const GOAL_TEXTURE: &str = "textures/goal_big.png";
pub const ICE_TEXTURE: &str = "textures/ice_big.png";
pub const HIDDEN_WALL_TEXTURES: [&str; 3] = [
    "textures/hidden_wall_red.png",
    "textures/hidden_wall_blue.png",
    "textures/hidden_wall_green.png",
];
pub const HIGHLIGHTS_TEXTURE: &str = "textures/highlight.png";
pub const BUTTON_PRESS_TEXTURE: &str = "textures/button_press.png";
pub const POWERUP_TEXTURE: &str = "textures/powerup_big.png";
pub const HIDING_WALL_ATLAS: &str = "textures/hiding_wall_atlas_big.png";
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
pub const BUTTON_ATLAS: &str = "textures/button_atlas_big.png";
pub const BACKGROUND_TEXTURE: &str = "textures/background.png";
/** FONTS */

pub const MAIN_MENU_FONT: &str = "fonts/njal_bold.otf";
pub const LEVEL_FONT: &str = "fonts/pixel NES.otf";

/** SAVES */
pub const PLAYER_TEXTURE_SAVE: &str = "assets/saves/player.txt";
pub const LEVEL_SAVE: &str = "assets/saves/level.txt";

/** DIMENSIONS */
pub const TILE_WIDTH: f32 = 50.;
pub const TILE_HEIGHT: f32 = TILE_WIDTH / IMAGE_WIDTH * IMAGE_HEIGHT;
pub const SIDE_HEIGHT: f32 = TILE_HEIGHT + TILE_FRONT_HEIGHT;
pub const SIDE_WIDTH: f32 = IMAGE_SIDE_WIDTH * IMAGE_MULTIPLIER;
pub const IMAGE_WIDTH: f32 = 300.;
pub const IMAGE_HEIGHT: f32 = 282.;
pub const IMAGE_FRONT_HEIGHT: f32 = 200.;
pub const IMAGE_SIDE_WIDTH: f32 = 104.;
pub const TILE_FRONT_HEIGHT: f32 = IMAGE_FRONT_HEIGHT * IMAGE_MULTIPLIER;
pub const IMAGE_MULTIPLIER: f32 = TILE_WIDTH / IMAGE_WIDTH;
pub const MOVE_ANIMATION_TIME: f32 = 0.15;
pub const MAX_WIDTH: u32 = 20;
pub const MAX_HEIGHT: u32 = 20;

/** Z_INDICES */
pub const LOWER_HALF_OBJECT_Z_INDEX: f32 = 2.5;
pub const UPPER_HALF_OBJECT_Z_INDEX: f32 = 3.0;
pub const UPPER_HALF_STICKER_Z_INDEX: f32 = 3.5;
// pub const CORNER_STICKER_Z_INDEX: f32 = 4.0;
// pub const FLOOR_Z_INDEX: f32 = 1.0;
// pub const FLOOR_STICKER_Z_INDEX: f32 = 1.5;

/** MISCELLANEOUS */
// pub const MAX_MAPS: usize = 10;
// pub const INITIAL_MAP: usize = 0;
pub const EAT_COUNTER: usize = 5;

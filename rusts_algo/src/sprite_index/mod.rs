

pub fn exec(cur_frame:u32, duration_frame:u32, num_sprites:u32) -> u32 {
    let cur_sprite_idx = (cur_frame * num_sprites / duration_frame) % num_sprites;
    let num_sprites = num_sprites - 1;
    println!("{cur_sprite_idx}/{num_sprites} @ frame ({cur_frame}/{duration_frame})");
    cur_sprite_idx
}

pub fn run() {
    exec(0, 40, 4);
    exec(8, 40, 4);
    exec(12, 40, 4);
    exec(23, 40, 4);
    exec(39, 40, 4);
    exec(40, 40, 4);
}

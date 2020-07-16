#![feature(link_args)]

fn main() {
    #[link(name="SDL")]
    #[link(name="SDL_ttf")]
    #[link(name="asound")]
    extern "C" {}
}
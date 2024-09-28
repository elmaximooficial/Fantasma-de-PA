use wasm4 as w4;

pub struct MyRuntime {
    framebuffer: w4::draw::Framebuffer
}

impl w4::rt::Runtime for MyRuntime {
    fn start(resources: wasm4::rt::Resources) -> Self {
        return MyRuntime {
            framebuffer: resources.framebuffer
        }
    }

    fn update(&mut self) {
        w4::include_sprites!(
            const PALETTE: _ = common_palette!(0xa64902, 0x000000);

            const SHIT: _ = include_sprite!(r"res\sprites\Sprite-0001.png");
        );

        self.framebuffer.replace_palette(PALETTE);
        self.framebuffer.blit(&SHIT, [10; 2], <_>::default());
    }
}

w4::main! { MyRuntime }
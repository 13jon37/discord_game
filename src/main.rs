use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::{Point, Rect};
use sdl2::image::{self, LoadTexture, InitFlag};
use sdl2::keyboard::Keycode;
use std::time::Duration;

const PLAYER_SPEED: i32 = 20;

struct Player { 
    position: Point,
    sprite: Rect,
    speed: i32,
    direction: Direction,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Player {
    fn new(position: Point, sprite: Rect, direction: Direction)-> Player {
        Player {
            position: position,
            sprite: sprite,
            speed: 0,
            direction: direction,
        }
    }

    fn render(&self, canvas: &mut WindowCanvas, texture: &Texture) -> Result<(), String> {
        let (width, height) = canvas.output_size()?;

        // Treat the center of the screen as the (0, 0) coordinate
        let screen_position = self.position + Point::new(width as i32 / 2, height as i32 / 2);
        let screen_rect = Rect::from_center(screen_position, self.sprite.width(), self.sprite.height());

        canvas.copy(texture, self.sprite, screen_rect)?;

        Ok(())
    }

    fn input(&mut self, event: &Event) {
        match event {
            Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } => {
                self.speed = PLAYER_SPEED;
                self.direction = Direction::Left;
            },
            Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => {
                self.speed = PLAYER_SPEED;
                self.direction = Direction::Right;
            },
            Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, .. } => {
                self.speed = PLAYER_SPEED;
                self.direction = Direction::Up;
            },
            Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, .. } => {
                self.speed = PLAYER_SPEED;
                self.direction = Direction::Down;
            },
            Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } |
            Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } |
            Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. } |
            Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } => {
                self.speed = 0;
            },
            _ => {} // ignore the other cases we're not handling manually
        }
    }

    fn update(&mut self) {
        use self::Direction::*;
        match self.direction {
            Left => {
                self.position = self.position.offset(-self.speed, 0);
            },
            Right => {
                self.position = self.position.offset(self.speed, 0);
            },
            Up => {
                self.position = self.position.offset(0, -self.speed);
            },
            Down => {
                self.position = self.position.offset(0, self.speed);
            },
        }
    }
}

fn render(canvas: &mut WindowCanvas, player: &Player, texture: &Texture) -> Result<(), String> {
    canvas.clear();
    canvas.set_draw_color(Color::RGB(255, 5, 255));

    player.render(canvas, texture)?;

    canvas.present();

    Ok(())
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem.window("Discord Game", 800, 600)
        .position_centered()
        .build()
        .expect("Could not init video subsytem.");

    let mut canvas = window.into_canvas().build()
        .expect("Could not make a canvas.");

    let mut event_pump = sdl_context.event_pump()?;

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("assets/bardo.png")?;

    let mut player = Player::new(Point::new(0, 0),
                                 Rect::new(0, 0, 26, 36),
                                 Direction::Down);

    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running;
                },
                _ => {}
            }
            player.input(&event);
        }
        // Update
        player.update();

        // Render
        render(&mut canvas, &player, &texture)?;

        // Time management
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

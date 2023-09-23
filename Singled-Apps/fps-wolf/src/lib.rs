#![no_std]

use core::f32::consts::{FRAC_PI_2, PI};
use core::{arch::wasm32, panic::PanicInfo}; // PIE in the house

use libm::{ceilf, cosf, fabsf, floorf, sinf, sqrtf, tanf}; // f cuz float

/*
A pointer to the current state of the first gamepad
runtime will update this section of memory
with the state of our gamepad (keyboard) on each frame.
 */
const DRAW_COLORS: *mut u16 = 0x14 as *mut u16;
const GAMEPAD1: *const u8 = 0x16 as *const u8;

// Describe the bits in the gamepad which describe each button.
const BUTTON_LEFT: u8 = 16; // 0b00010000
const BUTTON_RIGHT: u8 = 32; // 0b00100000
const BUTTON_UP: u8 = 64; // 0b01000000
const BUTTON_DOWN: u8 = 128; // 0b10000000

/*
In this case, [u16; HEIGHT] can represent a map
with a width of 16 cells and an arbitrary height of our choosing.
Using Rust’s integer literal syntax we can represent our map
pretty simply by writing a 1 where there is a wall
and a 0 where there is no wall
 */
const MAP: [u16; 8] = [
    0b1111111111111111,
    0b1000001010000101,
    0b1011100000110101,
    0b1000111010010001,
    0b1010001011110111,
    0b1011101001100001,
    0b1000100000001101,
    0b1111111111111111,
];

/*
One thing to note about point_in_wall is
that the Y axis is “flipped” meaning y=0y=0 is at the top.

This is not only faster but reflects
the coordinate system software most commonly uses.
 */
fn point_n_wall(x: f32, y: f32) -> bool {
    match MAP.get(y as usize) {
        Some(line) => (line & (0b1 << x as usize)) != 0,
        None => true,
    }
}

struct State {
    // The player's state.
    player_x: f32,
    player_y: f32,
    player_angle: f32,
}
static mut STATE: State = State {
    // Making the player state static.
    player_x: 1.5,
    player_y: 1.5,
    player_angle: 0.0,
};

const FOV: f32 = PI / 2.7; // The player's field of view.
const HALF_FOV: f32 = FOV * 0.5; // Half the player's field of view.
const ANGLE_STEP: f32 = FOV / 160.0; // The angle between each ray.
const WALL_HEIGHT: f32 = 100.0; // A magic number.

const STEP_SIZE: f32 = 0.045; // How far the player moves each frame.

impl State {
    // For Character Movement
    pub fn update(&mut self, up: bool, down: bool, left: bool, right: bool) {
        // store current position so that we know where we are not lol
        let previous_position = (self.player_x, self.player_y);
        /*
        we modify the player’s x and y positions
        based on the cosf and sinf values
        of the player’s angle multiplied by a constant STEP_SIZE.
         */
        if up {
            self.player_x += cosf(self.player_angle) * STEP_SIZE;
            self.player_y += -sinf(self.player_angle) * STEP_SIZE; // -ve cuz Y-axis flip
        }
        if down {
            self.player_x -= cosf(self.player_angle) * STEP_SIZE;
            self.player_y -= -sinf(self.player_angle) * STEP_SIZE;
        }
        if right {
            self.player_angle -= STEP_SIZE;
        }
        if left {
            self.player_angle += STEP_SIZE;
        }

        // if moving put us into a wall just revert it
        if point_n_wall(self.player_x, self.player_y) {
            (self.player_x, self.player_y) = previous_position;
        }
    }
}

fn distance(a: f32, b: f32) -> f32 {
    // Use D = sqrt(x^2 + y^2)
    sqrtf((a * a) + (b * b))
}

impl State {
    /// Returns the nearest wall the ray intersects with on a horizontal grid line.
    fn horizontal_intersection(&self, angle: f32) -> f32 {
        // This tells you if the angle is "facing up"
        // regardless of how big the angle is.
        let up = fabsf(floorf(angle / PI) % 2.0) != 0.0;

        // first_y and first_x are the first grid intersections
        // that the ray intersects with.
        let first_y = if up {
            ceilf(self.player_y) - self.player_y
        } else {
            floorf(self.player_y) - self.player_y
        };
        let first_x = -first_y / tanf(angle);

        // dy and dx are the "ray extension" values mentioned earlier.
        let dy = if up { 1.0 } else { -1.0 };
        let dx = -dy / tanf(angle);

        // next_x and next_y are mutable values which will keep track
        // of how far away the ray is from the player.
        let mut next_x = first_x;
        let mut next_y = first_y;

        // This is the loop where the ray is extended until it hits
        // the wall. It's not an infinite loop as implied in the
        // explanation, instead it only goes from 0 to 256.
        //
        // This was chosen because if something goes wrong and the
        // ray never hits a wall (which should never happen) the
        // loop will eventually break and the game will keep on running.
        for _ in 0..256 {
            // current_x and current_y are where the ray is currently
            // on the map, while next_x and next_y are relative
            // coordinates, current_x and current_y are absolute
            // points.
            let current_x = next_x + self.player_x;
            let current_y = if up {
                next_y + self.player_y
            } else {
                next_y + self.player_y - 1.0
            };

            // Tell the loop to quit if we've just hit a wall.
            if point_n_wall(current_x, current_y) {
                break;
            }

            // if we didn't hit a wall on this extension add
            // dx and dy to our current position and keep going.
            next_x += dx;
            next_y += dy;
        }

        // return the distance from next_x and next_y to the player.
        distance(next_x, next_y)
    }
}

impl State {
    /// Returns the nearest wall the ray intersects with on a vertical grid line.
    fn vertical_intersection(&self, angle: f32) -> f32 {
        // This tells you if the angle is "facing up"
        // regardless of how big the angle is.
        let right = fabsf(floorf((angle - FRAC_PI_2) / PI) % 2.0) != 0.0;

        // first_y and first_x are the first grid intersections
        // that the ray intersects with.
        let first_x = if right {
            ceilf(self.player_x) - self.player_x
        } else {
            floorf(self.player_x) - self.player_x
        };
        let first_y = -tanf(angle) * first_x;

        // dy and dx are the "ray extension" values mentioned earlier.
        let dx = if right { 1.0 } else { -1.0 };
        let dy = dx * -tanf(angle); // Quik maths

        // next_x and next_y are mutable values which will keep track
        // of how far away the ray is from the player.
        let mut next_x = first_x;
        let mut next_y = first_y;

        // This is the loop where the ray is extended until it hits
        // the wall. It's not an infinite loop as implied in the
        // explanation, instead it only goes from 0 to 256.
        //
        // This was chosen because if something goes wrong and the
        // ray never hits a wall (which should never happen) the
        // loop will eventually quit and the game will keep on running.
        for _ in 0..256 {
            // current_x and current_y are where the ray is currently
            // on the map, while next_x and next_y are relative
            // coordinates, current_x and current_y are absolute
            // points.
            let current_x = if right {
                next_x + self.player_x
            } else {
                next_x + self.player_x - 1.0
            };
            let current_y = next_y + self.player_y;

            // Tell the loop to quit if we've just hit a wall.
            if point_n_wall(current_x, current_y) {
                break;
            }

            // if we didn't hit a wall on this extension add
            // dx and dy to our current position and keep going.
            next_x += dx;
            next_y += dy;
        }

        // return the distance from next_x and next_y to the player.
        distance(next_x, next_y)
    }
}

impl State {
    /// Returns 160 wall heights and their "color" from the player's perspective.
    pub fn get_view(&self) -> [(i32, bool); 160] {
        // The player's FOV is split in half by their viewing angle.
        // In order to get the ray's starting angle we must
        // add half the FOV to the player's angle to get
        // the edge of the player's FOV.
        let starting_angle = self.player_angle + HALF_FOV;

        let mut walls = [(0, false); 160];

        for (idx, wall) in walls.iter_mut().enumerate() {
            // `idx` is what number ray we are, `wall` is
            // a mutable reference to a value in `walls`.
            let angle = starting_angle - idx as f32 * ANGLE_STEP;

            // Get both the closest horizontal and vertical wall
            // intersections for this angle.
            let h_dist = self.horizontal_intersection(angle);
            let v_dist = self.vertical_intersection(angle);

            let (min_dist, shadow) = if h_dist < v_dist {
                (h_dist, false)
            } else {
                (v_dist, true)
            };

            // Get the minimum of the two distances and
            // "convert" it into a wall height.
            *wall = (
                (WALL_HEIGHT / (min_dist * cosf(angle - self.player_angle))) as i32,
                shadow,
            );
        }

        walls
    }
}

extern "C" {
    // Import the `env` module from Wasm.

    fn vline(x: i32, y: i32, len: u32);
    /*
    vline draws a vertical line on the window at x, y
    and extends it down len pixels.
    */
}

/*
A little bit of boilerplate that Rust requires
we provide if we choose to use #![no_std]
It will run when the program panics.
 */
#[panic_handler]
fn phandler(_: &PanicInfo<'_>) -> ! {
    wasm32::unreachable();
}

#[no_mangle]
/*
The main entry point into our program,
WASM-4 calls this function on each frame.
 */
unsafe fn update() {
    STATE.update(
        *GAMEPAD1 & BUTTON_UP != 0,
        *GAMEPAD1 & BUTTON_DOWN != 0,
        *GAMEPAD1 & BUTTON_LEFT != 0,
        *GAMEPAD1 & BUTTON_RIGHT != 0,
    );

    // Go through each column on screen and draw walls in the center.
    for (x, wall) in STATE.get_view().iter().enumerate() {
        let (height, shadow) = wall;

        if *shadow {
            // draw with color 2 for walls with "shadow"
            *DRAW_COLORS = 0x2;
        } else {
            // draw with color 3 for walls without "shadow"
            *DRAW_COLORS = 0x3;
        }

        vline(x as i32, 80 - (height / 2), *height as u32);
    }
}

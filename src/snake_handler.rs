use pc_keyboard::KeyCode::W;
use vga_buffer::WRITER;
use crate::{Color, vga_buffer, interrupts};
use crate::interrupts::Direction;

const BUFFER_HEIGHT: u8 = 25;
const BUFFER_WIDTH: u8 = 80;


struct Position {
    x: u8,
    y: u8,
}
pub fn game_loop()
{
    let mut length = 4;
    let mut x = 3;
    let mut y =  0;
    let mut next_x = 0;
    let mut next_y =  0;
    let mut last_apple_position: Position = spawn_apple();
    let mut snake_body: [Position; 300] = [ Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200},Position {x: 200, y: 200},Position {x: 200, y: 200},Position {x: 200, y: 200},Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200},Position {x: 200, y: 200},Position {x: 200, y: 200},Position {x: 200, y: 200},Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200}, Position {x: 200, y: 200},Position {x: 200, y: 200},Position {x: 200, y: 200},Position {x: 200, y: 200},Position {x: 200, y: 200},Position {x: 200, y: 200},Position {x: 200, y: 200},Position {x: 200, y: 200},Position {x: 200, y: 200},Position {x: 200, y: 200},Position {x: 200, y: 200},Position {x: 200, y: 200},Position {x: 0, y: 0},Position {x: 1, y: 0},Position {x: 2, y: 0},];
    let mut next_index_to_update: usize = 296;
    loop
    {
        //check for interrups (like keyboard input or timer ticks
        hlt();
        //clear our whole screen with spaces
        WRITER.lock().clear_buffer();

        //update the position
        match interrupts::SNAKE.lock().direction {
            Direction::Right => {
                match x+1 {
                    BUFFER_WIDTH => die(),
                    _ => next_x=x+1
                }
            },
            Direction::Left => {
                match x as i16-1 {
                    -1 => die() ,
                    _ => next_x=x-1
                }
            },
            Direction::Up => {
                match y as i16-1 {
                    -1 => die(),
                    _ => next_y=y-1
                }
            },
            Direction::Down => {
                match y+1 {
                    BUFFER_HEIGHT => die(),
                    _ => next_y=y+1
                }
            },
        }

        //update all of the positions of the snakes body
        for i in 0..snake_body.len()
        {
            if snake_body[i].x != 200 && snake_body[i].y != 200
            {
                if i == snake_body.len()-1
                {
                    snake_body[i] = Position {
                        x,
                        y,
                    };
                }
                else {
                    snake_body[i] = Position {
                        x:snake_body[i+1].x,
                        y:snake_body[i+1].y,
                    };
                }

                //if the head hits one of the snakes bodies it's GAME OVER
                if next_x == snake_body[i].x && next_y == snake_body[i].y
                {
                    die();
                }
                //render all of the snakes body
                WRITER.lock().fill_block(Color::LightGreen, Color::LightGreen, snake_body[i].x, snake_body[i].y);
            }
        }
        //render the apple and the snakes head and the length.
        x = next_x;
        y = next_y;
        WRITER.lock().fill_block(Color::Red, Color::Red, last_apple_position.x, last_apple_position.y);
        WRITER.lock().fill_block(Color::Green, Color::Green, x, y);
        crate::print!("{}", length);


        //if we hit an apple we should spawn a new one and add a new body part. (it'll follow automatically with our structure)
        //so the position doesn't matter
        if x == last_apple_position.x && y == last_apple_position.y {
            last_apple_position = spawn_apple();
            snake_body[next_index_to_update] = Position {
                x: 0,
                y: 0,
            };
            next_index_to_update-=1;
            length+=1;
        }


        wait();
    }
}

pub fn hlt() {
    x86_64::instructions::hlt();
}

pub fn wait()
{
    for _i in 0..300000 {}
}

fn spawn_apple() -> Position
{
    let count = interrupts::COUNT.lock().count;
    let mut x = 0;
    let mut y = 0;
    if count != 0
    {
        x = (count%BUFFER_WIDTH as u128) as u8;
        y = (count%BUFFER_HEIGHT as u128) as u8;
    }
    else {
        x = 32;
        y = (count % 3) as u8+12;
    }


    WRITER.lock().fill_block(Color::Red, Color::Red, x, y);

    return Position {
        x,
        y,
    };
}

fn die()
{
    /*
    cause a page fault error which won't be handled which will cause a double fault
    which won't be handled which will cause a triple fault which will reset the cpu.
    exactly the goal :D
     */
    unsafe {
        *(0x80  as *mut i32) = 88;
    }
}
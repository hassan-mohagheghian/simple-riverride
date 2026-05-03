use core::time;
use std::{
    io::{Stdout, Write, stdout},
    thread,
    time::Duration,
};

use rand::prelude::*;

use crossterm::{
    ExecutableCommand, QueueableCommand,
    cursor::{Hide, MoveTo, Show},
    event::{poll, read},
    style::Print,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size},
};

struct Enemy {
    l: u16,
    c: u16,
}

struct World {
    player_c: u16,
    player_l: u16,
    maxc: u16,
    maxl: u16,
    map: Vec<(u16, u16)>,
    died: bool,
    next_start: u16,
    next_end: u16,
    enemy: Vec<Enemy>,
}

impl World {
    fn new(maxc: u16, maxl: u16) -> Self {
        return World {
            player_c: maxc / 2,
            player_l: maxl - 1,
            maxc: maxc,
            maxl: maxl,
            map: vec![((maxc / 2) - 5, (maxc / 2) + 5); maxl as usize],
            died: false,
            next_start: maxc / 2 - 10,
            next_end: maxc / 2 + 10,
            enemy: vec![],
        };
    }
}

fn physic(mut world: World) -> std::io::Result<World> {
    if world.player_c < world.map[world.player_l as usize].0
        || world.player_c + 1 >= world.map[world.player_l as usize].1
    {
        world.died = true
    }

    for l in (0..world.map.len() - 1).rev() {
        world.map[l + 1] = world.map[l];
    }
    if world.next_end < world.map[0].1 {
        world.map[0].1 -= 1;
    }
    if world.next_end > world.map[0].1 {
        world.map[0].1 += 1;
    }

    if world.next_start < world.map[0].0 {
        world.map[0].0 -= 1;
    }
    if world.next_start > world.map[0].0 {
        world.map[0].0 += 1;
    }

    let mut rng = rand::rng();
    if rng.random_range(0..10) > 7 {
        if world.next_start == world.map[0].0 && world.next_end == world.map[0].1 {
            world.next_start = rng.random_range((world.map[0].0 - 5)..(world.map[0].1 - 5));
            world.next_end = rng.random_range((world.map[0].0 + 5)..(world.map[0].1 + 5));
            if world.next_end - world.next_start <= 4 {
                world.next_start -= 4;
            }
        }
    }
    Ok(world)
}

fn draw(mut sc: &Stdout, world: &World) -> std::io::Result<()> {
    sc.queue(Clear(ClearType::All))?;

    // draw the map

    for l in 0..world.map.len() {
        sc.queue(MoveTo(0, l as u16))?;
        sc.queue(Print("+".repeat(world.map[l].0 as usize)))?;
        sc.queue(MoveTo(world.map[l].1, l as u16))?;
        sc.queue(Print("+".repeat((world.maxc - world.map[l].1) as usize)))?;
    }
    sc.queue(MoveTo(world.player_c, world.player_l))?;
    sc.queue(Print("🚢"))?;
    sc.flush()?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    // init the game
    let mut sc = stdout();
    enable_raw_mode()?;
    let (maxc, maxl) = size().unwrap();
    let mut world = World::new(maxc, maxl);

    sc.execute(Hide)?;
    while !world.died {
        if poll(Duration::from_millis(10))? {
            let key = read().unwrap();
            while poll(Duration::from_millis(10)).unwrap() {
                let _ = read();
            }
            match key {
                crossterm::event::Event::Key(event) => match event.code {
                    crossterm::event::KeyCode::Char('q') => break,
                    crossterm::event::KeyCode::Char('w') => {
                        if world.player_l >= 1 {
                            world.player_l -= 1;
                        }
                    }
                    crossterm::event::KeyCode::Char('s') => {
                        if world.player_l < maxl - 1 {
                            world.player_l += 1;
                        }
                    }
                    crossterm::event::KeyCode::Char('a') => {
                        if world.player_c > 1 {
                            world.player_c -= 1;
                        }
                    }
                    crossterm::event::KeyCode::Char('d') => {
                        if world.player_l < maxc - 1 {
                            world.player_c += 1;
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        } else {
        }

        world = physic(world).unwrap();

        draw(&sc, &world)?;
        thread::sleep(time::Duration::from_millis(100));
    }

    sc.execute(Show)?;
    disable_raw_mode()?;
    sc.queue(Clear(ClearType::All))?;
    sc.execute(Print("Thanks for playing"))?;
    Ok(())
}

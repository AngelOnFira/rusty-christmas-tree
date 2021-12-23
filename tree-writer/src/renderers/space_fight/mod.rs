use std::cmp::min;
use super::{Pixel, TreeCanvas};

use std::sync::{Arc, Mutex};
use rand::Rng;

const X_MAX: usize = 19;
const Y_MAX: usize = 75;

const MAX_SHIPS:usize = 10;
const SHIP_SHOOT_TIMER: i8 = 10;
const BULLET_SPEED: i8 = 2;
const EXPLOSION_SIZE_MAX: i8 = 4;

type Point = (i8, i8);


// Name: Space Fight
// Description: Space Fight!
// Author: Eve (@ayyEve)


// because we need access to a list, we can store our state in a lazy static constant
lazy_static::lazy_static! {
    // stores the state
    static ref THINGY:Arc<Mutex<SpaceFight>> = Arc::new(Mutex::new(SpaceFight::new()));
}


struct SpaceFight {
    ships: Vec<Ship>,
    bullets: Vec<Bullet>,
    explosions: Vec<Explosion>,
    stars: Vec<SpaceStar>
}
impl SpaceFight {
    fn new() -> Self {
        let ships = vec![
            Ship::new(10, 73, true), // friend C:
            Ship::new(11, 0, false) // not friend >:C
        ];

        Self {
            ships,
            bullets: Vec::new(),
            explosions: Vec::new(),
            stars: Vec::new(),
        }
    }
    fn update(&mut self, tick: u64) {
        
        // update stars
        if tick % 2 == 0 {
            self.stars.push(SpaceStar::new());
        }
        for star in self.stars.iter_mut() {
            star.update(tick)
        }
        self.stars.retain(|star| star.y < 75 && star.y >= 0);



        // add more ships every 5 ticks if we need to
        if tick % 5 == 0 && self.ships.len() < MAX_SHIPS {
            let mut frien_count = 0;
            for s in self.ships.iter() {
                if s.going_up {frien_count += 1}
            }

            // if not enough friens, add frien C:
            if frien_count < self.ships.len() / 2 {
                self.ships.push(Ship::new(
                    rand::thread_rng().gen_range(0..20),
                    73,
                    true
                ));
            } else {
                // too many friens, need enemies >:C
                self.ships.push(Ship::new(
                    rand::thread_rng().gen_range(0..20),
                    0,
                    false
                ));
            }
        }


        let mut bullet_remove_indices = Vec::new();
        let mut ship_remove_indices = Vec::new();
        let mut explosion_remove_indices = Vec::new();
        let mut explosions = std::mem::take(&mut self.explosions);


        // update ships
        let mut ships = std::mem::take(&mut self.ships);
        for i in ships.iter_mut() {
            i.update(tick, self)
        }

        let ships_cloned = ships.clone();
        for (i, ship) in ships.iter_mut().enumerate() {
            if ship_remove_indices.contains(&i) {continue}

            if ship.y < 0 || ship.y > 75 {
                ship_remove_indices.push(i);
                continue;
            }
            // ship.update(tick, self);

            // check collision with other ships
            let ship_bounds = ship.bounds();
            let mut collision = false;
            for (oi, os) in ships_cloned.iter().enumerate() {
                if i == oi {continue}
                if os.going_up == ship.going_up {continue}
                if ship_remove_indices.contains(&oi) {continue}
                
                //println!("ship1 {} ship2 {}", i, oi);
                if let Some(bang) = check_intersects(&ship_bounds, &os.bounds()) {
                    ship_remove_indices.push(i);
                    ship_remove_indices.push(oi);
                    explosions.push(Explosion::new(bang.0, bang.1));

                    collision = true;
                    break;
                }
            }
        }

        // remove bad ships
        ship_remove_indices.sort();
        for ind in ship_remove_indices.iter().rev() {
            ships.remove(*ind);
        }
        // reset our ships to the new list
        self.ships = ships;


        // update bullets
        let mut bullets = std::mem::take(&mut self.bullets);
        for i in bullets.iter_mut() {
            i.update(tick, self)
        }

        let bullets_cloned = bullets.clone();
        for (i, bullet) in bullets.iter_mut().enumerate() {
            if bullet_remove_indices.contains(&i) {continue}

            // out of bounds, remove
            if bullet.y < 0 || bullet.y > 75 {
                bullet_remove_indices.push(i);
                continue;
            }
            // bullet.update(tick, self);

            
            // check collision with other bullets
            let bullet_bounds = bullet.bounds();
            let mut collision = false;
            for (oi, ob) in bullets_cloned.iter().enumerate() {
                if oi == i {continue}
                if ob.going_up == bullet.going_up {continue}
                if bullet_remove_indices.contains(&oi) {continue}

                // collision
                //println!("bullet1 {} bullet2 {}", i, oi);
                if let Some(bang) = check_intersects(&bullet_bounds, &ob.bounds()) {
                    bullet_remove_indices.push(i);
                    bullet_remove_indices.push(oi);
                    explosions.push(Explosion::new(bang.0, bang.1));

                    collision = true;
                    break;
                }
            }
            if collision {continue}

            // check collision with ships
            for (si, ship) in self.ships.iter().enumerate() {
                // ignore if they're on the same side
                if ship_remove_indices.contains(&si) {continue}
                if ship.going_up == bullet.going_up {continue}

                // collision
                //println!("bullet {} ship {}", i, si);
                if let Some(bang) = check_intersects(&bullet_bounds, &ship.bounds()) {
                    bullet_remove_indices.push(i);
                    ship_remove_indices.push(si);
                    explosions.push(Explosion::new(bang.0, bang.1));

                    collision = true;
                    break;
                }
            }
            if collision {continue}
        }
        

        // remove bad bullets
        bullet_remove_indices.sort();
        for ind in bullet_remove_indices.iter().rev() {
            // i dont know whats causing this
            if bullets.len() == 0 || *ind >= bullets.len() {continue}
            // //println!("removing at {}", ind);
            bullets.remove(*ind);
        }

        // reset our bullets to the new list
        self.bullets = bullets;


        // update explosions
        for (i, explosion) in explosions.iter_mut().enumerate() {
            explosion.update(tick, self);
            if explosion.size == 0 {
                explosion_remove_indices.push(i);
            }
        }


        // remove bad explosions
        explosion_remove_indices.sort();
        for ind in explosion_remove_indices.iter().rev() {
            explosions.remove(*ind);
        }
        
        self.explosions = explosions;
    }

    fn draw(&self) -> Vec<(i8, i8, Pixel)> {
        let mut pixel_list = Vec::new();

        // draw stars
        for star in self.stars.iter() {
            star.draw(&mut pixel_list);
        }
    
        // draw ships
        for ship in self.ships.iter() {
            ship.draw(&mut pixel_list);
        }
    
        // draw bullets
        for bullet in self.bullets.iter() {
            bullet.draw(&mut pixel_list);
        }
    
        // draw explosions
        for explosion in self.explosions.iter() {
            explosion.draw(&mut pixel_list);
        }

        pixel_list
    }
}


pub fn draw(tick: u64) -> TreeCanvas {
    let mut canvas = TreeCanvas::new();

    // lock the thingy so we can access it
    let mut lock = THINGY.lock().unwrap();
    lock.update(tick);

    // clear the screen
    for y in 0..75 {
        for x in 0..20 {
            let this_pixel = Pixel {
                r: 0,
                g: 0,
                b: 0,
            };
            canvas.set_pixel(x, y, this_pixel)
        }
    }

    let pixel_list = lock.draw();

    // //println!("=======================");
    // purge any bad pixels from the list 
    for (x, y, p) in pixel_list.iter() {
        //println!("{},{}", x, y);
        if !(*x >= 0 && *x < 20 && *y >= 0 && *y < 75) {continue}
        canvas.set_pixel(*x as usize, (74 - *y) as usize, *p);
    }
    
    canvas
}


/// pixel star
#[derive(Clone)]
pub struct SpaceStar {
    x: i8,
    y: i8,
    // color: (u8, u8, u8),
}
impl SpaceStar {
    fn new() -> Self {
        let x = rand::thread_rng().gen_range(0..21);
        // println!("adding star");
        Self {
            x,
            y: 0
        }
    }
    fn update(&mut self, tick: u64) {
        self.y += 5;
    }

    fn draw(&self, canvas: &mut Vec<(i8, i8, Pixel)>) {
        canvas.push((self.x, self.y, Pixel {
            r: 255,
            g: 255,
            b: 255,
        }));
    }
}


#[derive(Clone)]
pub struct Ship {
    x: i8,
    y: i8,
    // color: (u8, u8, u8),
    
    shoot_timer: i8,
    going_up: bool,
}
impl Ship {
    fn new(x: i8, y: i8, going_up: bool) -> Self {
        Self {
            x, y, 
            going_up,
            shoot_timer: 0
        }
    }

    fn bounds(&self) -> Vec<Point> {
        vec![
            // left
            (self.x - 1, self.y),
            // right
            (self.x + 1, self.y),

            // top or bottom middle
            (self.x, self.y + if self.going_up {-1} else {1})
        ]
    }

    fn update(&mut self, tick: u64, game: &mut SpaceFight) {
        if self.going_up {
            self.y -= 1
        } else {
            self.y += 1
        };
        

        // check shoot timer
        self.shoot_timer += 1;
        if self.shoot_timer == SHIP_SHOOT_TIMER {
            self.shoot_timer = 0;
            game.bullets.push(Bullet::new(self.x, self.y, self.going_up));
        }

        // move left and right?
    }

    fn draw(&self, canvas: &mut Vec<(i8, i8, Pixel)>) {
        // get color
        let pixel = if self.going_up {
            // color is green, ally
            Pixel {r: 0, g: 255, b: 0}
        } else {
            // color is red, enemy
            Pixel {r: 255, g: 0, b: 0}
        };

        for (x, y) in self.bounds() {
            canvas.push((x, y, pixel))
        }
    }
}


#[derive(Clone)]
pub struct Bullet {
    x: i8,
    y: i8,
    // color: (u8, u8, u8),
    
    going_up: bool,
}
impl Bullet {
    fn new(x: i8, y: i8, going_up: bool) -> Self {
        Self {
            x, y, going_up
        }
    }

    fn bounds(&self) -> Vec<Point> {
        vec![
            // middle
            (self.x, self.y),
            // top or bottom
            (self.x, self.y + if self.going_up {-1} else {1})
        ]
    }

    fn update(&mut self, tick: u64, game: &mut SpaceFight) {
        if self.going_up {
            self.y -= BULLET_SPEED
        } else {
            self.y += BULLET_SPEED
        };
    }

    fn draw(&self, canvas: &mut Vec<(i8, i8, Pixel)>) {
        // color is green, ally
        let pixel = Pixel {
            r: 255,
            g: 255,
            b: 0,
        };

        for (x, y) in self.bounds() {
            canvas.push((x, y, pixel.clone()));
        }
    }
}

#[derive(Clone)]
pub struct Explosion {
    x: i8,
    y: i8,
    size: i8,
    // color: (u8, u8, u8),

    growing: bool
}
impl Explosion {
    fn new(x: i8, y: i8) -> Self {
        Self {
            x, y,
            size: 1,
            growing: true,
        }
    }

    fn update(&mut self, tick: u64, game: &mut SpaceFight) {
        if self.growing {
            self.size += 1;
        } else {
            self.size -= 1;
        }

        if self.size >= EXPLOSION_SIZE_MAX {
            self.growing = false
        }
    }

    fn draw(&self, canvas: &mut Vec<(i8, i8, Pixel)>) {
        // color is green, ally
        let pixel = Pixel {
            r: 255,
            g: 10,
            b: 10,
        };
        
        canvas.push((self.x, self.y, pixel.clone()));
        for i in 0..self.size {
            // top
            canvas.push((self.x, self.y - i, pixel.clone()));
            // bottom
            canvas.push((self.x, self.y + i, pixel.clone()));
            // left
            canvas.push((self.x - i, self.y, pixel.clone()));
            // right
            canvas.push((self.x + i, self.y, pixel.clone()));
            //TODO: fill in corners?
        }
    }
}


fn check_intersects(l1: &Vec<Point>, l2: &Vec<Point>) -> Option<Point> {
    for &p1 in l1 {
        for &p2 in l2 {
            // //println!("checking :{:?} {:?}", p1, p2);
            if p1 == p2 {
                // //println!("collision");
                return Some(p1)
            }
        }
    }

    None
}
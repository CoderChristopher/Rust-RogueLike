extern crate ncurses;

use ncurses::*;
use rand::Rng;

enum GlobalState {
    Done,
    InGame,
}
enum GlobalStateMod {
    Quit,
    None,
}
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}
enum TileType {
    Wall,
    Ground,
    ClosedDoor,
    OpenedDoor,
}
enum ActorType {
    Player,
    Robot,
    //Projectile,
    Passive,
}
enum ActionType {
    Move(Direction, i32),
    Open(Direction),
    Stand,
    None,
}
enum Alignment {
    Good,
    Neutral,
    Evil,
}
enum CollisionType {
    Actor,
    World,
}
enum Collision {
    Collision(usize, CollisionType),
    NoCollision,
}
impl Copy for Direction {}
impl Copy for Alignment {}
impl Copy for ActionType {}
impl Copy for ActorType {}
impl Clone for Direction {
    fn clone(&self) -> Direction {
        *self
    }
}
impl Clone for Alignment {
    fn clone(&self) -> Alignment {
        *self
    }
}
impl Clone for ActionType {
    fn clone(&self) -> ActionType {
        *self
    }
}
impl Clone for ActorType {
    fn clone(&self) -> ActorType {
        *self
    }
}

struct WorldTile {
    tile: TileType,
    x: i32,
    y: i32,
}
struct Actor {
    character: u32,
    x: i32,
    y: i32,
    kind: ActorType,
    action: ActionType,
    alignment: Alignment,
    initutive: u8,
    moveability: u8,
    health: u8,
    attack: u8,
    defense: u8,
}

impl Actor {
    fn draw(&self) {
        mvaddch(self.y, self.x, self.character);
    }
    fn undo_action(&mut self) {
        match &self.action {
            ActionType::Move(direction, distance) => {
                if *distance < 1 {
                    return;
                }
                match direction {
                    Direction::Up => {
                        self.y += *distance;
                    }
                    Direction::Down => {
                        self.y -= *distance;
                    }
                    Direction::Left => {
                        self.x += *distance;
                    }
                    Direction::Right => {
                        self.x -= *distance;
                    }
                    Direction::None => {}
                }
            }
	    ActionType::Open(_) => {
	    }
            ActionType::Stand => {}
            ActionType::None => {}
        }
    }
}
impl Copy for Actor {}
impl Clone for Actor {
    fn clone(&self) -> Actor {
        *self
    }
}

fn check_collision_list(x: i32, y: i32, actors: &Vec<Actor>, world: &Vec<WorldTile>) -> Collision {
    for (ind, actor) in actors.iter().enumerate() {
        if actor.x == x && actor.y == y {
            return Collision::Collision(ind, CollisionType::Actor);
        }
    }
    for (ind, tile) in world.iter().enumerate() {
        if (tile.x) == x && (tile.y == y) && world_tile_is_solid(&tile.tile) {
            return Collision::Collision(ind, CollisionType::World);
        }
    }
    Collision::NoCollision
}

fn check_world_tile_list(x: i32, y: i32, world: &Vec<WorldTile> ) -> Collision {
    for (ind, tile) in world.iter().enumerate() {
        if (tile.x) == x && (tile.y == y)  {
            return Collision::Collision(ind, CollisionType::World);
        }
    }
    Collision::NoCollision
}

fn world_tile_is_solid(world_tile_type: &TileType) -> bool {
	match world_tile_type {
		TileType::Wall => {
			return true;
		}
		TileType::ClosedDoor => {
			return true;
		}
		TileType::OpenedDoor => {
			return false;
		}
		TileType::Ground => {
			return false;
		}
	}
	false
}

fn world_tile_toggle_door(world_tile: &mut WorldTile) -> (bool,&str){
	match world_tile.tile {
		TileType::Wall | TileType::Ground => {
			return (false, "Cannot open tile of this type!");
		}
		TileType::OpenedDoor => {
			world_tile.tile = TileType::ClosedDoor;
			return (true, "Closed a door.");
		}
		TileType::ClosedDoor => {
			world_tile.tile = TileType::OpenedDoor;
			return (true, "Opened a door.");
		}
	}
}


fn decide_action(index: usize, actors: &mut Vec<Actor>) -> GlobalStateMod {
    actors[index].action = ActionType::None;
    match actors[index].kind {
        ActorType::Player => {
            let first_action = getch();
            match first_action {
                113 => {
                    return GlobalStateMod::Quit;
                }
                104 => {
                    actors[index].action = ActionType::Move(Direction::Left, 1);
                }
                106 => {
                    actors[index].action = ActionType::Move(Direction::Down, 1);
                }
                107 => {
                    actors[index].action = ActionType::Move(Direction::Up, 1);
                }
                108 => {
                    actors[index].action = ActionType::Move(Direction::Right, 1);
                }
                111 => {
                    let secondary_action = getch();
                    match secondary_action {
                        104 => {
                            actors[index].action = ActionType::Open(Direction::Left);
                        }
                        106 => {
                            actors[index].action = ActionType::Open(Direction::Down);
                        }
                        107 => {
                            actors[index].action = ActionType::Open(Direction::Up);
                        }
                        108 => {
                            actors[index].action = ActionType::Open(Direction::Right);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        ActorType::Robot => {
            let mut player_direction = Direction::None;
            for act in actors.iter() {
                match act.kind {
                    ActorType::Player => {
                        if act.x > actors[index].x {
                            player_direction = Direction::Right;
                        } else if act.x < actors[index].x {
                            player_direction = Direction::Left;
                        } else if act.y > actors[index].y {
                            player_direction = Direction::Down;
                        } else if act.y < actors[index].y {
                            player_direction = Direction::Up;
                        } else {
                            player_direction = Direction::None;
                        }
                        break;
                    }
                    ActorType::Robot => {
                        player_direction = Direction::None;
                    }
                    ActorType::Passive => {
                        player_direction = Direction::None;
                    }
                }
            }
            actors[index].action = ActionType::Move(player_direction, 1);
        }
        ActorType::Passive => {}
    }

    GlobalStateMod::None
}
fn try_action(index: usize, actors: &mut Vec<Actor>, world: &mut Vec<WorldTile>,debug_messages: &mut Vec<String>) -> Collision {
    match actors[index].action {
        ActionType::Move(direction, distance) => match direction {
            Direction::Up => {
                match check_collision_list(
                    actors[index].x,
                    actors[index].y - distance,
                    actors,
                    world,
                ) {
                    Collision::NoCollision => {
                        actors[index].y -= distance;
                    }
                    Collision::Collision(index, ctype) => {
                        return Collision::Collision(index, ctype);
                    }
                }
            }
            Direction::Down => {
                match check_collision_list(
                    actors[index].x,
                    actors[index].y + distance,
                    actors,
                    world,
                ) {
                    Collision::NoCollision => {
                        actors[index].y += distance;
                    }
                    Collision::Collision(index, ctype) => {
                        return Collision::Collision(index, ctype);
                    }
                }
            }
            Direction::Left => {
                match check_collision_list(
                    actors[index].x - distance,
                    actors[index].y,
                    actors,
                    world,
                ) {
                    Collision::NoCollision => {
                        actors[index].x -= distance;
                    }
                    Collision::Collision(index, ctype) => {
                        return Collision::Collision(index, ctype);
                    }
                }
            }
            Direction::Right => {
                match check_collision_list(
                    actors[index].x + distance,
                    actors[index].y,
                    actors,
                    world,
                ) {
                    Collision::NoCollision => {
                        actors[index].x += distance;
                    }
                    Collision::Collision(index, ctype) => {
                        return Collision::Collision(index, ctype);
                    }
                }
            }
            Direction::None => {}
        },
	ActionType::Open(direction) => {
		match direction {
			Direction::Left => {
				match check_world_tile_list(actors[index].x - 1,actors[index].y,world) {
					Collision::NoCollision => {
						return Collision::NoCollision;
					}
					Collision::Collision(index, ctype) => {
						debug_messages.push(world_tile_toggle_door(&mut world[index]).1.to_string());
					}
				}
			}
			Direction::Right => {
				match check_world_tile_list(actors[index].x + 1,actors[index].y,world) {
					Collision::NoCollision => {
						return Collision::NoCollision;
					}
					Collision::Collision(index, ctype) => {
						debug_messages.push(world_tile_toggle_door(&mut world[index]).1.to_string());
					}
				}
			}
			Direction::Up => {
				match check_world_tile_list(actors[index].x ,actors[index].y - 1,world) {
					Collision::NoCollision => {
						return Collision::NoCollision;
					}
					Collision::Collision(index, ctype) => {
						debug_messages.push(world_tile_toggle_door(&mut world[index]).1.to_string());
					}
				}
			}
			Direction::Down => {
				match check_world_tile_list(actors[index].x ,actors[index].y + 1,world) {
					Collision::NoCollision => {
						return Collision::NoCollision;
					}
					Collision::Collision(index, ctype) => {
						debug_messages.push(world_tile_toggle_door(&mut world[index]).1.to_string());
					}
				}
			}
			Direction::None => {}
		}
	}
        ActionType::Stand => {}
        ActionType::None => {}
    }
    Collision::NoCollision
}

fn draw_world(world: &Vec<WorldTile>, actors: &Vec<Actor>, debug_messages: &Vec<String>) {
    erase();

    for tile in world.iter() {
        match tile.tile {
            TileType::Wall => {
                mvaddch(tile.y, tile.x, 35);
            }
            TileType::Ground => {
                mvaddch(tile.y, tile.x, 45);
            }
            TileType::ClosedDoor => {
                mvaddch(tile.y, tile.x, 37);
            }
            TileType::OpenedDoor => {
                mvaddch(tile.y, tile.x, 95);
            }
        }
    }

    for i in actors.iter() {
        i.draw();
    }
    for (ind, i) in debug_messages.iter().enumerate() {
        mvaddstr(0 + (ind as i32), 18, i);
    }
    mv(10, 0);
    refresh();
}

fn actions(
    world: &mut Vec<WorldTile>,
    actors: &mut Vec<Actor>,
    collision_list: &mut Vec<(usize, usize, CollisionType)>,
    game_state: &mut GlobalState,debug_messages: &mut Vec<String>
) {
    for ind1 in 0..actors.len() {
        match decide_action(ind1, actors) {
            GlobalStateMod::Quit => {
                *game_state = GlobalState::Done;
                break;
            }
            _ => {}
        }
        match try_action(ind1, actors, world, debug_messages) {
            Collision::Collision(ind2, ctype) => {
                collision_list.push((ind1, ind2, ctype));
            }
            Collision::NoCollision => {}
        }
    }
}

fn update_world(
    world: &mut Vec<WorldTile>,
    actors: &mut Vec<Actor>,
    collision_list: &mut Vec<(usize, usize, CollisionType)>,
    debug_messages: &mut Vec<String>,
) {
    loop {
        let mut clean = true;
        for ind1 in 0..actors.len() {
            match check_collision_list(actors[ind1].x, actors[ind1].y, &actors, &world) {
                Collision::Collision(ind2, ctype) => {
                    if ind1 == ind2 {
                        continue;
                    }
                    match ctype {
                        CollisionType::Actor => {
                            if actors[ind1].moveability > actors[ind2].moveability {
                                actors[ind1].undo_action();
                            } else if actors[ind1].moveability < actors[ind2].moveability {
                                actors[ind2].undo_action();
                            } else {
                                actors[ind1].undo_action();
                                actors[ind2].undo_action();
                            }
                            clean = false;
                        }
                        _ => {}
                    }
                }
                Collision::NoCollision => {}
            }
        }
        if clean {
            break;
        }
    }

    while !collision_list.is_empty() {
        match collision_list[0].2 {
            CollisionType::Actor => {
                if actors[collision_list[0].0].health == 0
                    || actors[collision_list[0].1].health == 0
                {
                    collision_list.remove(0);
                    continue;
                }
                if actors[collision_list[0].0].health > 0 {
                    actors[collision_list[0].1].health -= actors[collision_list[0].0].attack;
                }
                debug_messages.push(format!(
                    "Collision between World {} and {}. New Health:{} and {}",
                    collision_list[0].0,
                    collision_list[0].1,
                    actors[collision_list[0].0].health,
                    actors[collision_list[0].1].health
                ));
                collision_list.remove(0);
            }
            CollisionType::World => {
                debug_messages.push(format!(
                    "Collision between Actors {} and {}.",
                    collision_list[0].0, collision_list[0].1
                ));
                collision_list.remove(0);
            }
        }
    }

    loop {
        let mut clean = true;

        for i in 0..actors.len() {
            if actors[i].health == 0 {
                actors.remove(i);
                clean = false;
                break;
            }
        }

        if clean {
            break;
        }
    }

    while debug_messages.len() > 5 {
        debug_messages.remove(0);
    }
}

fn main() {
    let mut game_state = GlobalState::InGame;

    initscr();
    noecho();

    let mut world: Vec<WorldTile> = Vec::new();
    let mut actors: Vec<Actor> = Vec::new();
    let mut debug_messages: Vec<String> = Vec::new();

    actors.push(Actor {
        character: 64,
        x: 5,
        y: 5,
        kind: ActorType::Player,
        action: ActionType::None,
        alignment: Alignment::Good,
        initutive: 127,
        moveability: 0,
        health: 5,
        attack: 1,
        defense: 0,
    });
    /*    actors.push(Actor {
        character: 71,
        x: 6,
        y: 5,
        kind: ActorType::Robot,
        action: ActionType::None,
        alignment: Alignment::Evil,
        initutive: 127,
        moveability: 1,
        health: 5,
        attack: 1,
        defense: 0,
    });
    actors.push(Actor {
        character: 71,
        x: 2,
        y: 2,
        kind: ActorType::Robot,
        action: ActionType::None,
        alignment: Alignment::Evil,
        initutive: 128,
        moveability: 1,
        health: 5,
        attack: 1,
        defense: 0,
    });

    */

    for i in 0..160 {
        if (i % 16) == 0 {
            world.push({
                WorldTile {
                    tile: TileType::Wall,
                    x: (i as i32) % 16,
                    y: (i as i32) / 16,
                }
            });
        } else if (i + 1) % 16 == 0 {
            world.push({
                WorldTile {
                    tile: TileType::Wall,
                    x: (i as i32) % 16,
                    y: (i as i32) / 16,
                }
            });
        } else if i < 17 {
            world.push({
                WorldTile {
                    tile: TileType::Wall,
                    x: (i as i32) % 16,
                    y: (i as i32) / 16,
                }
            });
        } else if i > 144 {
            world.push({
                WorldTile {
                    tile: TileType::Wall,
                    x: (i as i32) % 16,
                    y: (i as i32) / 16,
                }
            });
        } else if i % 16 == 8 {
            if i / 16 != 5 {
                world.push({
                    WorldTile {
                        tile: TileType::Wall,
                        x: (i as i32) % 16,
                        y: (i as i32) / 16,
                    }
                });
            } else {
                world.push({
                    WorldTile {
                        tile: TileType::ClosedDoor,
                        x: (i as i32) % 16,
                        y: (i as i32) / 16,
                    }
                });
            }
        } else {
            world.push({
                WorldTile {
                    tile: TileType::Ground,
                    x: (i as i32) % 16,
                    y: (i as i32) / 16,
                }
            });
        }
    }

    loop {
        draw_world(&mut world, &mut actors, &mut debug_messages);

        let mut collision_list: Vec<(usize, usize, CollisionType)> = Vec::new();

        actions(
            &mut world,
            &mut actors,
            &mut collision_list,
            &mut game_state,
	    &mut debug_messages
        );

        update_world(
            &mut world,
            &mut actors,
            &mut collision_list,
            &mut debug_messages,
        );

        if let GlobalState::Done = game_state {
            break;
        }
    }
    endwin();
}

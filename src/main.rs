//TODO rewrite collision system
extern crate ncurses;

use ncurses::*;
use rand::Rng;

enum GlobalState{
	Done,
	InGame,
}
enum GlobalStateMod {
	Quit,
	None,
}
enum Direction{
	Up,
	Down,
	Left,
	Right,
	None
}
enum ActorType{
	Player,
	Robot,
	//Projectile,
	Passive,
}
enum ActionType{
	Move(Direction,i32),
	Stand,
	None
}
enum Alignment{
	Good,
	Neutral,
	Evil,
}
enum CollisionType{
	Actor,
	World
}
enum Collision{
	Collision(usize,CollisionType),
	NoCollision
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
	solid: bool,
	tile: u32,
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
}

impl Actor {
	fn draw(&self) {
		mvaddch(self.y,self.x,self.character);
	}
	fn decide_action(&mut self,actors: &Vec<Actor>) -> GlobalStateMod {
		self.action=ActionType::None;
		match self.kind {
			ActorType::Player => {
				let ch=getch();
				match ch {
					113 => {
						return GlobalStateMod::Quit;
					},
					104 => {
						self.action=ActionType::Move(Direction::Left,1);
					},
					106 => {
						self.action=ActionType::Move(Direction::Down,1);
					},
					107 => {
						self.action=ActionType::Move(Direction::Up,1);
					},
					108 => {
						self.action=ActionType::Move(Direction::Right,1);
					},
					_ => {
					}
				}
			},
			ActorType::Robot => {
				let mut target_found = false;
				let mut player_direction = Direction::None;
				for act in actors.iter(){
					match act.kind {
						ActorType::Player => {

							if act.x > self.x {
								player_direction = Direction::Right;
							}else if act.x < self.x {
								player_direction = Direction::Left;
							}else if act.y > self.y {
								player_direction = Direction::Down;
							}else if act.y < self.y {
								player_direction = Direction::Up;
							}else{
								player_direction = Direction::None;
							}
							target_found = true;
						},
						ActorType::Robot => {
							player_direction = Direction::None;
						},
						ActorType::Passive => {
							player_direction = Direction::None;
						}
					}
					if target_found {
						break;
					}
				}
				self.action=ActionType::Move(player_direction,1);
			},
			ActorType::Passive => {
			}
		}

		GlobalStateMod::None
	}
	fn try_action(&mut self,actors: &Vec<Actor>,world: &Vec<WorldTile>) -> Collision {
		match &self.action {
			ActionType::Move(direction,distance) => {
				match direction {
					Direction::Up => {
						match check_collision_list(self.x,self.y-distance,actors,world){
							Collision::NoCollision => {
								self.y-=*distance;
							},
							Collision::Collision(index,ctype) => {
								return	Collision::Collision(index,ctype);
							}
						}
					},
					Direction::Down => {
						match check_collision_list(self.x,self.y+distance,actors,world){
							Collision::NoCollision => {
								self.y+=*distance;
							},
							Collision::Collision(index,ctype) => {
								return	Collision::Collision(index,ctype);
							}
						}
					},
					Direction::Left => {
						match check_collision_list(self.x-distance,self.y,actors,world){
							Collision::NoCollision => {
								self.x-=*distance;
							},
							Collision::Collision(index,ctype) => {
								return	Collision::Collision(index,ctype);
							}
						}
					},
					Direction::Right => {
						match check_collision_list(self.x+distance,self.y,actors,world){
							Collision::NoCollision => {
								self.x+=*distance;
							},
							Collision::Collision(index,ctype) => {
								return	Collision::Collision(index,ctype);
							}
						}
					},
					Direction::None =>{
					},
				}
				
			},
			ActionType::Stand => {

			},
			ActionType::None => {
				
			}
		}
		Collision::NoCollision
	}
	fn undo_action(&mut self) {
		match &self.action {
			ActionType::Move(direction,distance) => {
				if *distance< 1 {
					return;
				}
				match direction {
					Direction::Up => {
						self.y+=*distance;
					},
					Direction::Down => {
						self.y-=*distance;
					},
					Direction::Left => {
						self.x+=*distance;
					},
					Direction::Right => {
						self.x-=*distance;
					},
					Direction::None =>{
					},
				}
				
			},
			ActionType::Stand => {

			},
			ActionType::None => {
				
			}
		}
	}
}
impl Copy for Actor {}
impl Clone for Actor {
	fn clone(&self) -> Actor {
		*self
	}
	
}

fn check_collision_list(x: i32, y:i32,actors: &Vec<Actor>,world: &Vec<WorldTile>) -> Collision {
	for (ind,actor) in actors.iter().enumerate() {
		if actor.x==x&&actor.y==y {
			return Collision::Collision(ind,CollisionType::Actor);
		}
	}
	for (ind,tile) in world.iter().enumerate() {
		if ( (ind as i32)%8) == x && ((ind as i32)/8) == y && tile.solid{
			return Collision::Collision(ind,CollisionType::Actor);
		}
	}
	Collision::NoCollision
}

fn main() {
	let mut game_state = GlobalState::InGame;
	initscr();
	noecho();

	let mut world:  Vec<WorldTile> = Vec::new();
	let mut actors: Vec<Actor> = Vec::new();

	actors.push(Actor {
		character:	71,
		x:		2,
		y:		2,
		kind:		ActorType::Robot,
		action:		ActionType::None,
		alignment:	Alignment::Evil,
		initutive:	128,
		moveability:	1,
	});
	actors.push(Actor {
		character:	64,
		x:		5,
		y:		5,
		kind:		ActorType::Player,
		action:		ActionType::None,
		alignment:	Alignment::Good,
		initutive:	127,
		moveability:	0,
	});
	actors.push(Actor {
		character:	71,
		x:		6,
		y:		5,
		kind:		ActorType::Robot,
		action:		ActionType::None,
		alignment:	Alignment::Evil,
		initutive:	127,
		moveability:	1,
	});

	for i in 0..80 {
		if (i%8) == 0 {
			world.push({ WorldTile {
				solid: true,
				tile: 35
			}});
		}else if (i+1)%8 == 0{
			world.push({ WorldTile {
				solid: true,
				tile: 35
			}});
		}else if i<9 {
			world.push({ WorldTile {
				solid: true,
				tile: 35
			}});
		}else if i>70 {
			world.push({ WorldTile {
				solid: true,
				tile: 35
			}});
		}else{
			world.push({ WorldTile {
				solid: false,
				tile: 46
			} } );
		}
	}

	loop {
		erase();

		for (i,tile) in world.iter().enumerate() {
			let j:i32 = i as i32;
			mvaddch(j/8,j%8,tile.tile);
		}

		for i in actors.iter() {
			i.draw();
		}
		mv(10,0);
		refresh();

		//This probably a bad rustism, I should try and figure out a more elegant way to solve this ownership delema.
		let immutable_actors = actors.clone();
		let mut collision_list:Vec<(usize,usize,CollisionType)> = Vec::new();
		
		for (ind1,i) in actors.iter_mut().enumerate() {
			match i.decide_action(&immutable_actors){
				GlobalStateMod::Quit => {
					game_state = GlobalState::Done;
					break;
				},
				_ => {
				}
			}
			match i.try_action(&immutable_actors,&world){
				Collision::Collision(ind2,ctype) => {
					//Implement Code that Runs Through the Collision list and resolves collisions
					collision_list.push((ind1,ind2,ctype));	
				},
				Collision::NoCollision =>{
				}
			}
		}

		if let GlobalState::Done = game_state {
			break;
		}
	}
	endwin();
}

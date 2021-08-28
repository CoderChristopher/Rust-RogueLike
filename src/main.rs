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
						},
						ActorType::Robot => {
							player_direction = Direction::None;
						},
						ActorType::Passive => {
							player_direction = Direction::None;
						}
					}
					
				}
				self.action=ActionType::Move(player_direction,1);
			},
			ActorType::Passive => {
			}
		}

		GlobalStateMod::None
	}
	fn try_action(&mut self) {
		match &self.action {
			ActionType::Move(direction,distance) => {
				if *distance< 1 {
					return;
				}
				match direction {
					Direction::Up => {
						self.y-=*distance;
					},
					Direction::Down => {
						self.y+=*distance;
					},
					Direction::Left => {
						self.x-=*distance;
					},
					Direction::Right => {
						self.x+=*distance;
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
	fn push_around(&mut self,world: &Vec<u32>) {
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
						loop {
							let mut found:bool = false;
							match rand::thread_rng().gen_range(0..4){
								0 => {
									if world[((self.y as usize))*8+((self.x as usize)-1)] != 35 {
										self.x-=*distance;
										found=true;
									}
								},
								1 => {
									if world[((self.y as usize))*8+((self.x as usize)+1)] != 35 {
										self.x+=*distance;
										found=true;
									}
								},
								2 => {
									if world[((self.y as usize)-1)*8+((self.x as usize))] != 35 {
										self.y-=*distance;
										found=true;
									}
								},
								3 => {
									if world[((self.y as usize)+1)*8+((self.x as usize))] != 35 {
										self.y+=*distance;
										found=true;
									}
								},
								_ => {
								},
							}
							if found {
								break;
							}
						}
					},
				}
				
			},
			ActionType::Stand => {
				loop {
					let mut found:bool = false;
					match rand::thread_rng().gen_range(0..4){
						0 => {
							if world[((self.y as usize))*8+((self.x as usize)-1)] != 35 {
								self.x-=1;
								found=true;
							}
						},
						1 => {
							if world[((self.y as usize))*8+((self.x as usize)+1)] != 35 {
								self.x+=1;
								found=true;
							}
						},
						2 => {
							if world[((self.y as usize)-1)*8+((self.x as usize))] != 35 {
								self.y-=1;
								found=true;
							}
						},
						3 => {
							if world[((self.y as usize)+1)*8+((self.x as usize))] != 35 {
								self.y+=1;
								found=true;
							}
						},
						_ => {
						},
					}
					if found {
						break;
					}
				}
			},
			ActionType::None => {
				loop {
					let mut found:bool = false;
					match rand::thread_rng().gen_range(0..4){
						0 => {
							if world[((self.y as usize))*8+((self.x as usize)-1)] != 35 {
								self.x-=1;
								found=true;
							}
						},
						1 => {
							if world[((self.y as usize))*8+((self.x as usize)+1)] != 35 {
								self.x+=1;
								found=true;
							}
						},
						2 => {
							if world[((self.y as usize)-1)*8+((self.x as usize))] != 35 {
								self.y-=1;
								found=true;
							}
						},
						3 => {
							if world[((self.y as usize)+1)*8+((self.x as usize))] != 35 {
								self.y+=1;
								found=true;
							}
						},
						_ => {
						},
					}
					if found {
						break;
					}
				}
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

fn check_collision(act1: Actor,act2: Actor) -> bool {
	if act1.x==act2.x&&act1.y==act2.y {
		return true;
	}
	false
}

fn resolve_collision(indexes: (usize,usize), actor_list: &mut Vec<Actor>,world: &Vec<u32>){
	if actor_list[indexes.0].moveability > actor_list[indexes.1].moveability{
		actor_list[indexes.0].push_around(&world);
	}else if actor_list[indexes.0].moveability < actor_list[indexes.1].moveability{
		actor_list[indexes.1].push_around(&world);
	}else{
		actor_list[indexes.1].push_around(&world);
	}
}

fn main() {
	let mut game_state = GlobalState::InGame;
	initscr();
	noecho();

	let mut world:  Vec<u32> = Vec::new();
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

	for i in 0..80 {
		if (i%8) == 0 {
			world.push(35);
		}else if (i+1)%8 == 0{
			world.push(35);
		}else if i<9 {
			world.push(35);
		}else if i>70 {
			world.push(35);
		}else{
			world.push(46);
		}
	}

	loop {
		erase();

		for (i,tile) in world.iter().enumerate() {
			let j:i32 = i as i32;
			mvaddch(j/8,j%8,*tile);
		}

		for i in actors.iter() {
			i.draw();
		}
		mv(10,0);
		refresh();

		let immutable_actors = actors.clone();
		
		for i in actors.iter_mut() {
			match i.decide_action(&immutable_actors){
				GlobalStateMod::Quit => {
					game_state = GlobalState::Done;
					break;
				},
				_ => {
				}
			}
		}
		for i in actors.iter_mut() {
			i.try_action();	
		}


		loop {
			let mut collision_list:Vec<(usize,usize)> = Vec::new();
			let mut no_collision:bool = true;

			for (ind1,act1) in actors.iter().enumerate() {
				for (ind2,act2) in actors.iter().enumerate() {
					if ind1==ind2 {
						continue;
					}
					let mut exists:bool = false;

					for collision in collision_list.iter(){
						if collision.0 == ind2 && collision.1 == ind1 {
							exists = true;
							break;
						}
					}

					if exists {
						continue;
					}
					if check_collision(*act1,*act2) {
						collision_list.push((ind1,ind2));
						no_collision = false;
					}
					
				}
				
			}
			for act in actors.iter_mut() {
				if world[(act.y as usize)*8+(act.x as usize)] == 35 {
					act.undo_action();
					no_collision = false;
				}
			}
			if no_collision {
				break;
			}

			for collision in collision_list{
				resolve_collision((collision.0,collision.1),&mut actors,&world);
			}
		}


		if let GlobalState::Done = game_state {
			break;
		}
	}
	endwin();
}

use std::collections::HashMap;

const FULL_BOARD: u128 = 0b1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111;
pub const SOLUTION_NUMBER: u32 = 33938944;

#[derive(Clone, Debug)]
pub struct HashPosition {
    pub nb_sub_path: u32,
    pub moves: Vec<u8>,
}

#[derive(Clone, Debug)]
struct Position {
    position: u128,
}

impl Position {
    fn get_board(&self) -> u128 {
        self.position & FULL_BOARD
    }
    fn get_index(&self) -> u8 {
        (self.position >> 100) as u8
    }
    fn get_position(&self) -> u128 {
        self.position
    }
    fn is_grid_full(&self) -> bool {
        self.get_board() == FULL_BOARD
    }
    fn new(board: u128, index: u8) -> Self {
        let index = (index as u128) << 100;
        let position = board | index;
        Self { position }
    }
    fn get_hash(&self) -> u128 {
        self.position
    }
}

impl HashPosition {
    fn new(nb_sub_path: u32, mut moves: Vec<u8>) -> Self {
        moves.shrink_to(moves.len());
        Self { nb_sub_path, moves }
    }
}

fn get_subgrid(index: u8) -> u128 {
    let x = index % 10 % 3;
    let y = index / 10 % 3;

    let subgrid_indexes = (x..10)
        .step_by(3)
        .flat_map(|x| (y..10).step_by(3).map(|y| y * 10 + x).collect::<Vec<u8>>());

    subgrid_indexes.fold(0, |bitmap, index| bitmap | (1 << index))
}

fn is_subgrid_filled(position: &Position) -> bool {
    let subgrid_bitmap = get_subgrid(position.get_index());
    subgrid_bitmap & position.get_position() ^ subgrid_bitmap == 0
}

fn get_moves(position: &Position) -> Vec<u8> {
    let x = position.get_index() % 10;
    let y = position.get_index() / 10;

    let mut moves = vec![];
    if is_subgrid_filled(position) {
        if x < 8 && y < 8 {
            moves.push(position.get_index() + 22);
        }
        if x > 1 && y > 1 {
            moves.push(position.get_index() - 22);
        }
        if x < 8 && y > 1 {
            moves.push(position.get_index() - 18);
        }
        if x > 1 && y < 8 {
            moves.push(position.get_index() + 18);
        }
    } else {
        if x < 7 {
            moves.push(position.get_index() + 3);
        }
        if x > 2 {
            moves.push(position.get_index() - 3);
        }
        if y < 7 {
            moves.push(position.get_index() + 30);
        }
        if y > 2 {
            moves.push(position.get_index() - 30);
        }
    }

    moves
        .into_iter()
        .filter(|x| (1 << (*x as u128) & position.get_board()) == 0)
        .collect()
}

fn make_move(position: &Position, r#move: u8) -> Position {
    Position::new(position.get_board() | (1 << r#move), r#move)
}

fn explore_moves(
    graph: &mut HashMap<u128, HashPosition>,
    nb_sub_path_hashtable: &mut HashMap<u128, u32>,
    position: &Position,
) -> u32 {
    if position.is_grid_full() {
        return 1;
    }
    let position_hash = position.get_hash();

    if let Some(nb_sub_path) = nb_sub_path_hashtable.get(&position_hash) {
        return *nb_sub_path;
    }

    let mut nb_sub_path = 0;
    let mut winning_moves = vec![];
    for r#move in get_moves(position) {
        let new_position = make_move(position, r#move);
        let move_nb_sub_path = explore_moves(graph, nb_sub_path_hashtable, &new_position);
        nb_sub_path += move_nb_sub_path;
        if move_nb_sub_path > 0 {
            winning_moves.push((new_position.get_hash() ^ position_hash).trailing_zeros() as u8);
        }
    }

    nb_sub_path_hashtable.insert(position_hash, nb_sub_path);
    if nb_sub_path > 0 {
        graph.insert(position_hash, HashPosition::new(nb_sub_path, winning_moves));
    }
    nb_sub_path
}

pub fn make_graph() -> HashMap<u128, HashPosition> {
    let mut graph: HashMap<u128, HashPosition> = HashMap::new();
    let mut nb_sub_path_hashtable: HashMap<u128, u32> = HashMap::new();
    let position = Position::new(1, 0);
    explore_moves(&mut graph, &mut nb_sub_path_hashtable, &position);

    graph
}

pub fn get_moves_from_graph(graph: &HashMap<u128, HashPosition>, hash: u128) -> Vec<u8> {
    if let Some(hash_position) = graph.get(&hash) {
        return hash_position.moves.clone();
    }
    vec![]
}

pub fn get_path_from_index(graph: &HashMap<u128, HashPosition>, mut index: u32) -> Vec<u8> {
    assert!(index < SOLUTION_NUMBER);

    let mut path = vec![0];
    let mut node = graph.get(&1).unwrap();
    let mut board = 1u128;
    while path.len() < 100 {
        for r#move in &node.moves {
            let new_board = board | 1u128 << (*r#move as u128) | (*r#move as u128) << 100;
            if let Some(child_node) = graph.get(&new_board) {
                if index < child_node.nb_sub_path {
                    node = child_node;
                    path.push(*r#move);
                    board = new_board & FULL_BOARD;
                    break;
                } else {
                    index -= child_node.nb_sub_path;
                }
            } else {
                path.push(*r#move);
            }
        }
    }

    path
}

mod test {}

use std::collections::HashMap;

#[derive(Clone)]
#[derive(Debug)]
pub struct HashPosition {
    pub nb_sub_path: u32,
    pub moves: Vec<u128>,
}

#[derive(Clone)]
#[derive(Debug)]
struct Position {
    board: u128,
    index: u8,
}

impl HashPosition{
    fn new(nb_sub_path: u32, moves: Vec<u128>) -> Self{
        Self {nb_sub_path, moves}
    }
}

impl Position{
    fn new(board: u128, index: u8) -> Self{
        Self {board, index}
    }
    fn get_hash(&self) -> u128{
        (self.index as u128) << 100 | self.board
    }
}

fn get_subgrid(index: u8) -> u128{
    let x = index % 10 % 3;
    let y = index / 10 % 3;
    let xs = (x..10).step_by(3).collect::<Vec<u8>>();
    let ys = (y..10).step_by(3).collect::<Vec<u8>>();

    let mut bitmap = 0;
    for index in xs.iter().map(|x| ys.iter().map(|y| y * 10 + x).collect::<Vec<u8>>()).flatten(){
        bitmap |= 1 << index;
    }
    bitmap
}

fn is_subgrid_filled(position: &Position) -> bool {
    let subgrid_bitmap = get_subgrid(position.index);
    subgrid_bitmap & position.board ^ subgrid_bitmap == 0
}

fn get_moves(position: &Position) -> Vec<u8>{
    let x = position.index % 10;
    let y = position.index / 10;

    let mut moves = vec![];
    if is_subgrid_filled(&position){
        if x < 8 && y < 8 {
            moves.push(position.index + 22);
        }
        if x > 1 && y > 1 {
            moves.push(position.index - 22);
        }
        if x < 8 && y > 1 {
            moves.push(position.index - 18);
        }
        if x > 1 && y < 8 {
            moves.push(position.index + 18);
        }
    }else {
        if x < 7 {
            moves.push(position.index + 3);
        }
        if x > 2 {
            moves.push(position.index - 3);
        }
        if y < 7 {
            moves.push(position.index + 30);
        }
        if y > 2 {
            moves.push(position.index - 30);
        }
    }

    moves.iter().filter(|x| (1 << (**x as u128) & position.board) == 0).map(|x| *x).collect()
}

fn make_move(position: &Position, r#move: u8) -> Position{
    Position::new(position.board | (1 << r#move), r#move)
}

fn explore_moves(graph: &mut HashMap<u128, HashPosition>, nb_sub_path_hashtable: &mut HashMap<u128, u32>, position: &Position) -> u32{
    if position.board == 0b1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111 {
        return 1;
    }
    let position_hash = position.get_hash();

    if let Some(nb_sub_path) = nb_sub_path_hashtable.get(&position_hash){
        return *nb_sub_path;
    }

    let mut nb_sub_path = 0;
    let mut winning_moves = vec![];
    for r#move in get_moves(&position){
        let new_position = make_move(&position, r#move);
        let move_nb_sub_path = explore_moves(graph, nb_sub_path_hashtable, &new_position);
        nb_sub_path += move_nb_sub_path;
        if move_nb_sub_path > 0 {
            winning_moves.push(new_position.get_hash());
        }
    }

    nb_sub_path_hashtable.insert(position_hash, nb_sub_path);
    if nb_sub_path > 0 {
        graph.insert(position_hash, HashPosition::new(nb_sub_path, winning_moves));
    }
    nb_sub_path
}

pub fn make_graph() -> HashMap<u128, HashPosition> {
    let mut graph: HashMap<u128, HashPosition>= HashMap::new();
    let mut nb_sub_path_hashtable: HashMap<u128, u32> = HashMap::new();
    let position = Position::new(1, 0);
    explore_moves(&mut graph, &mut nb_sub_path_hashtable, &position);

    graph
}

pub fn get_moves_from_graph(graph: &HashMap<u128, HashPosition>, hash: u128) -> Vec<u8>{
    if let Some(hash_position) = graph.get(&hash){
        return hash_position.moves.iter().map(|x| x >> 100).map(|x| x as u8).collect();
    }
    vec![]
}

pub fn get_path_from_index(graph: &HashMap<u128, HashPosition>, mut index: u32) -> Vec<u8>{
    assert!(index < 33938944);

    let mut path = vec![0];
    let mut node = graph.get(&1).unwrap();
    while path.len() < 100 {
        for r#move in &node.moves {
            if let Some(child_node) = graph.get(r#move){
                if index < child_node.nb_sub_path{
                    node = child_node;
                    path.push((r#move >> 100) as u8);
                    break;
                }else {
                    index -= child_node.nb_sub_path;
                }
            }else {
                path.push((r#move >> 100) as u8);
            }
        }
    }

    path
}

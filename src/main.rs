fn main() {
    println!("Hello, world!");
    let fen_string="rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
    let board_array=convert_fen_to_vec(fen_string);
    let side_to_play=fen_string.split_whitespace().collect::<Vec<&str>>()[1];
    let half_move_clock=fen_string.split_whitespace().collect::<Vec<&str>>()[4];
    let full_move_number=fen_string.split_whitespace().collect::<Vec<&str>>()[5];
    let castling_availability=fen_string.split_whitespace().collect::<Vec<&str>>()[2];
    let enpassant_target=fen_string.split_whitespace().collect::<Vec<&str>>()[3];
    let mut available_moves=vec![];
    add_available_moves_rook("a1",&board_array,&mut available_moves,"b");
    println!("{:?}",available_moves);
}

fn add_available_moves_rook(start_square:&str,board_array:&[[&str;8];8],available_moves:&mut Vec<&str>,color:&str)->(){
    available_moves.push("Re4");
}

fn convert_fen_to_vec(fen:&str)->[[&str;8];8]{
    let mut return_array=[["";8];8];
    let piece_locations=fen.split_whitespace().collect::<Vec<&str>>()[0].split("/").collect::<Vec<&str>>();
    for (row_index,row) in piece_locations.iter().enumerate(){
        let mut in_row_index=0;
        let mut character_vector=row.split("").collect::<Vec<&str>>();
        character_vector.remove(0);
        character_vector.pop();
        for char in character_vector.iter(){
            if char.parse::<i64>().is_ok(){
                for i in 0..char.parse::<i64>().unwrap(){
                    //println!("{:?}",in_row_index);
                    //println!("{:?}",char);
                    return_array[row_index][in_row_index]="";
                    in_row_index+=1;
                }
            }else{
                //println!("{:?}",in_row_index);
                //println!("{:?}",char);
                return_array[row_index][in_row_index]=char;
                in_row_index+=1;
            }

        }
    }

    return_array
}

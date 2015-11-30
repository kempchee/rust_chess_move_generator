const COLUMNS_LOOKUP:[&'static str;8]=["a","b","c","d","e","f","g","h"];
const ROWS_LOOKUP:[&'static str;8]=["1","2","3","4","5","6","7","8"];

fn main() {
    println!("Hello, world!");
    let fen_string="rnbqkbn/1ppppppp/8/8/4P3/8/PPPP1PPP/rNBQKBNR b KQkq e3 0 1";
    let board_array=convert_fen_to_vec(fen_string);
    let side_to_play=fen_string.split_whitespace().collect::<Vec<&str>>()[1];
    let half_move_clock=fen_string.split_whitespace().collect::<Vec<&str>>()[4];
    let full_move_number=fen_string.split_whitespace().collect::<Vec<&str>>()[5];
    let castling_availability=fen_string.split_whitespace().collect::<Vec<&str>>()[2];
    let enpassant_target=fen_string.split_whitespace().collect::<Vec<&str>>()[3];
    let mut available_moves:Vec<String>=vec![];

    //println!("{:?}",board_array[0][0]);
    //println!("{:?}",board_array[0][0].to_string().chars().nth(0).unwrap().is_lowercase());


    for (row_index,row) in board_array.iter().enumerate(){
        for (column_index,column) in row.iter().enumerate(){
            if side_to_play=="b"{
                if board_array[row_index][column_index]!=""&&board_array[row_index][column_index].to_string().chars().nth(0).unwrap().is_lowercase(){
                    match board_array[row_index][column_index]{
                        "r"=>add_available_moves_rook(row_index as i8,column_index as i8,&board_array,&mut available_moves,"b"),
                        "p"=>add_available_moves_pawn(row_index,column_index,&board_array,&mut available_moves,"b"),
                        "k"=>add_available_moves_king(row_index,column_index,&board_array,&mut available_moves,"b"),
                        "q"=>add_available_moves_queen(row_index,column_index,&board_array,&mut available_moves,"b"),
                        "b"=>add_available_moves_bishop(row_index,column_index,&board_array,&mut available_moves,"b"),
                        "n"=>add_available_moves_knight(row_index,column_index,&board_array,&mut available_moves,"b"),
                        _=>println!("something else"),
                    }
                }
            }else{
                if board_array[row_index][column_index]!=""&&board_array[row_index][column_index].to_string().chars().nth(0).unwrap().is_uppercase(){
                    match board_array[row_index][column_index]{
                        "R"=>add_available_moves_rook(row_index as i8,column_index as i8,&board_array,&mut available_moves,"w"),
                        "P"=>add_available_moves_pawn(row_index,column_index,&board_array,&mut available_moves,"w"),
                        "K"=>add_available_moves_king(row_index,column_index,&board_array,&mut available_moves,"w"),
                        "Q"=>add_available_moves_queen(row_index,column_index,&board_array,&mut available_moves,"w"),
                        "B"=>add_available_moves_bishop(row_index,column_index,&board_array,&mut available_moves,"w"),
                        "N"=>add_available_moves_knight(row_index,column_index,&board_array,&mut available_moves,"w"),
                        _=>println!("something else"),
                    }

                }
            }

        }
    }
    println!("{:?}",available_moves);
}

fn add_available_moves_rook(start_row:i8,start_column:i8,board_array:&[[&str;8];8],available_moves:&mut Vec<String>,color:&str)->(){
    let mut notation_letter:String="R".to_string();
    let mut row_counter:i8=1;
    println!("{:?}",start_row);
    while start_row+row_counter<=7{
        let next_square_contents=board_array[(start_row+row_counter) as usize][start_column as usize];
        if next_square_contents==""{
            let possible_move=notation_letter.clone()+COLUMNS_LOOKUP[start_column as usize].clone()+ROWS_LOOKUP[(start_row+row_counter) as usize];
            available_moves.push(possible_move);
        }else if (next_square_contents.to_string().chars().nth(0).unwrap().is_uppercase()&&color=="w")||(next_square_contents.to_string().chars().nth(0).unwrap().is_lowercase()&&color=="b"){
            break
        }else{
            let possible_move=notation_letter.clone()+"x"+COLUMNS_LOOKUP[start_column as usize].clone()+ROWS_LOOKUP[(start_row+row_counter)as usize];
            available_moves.push(possible_move);
            break
        }
        row_counter+=1;
    }
    row_counter=1;
    println!("{:?}",start_row);
    while start_row-row_counter>=0{
        let next_square_contents=board_array[(start_row-row_counter) as usize][start_column as usize];
        if next_square_contents==""{
            let possible_move=notation_letter.clone()+COLUMNS_LOOKUP[start_column as usize].clone()+ROWS_LOOKUP[(start_row-row_counter) as usize];
            available_moves.push(possible_move);
        }else if (next_square_contents.to_string().chars().nth(0).unwrap().is_uppercase()&&color=="w")||(next_square_contents.to_string().chars().nth(0).unwrap().is_lowercase()&&color=="b"){
            break
        }else{
            let possible_move=notation_letter.clone()+"x"+COLUMNS_LOOKUP[start_column as usize].clone()+ROWS_LOOKUP[(start_row-row_counter) as usize];
            available_moves.push(possible_move);
            break
        }
        row_counter+=1;
    }
}

fn add_available_moves_pawn(start_row:usize,start_column:usize,board_array:&[[&str;8];8],available_moves:&mut Vec<String>,color:&str)->(){
    //available_moves.push("e4");
}

fn add_available_moves_bishop(start_row:usize,start_column:usize,board_array:&[[&str;8];8],available_moves:&mut Vec<String>,color:&str)->(){
    //available_moves.push("Be4");
}

fn add_available_moves_knight(start_row:usize,start_column:usize,board_array:&[[&str;8];8],available_moves:&mut Vec<String>,color:&str)->(){
    //available_moves.push("Ne4");
}

fn add_available_moves_king(start_row:usize,start_column:usize,board_array:&[[&str;8];8],available_moves:&mut Vec<String>,color:&str)->(){
    //available_moves.push("Ke4");
}

fn add_available_moves_queen(start_row:usize,start_column:usize,board_array:&[[&str;8];8],available_moves:&mut Vec<String>,color:&str)->(){
    //available_moves.push("Qe4");
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

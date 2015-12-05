const COLUMNS_LOOKUP:[&'static str;8]=["a","b","c","d","e","f","g","h"];
const ROWS_LOOKUP:[&'static str;8]=["8","7","6","5","4","3","2","1"];
use std::collections::HashMap;
fn main() {
    println!("Hello, world!");
    let fen_string="rnbqkbn/1ppppppp/8/8/4P3/8/PPPP1PPP/rNBQKBNR b KQkq e3 0 1";
    let board_array=convert_fen_to_array(fen_string);
    let side_to_play=fen_string.split_whitespace().collect::<Vec<&str>>()[1];
    let half_move_clock=fen_string.split_whitespace().collect::<Vec<&str>>()[4];
    let full_move_number=fen_string.split_whitespace().collect::<Vec<&str>>()[5];
    let castling_availability=fen_string.split_whitespace().collect::<Vec<&str>>()[2];
    let enpassant_target=fen_string.split_whitespace().collect::<Vec<&str>>()[3];
    let mut available_moves:Vec<String>=vec![];
    let mut duplicate_moves:HashMap<String,Vec<[usize;2]>>=HashMap::new();

    //println!("{:?}",board_array[0][0]);
    //println!("{:?}",board_array[0][0].to_string().chars().nth(0).unwrap().is_lowercase());


    for (row_index,row) in board_array.iter().enumerate(){
        for (column_index,column) in row.iter().enumerate(){
            if side_to_play=="b"{
                if board_array[row_index][column_index]!=""&&board_array[row_index][column_index].to_string().chars().nth(0).unwrap().is_lowercase(){
                    match board_array[row_index][column_index]{
                        "r"=>add_available_moves_rook(row_index as i8,column_index as i8,&board_array,&mut available_moves,"b",&mut duplicate_moves),
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
                        "R"=>add_available_moves_rook(row_index as i8,column_index as i8,&board_array,&mut available_moves,"w",&mut duplicate_moves),
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
    available_moves.sort();
    let mut duplicate_count=0;
    for (index,_) in available_moves.iter().enumerate(){
        if index!=0{
            if available_moves[index-1]==available_moves[index]{
                duplicate_count+=1;
            }
        }
        if index!=7{
            if available_moves[index+1]==available_moves[index]{
            }else{

            }
        }
    }
    for key in duplicate_moves.keys(){
        //let mut new_moves=vec!();
        let mut piece_locations=duplicate_moves.get(key).unwrap().clone();
        if piece_locations.len()>1{
            piece_locations.sort_by(|a,b|a[0].cmp(&b[0]));
        }
    }
    let converted_fen=convert_array_to_fen(board_array,side_to_play,castling_availability,enpassant_target,half_move_clock,full_move_number);
    println!("{:?}",converted_fen);
    println!("{:?}",duplicate_moves);
    println!("{:?}",available_moves);
}

fn convert_array_to_fen(board_array:[[&str;8];8],side_to_play:&str,castling_availability:&str,enpassant_target:&str,half_move_clock:&str,full_move_number:&str)->String{
    let mut fen_string="".to_string();
    for (row_index,row) in board_array.iter().enumerate(){
        let mut empty_square_count=0;
        for column in row.iter(){
            if column.to_string()==""{
                empty_square_count+=1;
            }else{
                if empty_square_count>0{
                    fen_string=fen_string+&empty_square_count.to_string()[..];
                }
                fen_string=fen_string+column;
                empty_square_count=0;
            }
        }
        if empty_square_count>0{
            fen_string=fen_string+&empty_square_count.to_string()[..];
        }
        if row_index!=7{
            fen_string=fen_string+"/";
        }
    }
    fen_string
}

fn add_available_moves_rook(start_row:i8,start_column:i8,board_array:&[[&str;8];8],available_moves:&mut Vec<String>,color:&str,duplicate_moves:&mut HashMap<String,Vec<[usize;2]>>)->(){
    let mut notation_letter:String="R".to_string();
    let mut row_counter:i8=1;
    let mut column_counter:i8=1;

    while start_row+row_counter<=7{
        let next_square_contents=board_array[(start_row+row_counter) as usize][start_column as usize];
        if next_square_contents==""{
            let possible_move=notation_letter.clone()+COLUMNS_LOOKUP[start_column as usize].clone()+ROWS_LOOKUP[(start_row+row_counter) as usize];
            insert_into_duplicate_moves(duplicate_moves,&possible_move,start_row,start_column);
            available_moves.push(possible_move);
        }else if (next_square_contents.to_string().chars().nth(0).unwrap().is_uppercase()&&color=="w")||(next_square_contents.to_string().chars().nth(0).unwrap().is_lowercase()&&color=="b"){
            break
        }else{
            let possible_move=notation_letter.clone()+"x"+COLUMNS_LOOKUP[start_column as usize].clone()+ROWS_LOOKUP[(start_row+row_counter)as usize];
            insert_into_duplicate_moves(duplicate_moves,&possible_move,start_row,start_column);
            available_moves.push(possible_move);
            break
        }
        row_counter+=1;
    }
    row_counter=1;

    while start_row-row_counter>=0{
        let next_square_contents=board_array[(start_row-row_counter) as usize][start_column as usize];
        if next_square_contents==""{
            let possible_move=notation_letter.clone()+COLUMNS_LOOKUP[start_column as usize].clone()+ROWS_LOOKUP[(start_row-row_counter) as usize];
            insert_into_duplicate_moves(duplicate_moves,&possible_move,start_row,start_column);
            available_moves.push(possible_move);
        }else if (next_square_contents.to_string().chars().nth(0).unwrap().is_uppercase()&&color=="w")||(next_square_contents.to_string().chars().nth(0).unwrap().is_lowercase()&&color=="b"){
            break
        }else{
            let possible_move=notation_letter.clone()+"x"+COLUMNS_LOOKUP[start_column as usize].clone()+ROWS_LOOKUP[(start_row-row_counter) as usize];
            insert_into_duplicate_moves(duplicate_moves,&possible_move,start_row,start_column);
            available_moves.push(possible_move);
            break
        }
        row_counter+=1;
    }

    while start_column+column_counter<=7{
        println!("{:?}",(start_column+column_counter));
        let next_square_contents=board_array[start_row as usize][(start_column+column_counter) as usize];
        if next_square_contents==""{
            let possible_move=notation_letter.clone()+COLUMNS_LOOKUP[(start_column+column_counter) as usize].clone()+ROWS_LOOKUP[start_row as usize];
            insert_into_duplicate_moves(duplicate_moves,&possible_move,start_row,start_column);
            available_moves.push(possible_move);
        }else if (next_square_contents.to_string().chars().nth(0).unwrap().is_uppercase()&&color=="w")||(next_square_contents.to_string().chars().nth(0).unwrap().is_lowercase()&&color=="b"){
            break
        }else{
            let possible_move=notation_letter.clone()+"x"+COLUMNS_LOOKUP[(start_column+column_counter) as usize].clone()+ROWS_LOOKUP[start_row as usize];
            insert_into_duplicate_moves(duplicate_moves,&possible_move,start_row,start_column);
            available_moves.push(possible_move);
            break
        }
        column_counter+=1;
    }

    column_counter=1;
    while start_column-column_counter>=0{
        println!("{:?}",(start_column-column_counter));
        let next_square_contents=board_array[start_row as usize][(start_column-column_counter) as usize];
        if next_square_contents==""{
            let possible_move=notation_letter.clone()+COLUMNS_LOOKUP[(start_column-column_counter) as usize].clone()+ROWS_LOOKUP[start_row as usize];
            insert_into_duplicate_moves(duplicate_moves,&possible_move,start_row,start_column);
            available_moves.push(possible_move);
        }else if (next_square_contents.to_string().chars().nth(0).unwrap().is_uppercase()&&color=="w")||(next_square_contents.to_string().chars().nth(0).unwrap().is_lowercase()&&color=="b"){
            break
        }else{
            let possible_move=notation_letter.clone()+"x"+COLUMNS_LOOKUP[(start_column-column_counter) as usize].clone()+ROWS_LOOKUP[start_row as usize];
            insert_into_duplicate_moves(duplicate_moves,&possible_move,start_row,start_column);
            available_moves.push(possible_move);
            break
        }
        column_counter+=1;
    }
}

fn insert_into_duplicate_moves(duplicate_moves:&mut HashMap<String,Vec<[usize;2]>>,possible_move:&String,start_row:i8,start_column:i8)->(){
    if duplicate_moves.contains_key(possible_move){
        duplicate_moves.get_mut(possible_move).unwrap().push([start_row as usize,start_column as usize]);
    }else{
        duplicate_moves.insert(possible_move.clone(),vec!([start_row as usize,start_column as usize]));
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

fn convert_fen_to_array(fen:&str)->[[&str;8];8]{
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

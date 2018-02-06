use std::path::Path;
use std::io::prelude::*;
use std::fs::File;
use std::error::Error;
use std::mem;
pub struct Cell{
    val:i32,
    con:[i32;9],
}
type cell = Cell;
struct Board{
    board:[[cell;9];9]
}

impl Board{
    fn new() -> Board{
       let s :Board = unsafe { mem::zeroed() };
        return s;
    }
}

fn main() {
    let mut b:Board = Board::new();
    read_board(&mut b);
    while zero_cell(&b) {
        update_con(&mut b);
        solve_board(&mut b);
        println!("repeat");
    }
    print_board(&b);

}

fn print_board(b:&Board){
    let mut row:usize = 0;
    let mut col:usize = 0;
    while col < 9 {
        while row < 9 {
            print!("{:?} ",b.board[col][row].val);
            row += 1;
        }
        println!("");
        col += 1;
        row = 0;
    }
}

fn zero_cell(b:&Board) -> bool{
    let mut row:usize = 0;
    let mut col:usize = 0;
    while col < 9 {
        while row < 9 {
            if b.board[col][row].val == 0 {
                return true;
            }
            row += 1;
        }
        col += 1;
        row = 0;
    }
    return false;
}
fn solve_board(b:&mut Board){
    let mut row:usize = 0;
    let mut col:usize = 0;
    while col < 9 {
        while row < 9 {
            if b.board[col][row].val == 0 {
                let res = find_cell(b.board[col][row].con);
                b.board[col][row].val = res;
//                println!("col = {} ,row = {} {:?} find_cell = {}", col, row, b.board[col][row].con, res);
            }
            row += 1;
        }
        col += 1;
        row = 0;
    }
}
fn update_con(b:&mut Board){
    let mut row:usize = 0;
    let mut col:usize = 0;
    while col < 9 {
        while row < 9 {
            if b.board[col][row].val == 0 {
                b.board[col][row].con = get_con(&b.board,row,col);
            }
            row += 1;
        }
        col += 1;
        row = 0;
    }
}

fn get_con(b:&[[cell;9];9],row:usize,col:usize) -> [i32;9]{
    let arr_row:[i32;9] = read_row(b,col);
    let arr_col:[i32;9] = read_col(b,row);
    let arr_sqr:[i32;9] = read_sqr(b,find_sqr(row,col));
    let merg_arr:[i32;9] = merge_array(arr_row,arr_col,arr_sqr);
    let rever_arr:[i32;9] = reverse_array(merg_arr);
    rever_arr
}
fn read_board(b:&mut Board){
    let mut s = String::new();
    // FIXME 
    let path = Path::new("~/board.txt");
    let mut f = match File::open(&path){
        Err(e) => panic!("problem {}",e.description()),
        Ok(file) => file
    };
    match f.read_to_string(&mut s){
        Err(e) => panic!("err"),
        Ok(_) => println!("ok"),
    }
    let radix:u32 = 10;
    let mut i:usize = 0;
    let mut j:usize = 0;
    let mut arr:[[i32;9];9]= [[0;9];9];
    for ch in s.chars(){
         if ch != ' ' && ch != '.' && ch != '\n'{
             b.board[j][i] = Cell{val:ch.to_digit(radix).unwrap() as i32,con:[0,0,0,0,0,0,0,0,0]};
             next_cell(&mut i,&mut j);
        }else if ch == '.'{
             b.board[j][i] = Cell{val:0,con:[0,0,0,0,0,0,0,0,0]};
            next_cell(&mut i,&mut j);

        }
    }
}
fn next_cell(row:&mut usize,col:&mut usize){
    if *row < 8{
        *row += 1;
    }else if *row == 8 {
        if *col < 8 {
            *row = 0;
            *col += 1;
        }else if *col == 8 {
            *row = 8;
            *col = 8;
        }
    }
}

// return array of value between 1...9 is not exist in array input
fn reverse_array(array:[i32;9]) -> [i32;9]{
    let mut i:i32 = 1;
    let mut j:usize = 0;
    let mut result:[i32;9] = [0;9];
    while i <= 9 {
        if ! array.contains(&i){
            result[j] = i as i32;
            j += 1;
        }
        i +=1;
    }
//    result.sort();
    return result;
}
fn merge_array(arr1:[i32;9],arr2:[i32;9],arr3:[i32;9]) -> [i32;9]{
    let mut i:i32 = 1;
    let mut j:usize = 0;
    let mut result:[i32;9] = [0;9];
    while i <= 9 {
        if arr1.contains(&i) || arr2.contains(&i) || arr3.contains(&i){
            result[j] = i as i32;
            j += 1;
        }
        i +=1;
    }
    result
}
fn find_cell(con:[i32;9]) -> i32{
    let mut i:usize = 0;
    let mut feild:i32 = 0;
    let mut result:i32 = 0;
    while i < 9 {
        if con[i] > 0 {
            feild +=1;
        }
        i +=1;
    }
    if feild == 1{
        i = 0;
        while i < 9 {
        if con[i] > 0 {
           result = con[i];
        }
        i +=1;
        }
    }
    result
}

fn read_row(b:&[[cell;9];9],col:usize) -> [i32;9]{
    let mut ir:usize = 0;
    let mut ib:usize = 0;
    let mut t:[i32;9] = [0;9];
    while ib < 9 {
        if b[col][ib].val != 0{
            t[ir] = b[col][ib].val;
            ir += 1;
        }
        ib += 1;
    }
    return t;
}
fn read_col(b:&[[cell;9];9],row:usize) -> [i32;9]{
    let mut ic:usize = 0;
    let mut ib:usize = 0;
    let mut t:[i32;9] = [0;9];
    while ib < 9 {
        if b[ib][row].val != 0{
            t[ic] = b[ib][row].val;
            ic += 1;
        }
        ib += 1;
    }
    return t;
}
fn read_sqr(b:&[[cell;9];9],sqr:i32) -> [i32;9]{
    let mut ra:usize = 0;
    let mut rb:usize = 0;
    let mut ca:usize = 0;
    let mut cb:usize = 0;
    let mut t:[i32;9] = [0;9];
    if sqr == 1 {
        ra = 0;
        rb = 3;
        ca = 0;
        cb = 3;
    }else if sqr == 2 {
        ra = 3;
        rb = 6;
        ca = 0;
        cb = 3;
    }else if sqr == 3{
        ra = 6;
        rb = 9;
        ca = 0;
        cb = 3;
    }else if sqr == 4 {
        ra = 0;
        rb = 3;
        ca = 3;
        cb = 6;
    }else if sqr == 5 {
        ra = 3;
        rb = 6;
        ca = 3;
        cb = 6;
    }else if sqr == 6 {
        ra = 6;
        rb = 9;
        ca = 3;
        cb = 6;
    }else if sqr == 7 {
        ra = 0;
        rb = 3;
        ca = 6;
        cb = 9;
    }else if sqr == 8 {
        ra = 3;
        rb = 6;
        ca = 6;
        cb = 9;
    }else if sqr == 9 {
        ra = 6;
        rb = 9;
        ca = 6;
        cb = 9;
    }
    let mut row = ra;
    let mut col = ca;
    let mut i = 0;
    while  col < cb{
        while  row < rb {
            t[i] = b[col][row].val;
            row += 1;
            i += 1;
        }
        col += 1;
        row = ra;
    }
    t.sort();
    return t;
}

fn find_sqr(row:usize,col:usize) -> i32{
    let mut sqr:i32 = 0;
    if row < 3 && col < 3{
        sqr = 1;
    }else if row >= 3 && row < 6 && col < 3{
        sqr = 2;
    }else if row >= 6 && col < 3{
        sqr = 3;
    }else if row < 3 && col >= 3 && col < 6{
        sqr = 4;
    }else if row >= 3 && row < 6 && col >= 3 && col < 6{
        sqr = 5;
    }else if row >= 6 && col >= 3 && col < 6{
        sqr = 6;
    }else if row < 3 && col >= 6 {
        sqr = 7;
    }else if row >= 3 && row < 6 && col >= 6 {
        sqr = 8;
    }else if row >= 6 && col >= 6 {
        sqr = 9;
    }
    sqr
}

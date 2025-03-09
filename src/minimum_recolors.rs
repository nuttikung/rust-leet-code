// println!("{}", minimum_recolors(String::from("WBBWWBBWBW") , 7))

use std::i32::MAX;

fn count_white(text: String) -> i32{
    let mut white_count = 0;
    for i in text.chars(){
        if i == 'W' {
            white_count+=1;
        }
    }
    return white_count;
}


fn minimum_recolors(blocks: String, k: i32) -> i32 {
    let max_window_len = blocks.len() as i32 - k;
    let mut min = i32::MAX;
    // sliding window
    for index in 0..=max_window_len {
        let start = index as usize;
        let end = (index+k) as usize;
        let sub_string = &blocks[start..end];
        let count = count_white(String::from(sub_string));

        if count == 0 {
            min = count;
            break;
        }

        if count < min {
            min = count;
        }
    }

    return min;
}

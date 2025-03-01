fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
    let mut answer:Vec<i32> = Vec::new();
    let mut index = 0;
    loop {
        if index >= words.len() {
            break;
        }
        if words[index].contains(x) {
            answer.push(index.try_into().unwrap());
        }
        index+=1;
    }
    return answer
}

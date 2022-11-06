use std::fs;

fn main() {
    let wordlist = fs::read_to_string("words.txt").expect("Can't open file"); /* Read wordlist from file as a string */
    let mut words: Vec<&str> = wordlist.split('\n').collect(); /* Split wordlist by new-line and collect into vector (?) */
    words.retain(|&w| w.len() == 6 && !(w.contains('\''))); /* Discard words that are not 6 characters long */

    for word in words {
        println!("{}", word);
    }

    let mut slot0: Vec<&str> = "s,f,c,a,y,r,d,k".split(',').collect();
    let mut slot1: Vec<&str> = "g,m,e,c,t,o,u,r".split(',').collect();
    let mut slot2: Vec<&str> = "t,l,e,v,o,x,m,j".split(',').collect(); /* Explore other collections that can be used */
    let mut slot3: Vec<&str> = "k,g,b,q,h,f,r,i".split(',').collect();
    let mut slot4: Vec<&str> = "n,z,v,i,t,u,p,a".split(',').collect();
    let mut slot5: Vec<&str> = "e,r,f,w,m,j,l,n".split(',').collect();

    // retain(|&w| w.contains("sm..."));
}

// fn truncateWordlist(word_list: &str) -> &str {
//     let mut words = word_list.split('\n').iter();
// }

// collect() consumes an iterator and converts it into a collection data type.

// without graphemes
// sturggling on the bonus so uploading to see other solutions
pub fn reverse(input: &str) -> String {
    input.chars().rev().fold(String::new(), {
        |mut st, c| {
            st.push(c);
            st
        }
    })
}

// use unicode_segmentation::UnicodeSegmentation;

// pub fn reverse(input: &str) -> String {
//     let str1 = UnicodeSegmentation::graphemes(input, true).collect::<Vec<&str>>();

//     str1.reverse();
//     str1.iter().fold(String::new(), {
//         |mut st, c| {
//             st.push(c);
//             st
//         }
//     })
// }

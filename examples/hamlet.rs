use std::collections::HashSet;

fn main() {
    let input = read_hamlet();

    // run the algorithm with different buffer sizes
    let estimate_100 = cvm::estimate_distinct_elements(&input, 100);
    let estimate_500 = cvm::estimate_distinct_elements(&input, 500);
    let estimate_1000 = cvm::estimate_distinct_elements(&input, 1000);

    // count the actual number of words, for comparison
    let correct = input.into_iter().collect::<HashSet<_>>().len();

    // print the results
    dbg!(estimate_100, estimate_500, estimate_1000, correct);
}

fn read_hamlet() -> Vec<String> {
    let text = include_str!("hamlet.txt");
    let text = text.replace(['.', ',', '?', ':', ';', '!'], " ");
    let text = text.replace("--", " ");
    let text = text.to_lowercase();
    text.split_whitespace().map(Into::into).collect()
}

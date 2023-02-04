use apriori::apriori;

fn main() {

    // We define our itemset 
    let sets = vec![vec![1, 2, 3, 4],
    vec![1, 2, 4],
    vec![1, 2],
    vec![2, 3, 4],
    vec![2, 3],
    vec![3, 4],
    vec![2, 4]];

    // we apply our apriori algorithm on it
    let output = apriori(sets, 0.4);

    // We print our result
    println!("Result: {:?}", output);

}

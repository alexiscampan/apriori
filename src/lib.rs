/// some imports 
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::fmt::Debug;

pub trait ItemType: Eq + Ord + Hash + Copy + Debug {}

#[derive(Debug, Default, Clone)]
pub struct ItemSet {
    pub items: HashSet<u64>,
    pub support: f64,
    pub count: u64,
}

///
/// Implementation of Hash for an ItemSet
impl Hash for ItemSet {
    /// For each item in itemset with hash them in b64 to encode it, we also sort it before hash
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut a: Vec<u64> = self.items.iter().cloned().collect();
        a.sort();
        for s in a.iter() {
            state.write_u64(*s);
        }
    }
}

///
/// Implementation of PartialEq for an ItemSet
impl PartialEq for ItemSet {
    /// This function define if items are same of not, only used for comparison
    fn eq(&self, other: &ItemSet) -> bool {
        self.items == other.items
    }
}

impl Eq for ItemSet {}

///
///  First iteration to count the frequency of each items 
fn first_pass(sets: &Vec<Vec<u64>>, min_support: f64) -> (HashSet<ItemSet>, u64) {
    let mut sets_count = 0;
    let large = sets
        .iter()
        .inspect(|_| sets_count += 1)
        .flat_map(|x| x.iter())
        .fold(HashMap::new(), |mut acc, x| { // for each entry of our hashmap, if we have'nt count it yet we create the item in our hashset, in the other case we just increment the support by 1
            *acc.entry(x).or_insert(0) += 1;
            acc
        })
        .iter()
        .filter(|&(_, &v)| v as f64 / sets_count as f64 > min_support) // then we create the supports as frequency, that will be use in apriori algotihm
        .fold(HashSet::new(), |mut acc, (&k, &v)| { // and finally recreate our brand new HashSet
            acc.insert(ItemSet {
                items: vec![*k].into_iter().collect::<HashSet<u64>>(),
                support: v as f64 / sets_count as f64,
                count: v,
                ..Default::default()
            });

            acc
        });

    (large, sets_count) // return our new hashset and the count associated
}

/// 
/// Function to generate the subsets of items
/// Based on an itemset this method will return a new Hashset with an apropriated format for apriori algorithm
fn generate_subsets(large: HashSet<ItemSet>) -> HashSet<ItemSet> {
    large.iter().fold(HashSet::new(), |mut acc, rit| {
        for lit in large.iter() { // for each item in our previous large HashSet
            for i1 in rit.items.difference(&lit.items).cloned() {  // we compute the difference with the other items of the large HashSet
                let mut candidate = lit.items.clone();
                candidate.insert(i1); // we store our canditate 

                if candidate.iter().fold(true, |acc, elem| {
                    let subset = candidate
                        .difference(&vec![*elem].into_iter().collect::<HashSet<u64>>()) // on compute la difference de chaque element de notre candidat 
                        .cloned()
                        .collect();
                    acc && large.contains(&ItemSet { // on verifie que notre hashset temporaire et notre large hashset contiennent le même subset 
                        items: subset,
                        ..Default::default()
                    })
                }) {
                    acc.insert(ItemSet { // on le candidat dans le nouvel Hashset
                        items: candidate,
                        ..Default::default()
                    });
                }
            }
        }

        acc
    })
}

///
/// Main function of apriori algorithm
pub fn apriori(sets: Vec<Vec<u64>>, min_support: f64) -> HashSet<ItemSet> {
    // we apply the weights of each items after the first pass
    let (mut large, sets_count) = first_pass(&sets, min_support);
    // we create an empty output that will receive our results
    let mut output = HashSet::<ItemSet>::new();

    // on itere sur tous nos inputs
    while !large.is_empty() {
        let mut candidates = generate_subsets(large); // on genere nos subsets en regardant chaque candidat

        // pour chaque set on va calculer son support correspondant
        for set in sets.iter() { 
            let hash_set = HashSet::from_iter(set.iter().cloned());

            candidates = candidates
                .into_iter()
                .map(|mut x| {
                    if x.items.is_subset(&hash_set) {
                        x.count += 1;
                        x.support = x.count as f64 / sets_count as f64;
                    }

                    x
                })
                .collect();
        }

        // on filtre désormais avec le min support défini lors de notre appel à la fonction apriori
        large = HashSet::from_iter(
            candidates
                .iter()
                .cloned()
                .filter(|x| x.support > min_support),
        );

        output.extend(large.clone());
    }

    output // we return our results
}

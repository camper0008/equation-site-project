use crate::database::db::{Db, DbError};
use crate::models::PreviewableEquation;
use std::cmp::Ordering;

struct PrevEqWithLevDist {
    pub id: String,
    pub title: String,
    pub date_created: String,
    pub lev_dist: usize,
}

impl Ord for PrevEqWithLevDist {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.lev_dist).cmp(&other.lev_dist)
    }
}

impl PartialOrd for PrevEqWithLevDist {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PrevEqWithLevDist {
    fn eq(&self, other: &Self) -> bool {
        (self.lev_dist) == (other.lev_dist)
    }
}

impl Eq for PrevEqWithLevDist {}

fn levenshtein(a: &str, b: &str) -> usize {
    /*
     * `levenshtein-rs` - levenshtein
     *
     * MIT licensed.
     *
     * Copyright (c) 2016 Titus Wormer <tituswormer@gmail.com>
     */
    let mut result = 0;

    /* Shortcut optimizations / degenerate cases. */
    if a == b {
        return result;
    }

    let length_a = a.chars().count();
    let length_b = b.chars().count();

    if length_a == 0 {
        return length_b;
    }

    if length_b == 0 {
        return length_a;
    }

    /* Initialize the vector.
     *
     * This is why it’s fast, normally a matrix is used,
     * here we use a single vector. */
    let mut cache: Vec<usize> = (1..).take(length_a).collect();
    let mut distance_a;
    let mut distance_b;

    /* Loop. */
    for (index_b, code_b) in b.chars().enumerate() {
        result = index_b;
        distance_a = index_b;

        for (index_a, code_a) in a.chars().enumerate() {
            distance_b = if code_a == code_b {
                distance_a
            } else {
                distance_a + 1
            };

            distance_a = cache[index_a];

            result = if distance_a > result {
                if distance_b > result {
                    result + 1
                } else {
                    distance_b
                }
            } else if distance_b > distance_a {
                distance_a + 1
            } else {
                distance_b
            };

            cache[index_a] = result;
        }
    }

    result
}

pub async fn search_equations(
    db: &mut Db,
    query: String,
) -> Result<Vec<PreviewableEquation>, DbError> {
    let mut levenshteined = db
        .all_titles()
        .await?
        .into_iter()
        .map(|eq| PrevEqWithLevDist {
            id: eq.id,
            title: eq.title.clone(),
            date_created: eq.date_created,
            lev_dist: levenshtein(&eq.title, &query),
        })
        .collect::<Vec<PrevEqWithLevDist>>();
    levenshteined.sort_unstable();
    Ok(levenshteined
        .into_iter()
        .take(100)
        .map(|eq| PreviewableEquation {
            id: eq.id,
            title: eq.title,
            date_created: eq.date_created,
        })
        .collect())
}

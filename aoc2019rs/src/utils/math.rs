pub fn index_permutations(size: usize) -> Vec<Vec<usize>> {
    let mut result = Vec::new();
    let mut current_size = 0;
    while current_size < size {
        result = next_permutation_set(&result);
        current_size += 1;
    }
    result
}

fn next_permutation_set(permutations: &[Vec<usize>]) -> Vec<Vec<usize>> {
    if permutations.is_empty() {
        return vec![vec![0]];
    }
    
    let mut result = Vec::new();
    
    for permutation in permutations {
        let next_index = permutation.len();
        for i in 0..permutation.len() + 1 {
            let mut next_permutation = Vec::new();
            next_permutation.extend_from_slice(&permutation[..i]);
            next_permutation.push(next_index);
            next_permutation.extend_from_slice(&permutation[i..]);
            result.push(next_permutation);
        }
    }

    result
}

pub fn permutations<T>(input: &[T]) -> Vec<Vec<&T>> {
    let mut result = Vec::new();

    for indices in index_permutations(input.len()) {
        let mut next_permutation = Vec::with_capacity(indices.len());
        for index in indices {
            next_permutation.push(&input[index]);
        }
        result.push(next_permutation);
    }

    result
}

pub fn permutations_cloned<T: Clone>(input: &[T]) -> Vec<Vec<T>> {
    permutations(input).into_iter()
        .map(|perm| perm.into_iter().map(Clone::clone).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutations() {
        let set = vec!['A', 'B', 'C'];
        let permutations = permutations(&set);
        assert_eq!(permutations, &[
            [&'C', &'B', &'A'], [&'B', &'C', &'A'], 
            [&'B', &'A', &'C'], [&'C', &'A', &'B'], 
            [&'A', &'C', &'B'], [&'A', &'B', &'C'],
        ]);
    }

    #[test]
    fn test_index_permutations() {
        assert_eq!(index_permutations(0), Vec::<Vec<usize>>::new());
        assert_eq!(index_permutations(1), &[[0]]);
        assert_eq!(index_permutations(2), &[[1, 0], [0, 1]]);
        assert_eq!(index_permutations(3),&[
            [2, 1, 0], [1, 2, 0], 
            [1, 0, 2], [2, 0, 1], 
            [0, 2, 1], [0, 1, 2],
        ]);
    }

    #[test]
    fn test_next_permutation_0() {
        let perm = next_permutation_set(&[]);
        assert_eq!(perm, &[[0]]);
    }

    #[test]
    fn test_next_permutation_1() {
        let perm = next_permutation_set(&next_permutation_set(&[]));
        assert_eq!(perm, &[[1, 0], [0, 1]]);
    }

    #[test]
    fn test_next_permutation_2() {
        let mut perm = next_permutation_set(&next_permutation_set(&next_permutation_set(&[])));
        perm.sort();
        assert_eq!(perm, &[
            [0, 1, 2], [0, 2, 1],
            [1, 0, 2], [1, 2, 0],
            [2, 0, 1], [2, 1, 0],
        ]);
    }

    #[test]
    fn test_next_permutation_3() {
        let mut perm = next_permutation_set(&next_permutation_set(&next_permutation_set(&next_permutation_set(&[]))));
        perm.sort();
        assert_eq!(perm, &[
            [0, 1, 2, 3], [0, 1, 3, 2], [0, 2, 1, 3], [0, 2, 3, 1], [0, 3, 1, 2], [0, 3, 2, 1], 
            [1, 0, 2, 3], [1, 0, 3, 2], [1, 2, 0, 3], [1, 2, 3, 0], [1, 3, 0, 2], [1, 3, 2, 0], 
            [2, 0, 1, 3], [2, 0, 3, 1], [2, 1, 0, 3], [2, 1, 3, 0], [2, 3, 0, 1], [2, 3, 1, 0], 
            [3, 0, 1, 2], [3, 0, 2, 1], [3, 1, 0, 2], [3, 1, 2, 0], [3, 2, 0, 1], [3, 2, 1, 0],
        ]);
    }
}

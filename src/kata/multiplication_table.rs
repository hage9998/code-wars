// Your task, is to create NxN multiplication table, of size provided in parameter.

// for example, when given size is 3:

// 1 2 3
// 2 4 6
// 3 6 9

fn multiplication_table(len: usize) -> Vec<Vec<usize>> {
    let mut vec = Vec::new();
    let mut new_vec = Vec::with_capacity(len);
    
    for i in 1..len + 1{
        for j in 1..len + 1{
            new_vec.push(i * j)
        }
        vec.push(new_vec.clone());
        new_vec.clear();
    }
    vec
}

fn multiplication_table2(n: usize) -> Vec<Vec<usize>> {
    (1..=n).map(|i| (1..=n).map(|j| i*j).collect()).collect()
}

fn multiplication_table3(len: usize) -> Vec<Vec<usize>> {
    (1..=len)
        .map(|n| (n..).step_by(n).take(len).collect())
        .collect()
}
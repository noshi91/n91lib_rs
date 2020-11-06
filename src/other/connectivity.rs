pub fn is_connected(n: usize, edges: &[(usize, usize)]) -> bool {
    assert_ne!(n, 0);

    let mut adj = vec![vec![]; n];
    for &(u, v) in edges {
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut reached = vec![false; n];
    let mut stack = vec![];
    stack.push(0);
    while let Some(v) = stack.pop() {
        if reached[v] {
            continue;
        }
        reached[v] = true;
        for &u in &adj[v] {
            stack.push(u);
        }
    }
    reached.into_iter().all(|x| x)
}

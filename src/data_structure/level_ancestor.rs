/*

References

[1] Ben-Amram, A. M. (2009). The Euler path to static level-ancestors.
    arXiv preprint arXiv:0909.1030.

[2] level ancestor problem
    アルゴリズム/データ構造を語る会　第 0 回
    https://drive.google.com/drive/folders/1htiwJtiSZ_ruYJYRk54cuJlJgea_XGrH

[3] Bender, M. A., & Farach-Colton, M. (2004).
    The level ancestor problem simplified.
    Theoretical Computer Science, 321(1), 5-12.


Description

T: 根付き木
n: |V(T)|

new(n, edges): データ構造を構築する。
query(v, d): 頂点 v の祖先で深さが d のものを計算する。

時間計算量
new: Θ(n)
query: Θ(1)

木を最長パスで分解してパスにする。
それぞれのパスについて根の方向に 2 倍だけ伸ばした配列を管理する。
これとダブリングを組み合わせることで、構築 Θ(n log(n)) クエリ Θ(1) の
アルゴリズムを得ることが出来る。
大きさ O(log(n)) の木の LA は select に帰着することが出来ることを利用して、
ダブリングのテーブルを持つ場所を削減することで構築が Θ(n) に改善される。


Detail

query の時間計算量は select を Θ(1) で行えるという仮定に基づいている。
適切にテーブルを作成することでこれは可能になるが、
実用上は面倒なので組み込みの関数を使用している。

*/

pub struct LevelAncestor {
    ladder: Box<[usize]>,
    nodes: Box<[Node]>,
}

#[derive(Clone)]
struct Node {
    depth: usize,
    data: NodeData,
}

#[derive(Clone)]
enum NodeData {
    Micro(MicroNode),
    Macro(MacroNode),
}

use NodeData::*;

#[derive(Clone)]
struct MacroNode {
    macro_jump: usize,
    ladder_pos: usize,
    doubling: Option<Box<[usize]>>,
}

#[derive(Clone)]
struct MicroNode {
    micro_root: usize,
    macro_head: usize,
    bit: usize,
    dfs: Option<Box<[usize]>>,
}

impl LevelAncestor {
    pub fn new(n: usize, edges: &[(usize, usize)]) -> Self {
        let mut graph = vec![vec![]; n].into_boxed_slice();
        for &(u, v) in edges {
            graph[u].push(v);
            graph[v].push(u);
        }

        let mut c = Closure {
            graph,
            nodes: vec![
                Partial {
                    depth: 0,
                    height: 0,
                    data: None,
                };
                n
            ]
            .into_boxed_slice(),
        };
        c.call(0, 0);

        let tree = c
            .graph
            .into_vec()
            .into_iter()
            .map(|a| a.into_boxed_slice())
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let mut nodes = c.nodes;

        let mut b = Builder {
            tree: &tree,
            nodes: &mut nodes,
            path: vec![],
            ladder: vec![],
        };
        b.call(n, 0, 0);

        Self {
            ladder: b.ladder.into_boxed_slice(),
            nodes: nodes
                .into_vec()
                .into_iter()
                .map(|p| p.build())
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }

    pub fn query(&self, v: usize, d: usize) -> usize {
        assert!(d <= self.nodes[v].depth);

        match self.nodes[v].data {
            Micro(ref m) => {
                let h_d = self.nodes[m.micro_root].depth;
                if h_d > d {
                    self.macro_la(m.macro_head, d)
                } else {
                    use crate::other::bit::select;

                    let d = d - h_d;
                    let root = self.nodes[m.micro_root].data.get_micro();
                    root.dfs.as_ref().unwrap()[select(m.bit, d)]
                }
            }
            Macro(_) => self.macro_la(v, d),
        }
    }

    fn macro_la(&self, v: usize, d: usize) -> usize {
        let v = self.nodes[v].data.get_macro().macro_jump;
        let jump = &self.nodes[v];
        let up = jump.depth - d;
        if up == 0 {
            v
        } else {
            use crate::other::bit::bsr;

            let p = bsr(up);
            let v = jump.data.get_macro().doubling.as_ref().unwrap()[p];
            let up = up - (1 << p);

            self.ladder[self.nodes[v].data.get_macro().ladder_pos - up]
        }
    }
}

impl NodeData {
    fn get_micro(&self) -> &MicroNode {
        match *self {
            Micro(ref m) => m,
            Macro(_) => panic!(),
        }
    }

    fn get_micro_mut(&mut self) -> &mut MicroNode {
        match *self {
            Micro(ref mut m) => m,
            Macro(_) => panic!(),
        }
    }

    fn get_macro(&self) -> &MacroNode {
        match *self {
            Micro(_) => panic!(),
            Macro(ref m) => m,
        }
    }

    fn get_macro_mut(&mut self) -> &mut MacroNode {
        match *self {
            Micro(_) => panic!(),
            Macro(ref mut m) => m,
        }
    }
}

#[derive(Clone)]
struct Partial {
    depth: usize,
    height: usize,
    data: Option<NodeData>,
}

impl Partial {
    fn build(self) -> Node {
        Node {
            depth: self.depth,
            data: self.data.unwrap(),
        }
    }
}

struct Closure {
    graph: Box<[Vec<usize>]>,
    nodes: Box<[Partial]>,
}

impl Closure {
    fn call(&mut self, v: usize, depth: usize) -> usize {
        use std::mem::take;

        let mut len = 0;
        let mut height = 0;
        let children = take(&mut self.graph[v]);
        for &u in &children {
            let idx = self.graph[u].iter().position(|&p| p == v).unwrap();
            self.graph[u].swap_remove(idx);

            len += self.call(u, depth + 1);

            height = height.max(self.nodes[u].height);
        }
        self.graph[v] = children;
        len += 1;
        height += 1;

        self.nodes[v] = Partial {
            depth,
            height,
            data: None,
        };

        use crate::other::bit::WORD;

        if len <= WORD {
            self.nodes[v].height = 0;
        } else {
            let (idx, _) = self.graph[v]
                .iter()
                .enumerate()
                .max_by_key(|&(_, &u)| self.nodes[u].height)
                .unwrap();
            self.graph[v].swap(0, idx);
        }

        return len;
    }
}

struct Builder<'a> {
    tree: &'a Box<[Box<[usize]>]>,
    nodes: &'a mut Box<[Partial]>,
    path: Vec<usize>,
    ladder: Vec<usize>,
}

impl<'a> Builder<'a> {
    fn call(&mut self, p: usize, v: usize, d: usize) {
        if self.nodes[v].height == 0 {
            let mut m = MakeMicroTree {
                tree: self.tree,
                nodes: &mut self.nodes,
                root: v,
                head: p,
                dfs: vec![],
            };
            m.call(v, 0);
            let dfs = m.dfs.into_boxed_slice();
            self.nodes[v].data.as_mut().unwrap().get_micro_mut().dfs = Some(dfs);
            return;
        }

        self.path.push(v);

        self.call(v, self.tree[v][0], d);

        for &u in self.tree[v][1..].iter() {
            self.call(v, u, self.path.len());
        }

        if self.nodes[v].height == 1 {
            let s = d.saturating_sub(self.path.len() - d - 1);
            let offset = self.ladder.len() + (d - s);
            self.ladder.extend_from_slice(&self.path[s..]);
            for (i, &u) in self.path[d..].iter().enumerate() {
                self.nodes[u].data = Some(Macro(MacroNode {
                    macro_jump: v,
                    ladder_pos: offset + i,
                    doubling: None,
                }));
            }

            let last = self.path.len() - 1;
            let mut doubling = vec![];
            let mut step = 1;
            while let Some(idx) = last.checked_sub(step) {
                doubling.push(self.path[idx]);
                step *= 2;
            }
            self.nodes[v]
                .data
                .as_mut()
                .unwrap()
                .get_macro_mut()
                .doubling = Some(doubling.into_boxed_slice());
        }

        self.path.pop();
    }
}

struct MakeMicroTree<'a> {
    tree: &'a Box<[Box<[usize]>]>,
    nodes: &'a mut Box<[Partial]>,
    root: usize,
    head: usize,
    dfs: Vec<usize>,
}

impl<'a> MakeMicroTree<'a> {
    fn call(&mut self, v: usize, bit: usize) {
        let bit = bit | 1 << self.dfs.len();
        self.dfs.push(v);
        self.nodes[v].data = Some(Micro(MicroNode {
            micro_root: self.root,
            macro_head: self.head,
            bit,
            dfs: None,
        }));
        for &u in self.tree[v].iter() {
            self.call(u, bit);
        }
    }
}

#[test]
fn test_level_ancestor() {
    struct Naive {
        parent: Vec<usize>,
        depth: Vec<usize>,
    }

    impl Naive {
        fn new(n: usize, edges: &[(usize, usize)]) -> Self {
            let mut g = vec![vec![]; n];
            for &(u, v) in edges {
                g[u].push(v);
                g[v].push(u);
            }

            let mut res = Naive {
                parent: vec![0; n],
                depth: vec![0; n],
            };

            fn dfs(g: &Vec<Vec<usize>>, res: &mut Naive, v: usize, p: usize, d: usize) {
                res.parent[v] = p;
                res.depth[v] = d;
                for &u in g[v].iter() {
                    if u != p {
                        dfs(g, res, u, v, d + 1);
                    }
                }
            }

            dfs(&g, &mut res, 0, n, 0);

            res
        }

        fn query(&self, mut v: usize, d: usize) -> usize {
            while self.depth[v] != d {
                v = self.parent[v];
            }
            v
        }
    }

    fn test_set(cases: usize, n: usize, q: usize) {
        use crate::other::rand::rand_int;

        for _ in 0..cases {
            let n = rand_int(1..n);
            let mut edges = vec![];
            let mut perm = (0..n).collect::<Vec<_>>();
            for i in 1..n {
                let j = rand_int(1..i + 1);
                perm.swap(i, j);
            }
            for i in 1..n {
                let j = rand_int(0..i);
                edges.push((perm[i], perm[j]));
            }

            let la = LevelAncestor::new(n, &edges);
            let naive = Naive::new(n, &edges);

            for _ in 0..q {
                let v = rand_int(0..n);
                let d = rand_int(0..naive.depth[v] + 1);

                assert_eq!(la.query(v, d), naive.query(v, d));
            }
        }
    }

    test_set(10, 30000, 10000);
    test_set(100, 300, 100);
    test_set(1000, 30, 10);
    test_set(10, 100, 10000);
}

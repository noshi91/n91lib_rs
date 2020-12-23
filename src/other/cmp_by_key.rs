/*

特に heap 等で用いる

*/

#[derive(Clone, Copy)]
pub struct CmpByKey<K, V>(pub K, pub V);

use std::cmp::Ordering;

impl<K, V> PartialEq for CmpByKey<K, V>
where
    K: PartialEq,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.0.eq(&rhs.0)
    }
}

impl<K, V> Eq for CmpByKey<K, V> where K: Eq {}

impl<K, V> PartialOrd for CmpByKey<K, V>
where
    K: PartialOrd,
{
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&rhs.0)
    }
}

impl<K, V> Ord for CmpByKey<K, V>
where
    K: Ord,
{
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.0.cmp(&rhs.0)
    }
}

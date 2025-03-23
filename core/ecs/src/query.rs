use crate::component::ComponentId;

/// When executing a `System` we want there to be an
/// associated `Query` that represents all the items we need to find
/// to run against.
pub struct Query {
    components: Vec<ComponentId>
}

impl Query {
    fn iter(&self) -> QueryIter<'_> {
        QueryIter { idx: 0, components: &self.components }
    }
}

pub struct QueryIter<'a> {
    idx: usize,
    components: &'a [ComponentId]
}

impl<'a> Iterator for QueryIter<'a> {
    type Item = ComponentId;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.components.len() {
            None
        } else {
            self.idx += 1;
            Some(self.components[self.idx - 1])
        }
    }
}
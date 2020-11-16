use super::errors::GraphErr;
use super::graph::Graph;
use super::models::FIELD;
use hashbrown::{HashMap, HashSet};
use std::collections::{BinaryHeap, VecDeque};
use std::fmt::Debug;

/// Iterator that owns the data.
#[derive(Debug)]
pub struct OwningIterator<'a> {
    iterable: VecDeque<FIELD>,
    cur_idx: usize, // Quite the hack, but it works
    phantom: PhantomData<&'a u8>,
}

impl<'a> OwningIterator<'a> {
    pub fn new(iterable: VecDeque<FIELD>) -> Self {
        OwningIterator {
            iterable,
            cur_idx: 0,
            phantom: PhantomData,
        }
    }
}

impl<'a> Iterator for OwningIterator<'a> {
    type Item = &'a FIELD;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_idx == self.iterable.len() {
            None
        } else {
            let last_idx = self.cur_idx;
            self.cur_idx += 1;

            // Since we cannot annotate the lifetime 'a to &mut self
            // because of the Iterator trait's signature, this seems
            // the only way to make the compiler happy.
            //
            // TODO: If you can make this work with safe Rust, please do.
            unsafe {
                let ptr = &self.iterable[last_idx] as *const FIELD;
                let transmuted = mem::transmute::<*const FIELD, &FIELD>(ptr);
                Some(transmuted)
            }
        }
    }
}

#[derive(Clone, Debug)]
/// Dijkstra Single-source Shortest Path Iterator
pub struct Dijkstra<'a> {
    source: &'a FIELD,
    iterable: &'a Graph,
    iterator: VecDeque<FIELD>,
    distances: HashMap<FIELD, i16>,
    previous: HashMap<FIELD, Option<FIELD>>,
}

pub trait MergedTrait<'a>: Iterator<Item = &'a FIELD> + Debug {}

impl<'a, T> MergedTrait<'a> for T where T: Iterator<Item = &'a FIELD> + Debug {}

/// Generic Vertex Iterator.
#[derive(Debug)]
pub struct VertexIter<'a>(pub Box<dyn 'a + MergedTrait<'a>>);

impl<'a> Iterator for VertexIter<'a> {
    type Item = &'a FIELD;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
impl<'a> Dijkstra<'a> {
    pub fn new(graph: &'a Graph, src: &'a FIELD) -> Result<Dijkstra<'a>, GraphErr> {
        if graph.fetch(src).is_none() {
            return Err(GraphErr::NoSuchVertex);
        }

        // normally at this point weights would be validated but due to them bneing irrelevant this can be ifnored

        let mut instance = Dijkstra {
            source: src,
            iterable: graph,
            iterator: VecDeque::with_capacity(graph.vertex_count()),
            distances: HashMap::with_capacity(graph.vertex_count()),
            previous: HashMap::with_capacity(graph.vertex_count()),
        };

        instance.calc_distances();

        Ok(instance)
    }

    pub fn set_source(&mut self, vert: &'a FIELD) -> Result<(), GraphErr> {
        if self.iterable.fetch(vert).is_none() {
            return Err(GraphErr::NoSuchVertex);
        }

        self.source = vert;
        self.distances.clear();
        self.previous.clear();
        self.calc_distances();

        Ok(())
    }

    pub fn get_path_to(mut self, vert: &'a FIELD) -> Result<VertexIter, GraphErr> {
        if self.iterable.fetch(vert).is_none() {
            return Err(GraphErr::NoSuchVertex);
        }

        if self.previous.contains_key(vert) {
            let mut cur_vert = Some(vert);
            self.iterator.clear();

            while cur_vert.is_some() {
                self.iterator.push_front(*cur_vert.unwrap());

                match self.previous.get(cur_vert.unwrap()) {
                    Some(v) => cur_vert = v.as_ref(),
                    None => cur_vert = None,
                }
            }

            return Ok(VertexIter(Box::new(OwningIterator::new(self.iterator))));
        }

        Ok(VertexIter(Box::new(iter::empty())))
    }

    pub fn get_distance(&mut self, vert: &'a FIELD) -> Result<i16, GraphErr> {
        if self.iterable.fetch(vert).is_none() {
            return Err(GraphErr::NoSuchVertex);
        }

        if self.distances.contains_key(vert) {
            return Ok(*self.distances.get(vert).unwrap());
        }

        Ok(i16::MAX)
    }

    fn calc_distances(&mut self) {
        let mut visited: HashSet<FIELD> = HashSet::with_capacity(self.iterable.vertex_count());
        let mut vertex_pq: BinaryHeap<VertexMeta> =
            BinaryHeap::with_capacity(self.iterable.vertex_count());

        for vert in self.iterable.vertices() {
            self.distances.insert(*vert, i16::MAX);
        }

        vertex_pq.push(VertexMeta {
            id: *self.source,
            distance: 0,
        });

        self.distances.insert(*self.source, 0);
        self.previous.insert(*self.source, None);

        while let Some(vert_meta) = vertex_pq.pop() {
            if !visited.insert(vert_meta.id) || vert_meta.id.occupied {
                continue;
            }

            for neighbor in self.iterable.out_neighbors(&vert_meta.id) {
                if !visited.contains(&neighbor) {
                    let mut alt_dist = *self.distances.get(&vert_meta.id).unwrap();

                    if let Some(w) = self.iterable.weight(&vert_meta.id, &neighbor) {
                        alt_dist += w;
                    }

                    if alt_dist < *self.distances.get(&neighbor).unwrap() {
                        self.distances.insert(*neighbor, alt_dist);
                        self.previous.insert(*neighbor, Some(vert_meta.id));

                        vertex_pq.push(VertexMeta {
                            id: *neighbor,
                            distance: alt_dist,
                        });
                    }
                }
            }
        }
    }
}

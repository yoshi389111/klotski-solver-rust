use std::collections::HashSet;
use std::hash::Hash;

#[derive(Default)]
pub struct VisitedHistory<T: Eq + Hash> {
    current: HashSet<T>,
    previous: HashSet<T>,
    pre_previous: HashSet<T>,
    depth: usize,
}

impl<T: Eq + Hash> VisitedHistory<T> {
    /// Creates a new `VisitedHistory`.
    pub fn new() -> Self {
        Self {
            current: HashSet::new(),
            previous: HashSet::new(),
            pre_previous: HashSet::new(),
            depth: 0,
        }
    }

    /// Attempts to visit the given node at the specified depth.
    ///
    /// If the node has not been visited in the current or recent generations, it is marked as visited and returns `true`.
    /// If the node has already been visited, returns `false`.
    ///
    /// # Arguments
    /// * `node` - The node to visit.
    /// * `depth` - The current search depth (used to manage generations).
    ///
    /// # Returns
    /// * `true` if the node was not visited before and is now marked as visited.
    /// * `false` if the node was already visited.
    pub fn try_visit(&mut self, node: T, depth: usize) -> bool {
        if depth != self.depth {
            self.advance_generation();
            self.depth = depth;

            if log::log_enabled!(log::Level::Debug) {
                if depth != 0 {
                    log::debug!("   count: {}", self.previous.len());
                }
                log::debug!("Depth: {depth}");
            }
        }
        if self.is_visited(&node) {
            false
        } else {
            self.mark_visited(node);
            true
        }
    }

    fn is_visited(&self, node: &T) -> bool {
        self.current.contains(node)
            || self.previous.contains(node)
            || self.pre_previous.contains(node)
    }

    fn mark_visited(&mut self, node: T) {
        self.current.insert(node);
    }

    fn advance_generation(&mut self) {
        let current = std::mem::take(&mut self.current);
        let previous = std::mem::replace(&mut self.previous, current);
        self.pre_previous = previous;
    }
}

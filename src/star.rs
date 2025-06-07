use indexmap::IndexSet;
use std::collections::HashSet;
use zellij_tile::prelude::PaneId;

/// A collection of pane IDs that are starred.
#[derive(Default)]
pub struct Star {
    pane_ids: IndexSet<PaneId>,
}

impl Star {
    /// Star a pane by adding PaneId into pane_ids.
    fn add(&mut self, pane_id: PaneId) {
        self.pane_ids.insert(pane_id);
    }

    /// Unstar a pane by removing PaneId from pane_ids.
    fn remove(&mut self, pane_id: &PaneId) {
        self.pane_ids.retain(|id| id != pane_id);
    }

    /// Remove any pane_id that is not in pane_ids.
    pub fn sync(&mut self, pane_ids: &HashSet<PaneId>) {
        self.pane_ids.retain(|id| pane_ids.contains(id));
    }

    /// Check if Star has pane_id.
    pub fn has(&self, pane_id: &PaneId) -> bool {
        self.pane_ids.contains(pane_id)
    }

    /// Add pane_id if it is not yet added
    /// and remove if it is already in the list.
    pub fn toggle(&mut self, pane_id: PaneId) {
        if self.pane_ids.contains(&pane_id) {
            self.remove(&pane_id);
        } else {
            self.add(pane_id);
        }
    }
}

// write test
#[cfg(test)]
mod tests {
    use super::*;
    use zellij_tile::prelude::PaneId;

    #[test]
    fn test_star() {
        let mut star = Star::default();

        let pane_ids: HashSet<PaneId> = [
            PaneId::Terminal(1),
            PaneId::Terminal(2),
            PaneId::Terminal(3),
        ]
        .iter()
        .cloned()
        .collect();

        star.add(PaneId::Terminal(2));
        star.add(PaneId::Terminal(10));
        star.add(PaneId::Terminal(3));
        star.add(PaneId::Terminal(2)); // Adding same pane_id again should not have any effect.
        star.toggle(PaneId::Terminal(4));
        star.toggle(PaneId::Terminal(4));

        assert_eq!(
            star.pane_ids,
            IndexSet::from([
                PaneId::Terminal(2),
                PaneId::Terminal(10),
                PaneId::Terminal(3)
            ])
        );

        star.remove(&PaneId::Terminal(2));
        star.remove(&PaneId::Terminal(2)); // Removing the same pane_id again should not have any effect.

        assert_eq!(
            star.pane_ids,
            IndexSet::from([PaneId::Terminal(10), PaneId::Terminal(3)])
        );

        star.sync(&pane_ids);

        assert_eq!(star.pane_ids, IndexSet::from([PaneId::Terminal(3)]));

        assert!(star.has(&PaneId::Terminal(3)));
        assert!(!star.has(&PaneId::Terminal(2)));
    }
}

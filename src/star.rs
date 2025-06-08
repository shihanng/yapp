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

    /// Return the next pane_id after the input.
    /// If the input does not exist, return the first pane_id.
    /// If the input is the last pane_id in the list, return the first pane_id.
    pub fn next(&self, pane_id: &PaneId) -> Option<&PaneId> {
        if self.pane_ids.is_empty() {
            return None;
        }

        let index_of = self.pane_ids.get_index_of(pane_id);
        match index_of {
            Some(index) => {
                // If the pane_id is found, return the next one, or the first if it's the last.
                let next_index = (index + 1) % self.pane_ids.len();
                self.pane_ids.get_index(next_index)
            }
            None => self.pane_ids.first(),
        }
    }

    /// Return the previous pane_id before the input.
    /// If the input does not exist, return the first pane_id.
    /// If the input is the first pane_id in the list, return the last pane_id.
    pub fn previous(&self, pane_id: &PaneId) -> Option<&PaneId> {
        if self.pane_ids.is_empty() {
            return None;
        }

        let index_of = self.pane_ids.get_index_of(pane_id);
        match index_of {
            Some(index) => {
                // If the pane_id is found, return the previous one, or the last if it's the first.
                let prev_index = if index == 0 {
                    self.pane_ids.len() - 1
                } else {
                    index - 1
                };
                self.pane_ids.get_index(prev_index)
            }
            None => self.pane_ids.first(),
        }
    }
}

// write test
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
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

    #[fixture]
    fn empty_star() -> Star {
        Star::default()
    }

    #[fixture]
    fn one_star() -> Star {
        Star {
            pane_ids: IndexSet::from([PaneId::Terminal(1)]),
        }
    }

    #[fixture]
    fn many_stars() -> Star {
        Star {
            pane_ids: IndexSet::from([
                PaneId::Terminal(1),
                PaneId::Terminal(2),
                PaneId::Terminal(3),
            ]),
        }
    }

    #[rstest]
    #[case(empty_star(), PaneId::Terminal(2), None)]
    #[case(one_star(), PaneId::Terminal(1), Some(&PaneId::Terminal(1)))]
    #[case(one_star(), PaneId::Terminal(10), Some(&PaneId::Terminal(1)))]
    #[case(many_stars(), PaneId::Terminal(10), Some(&PaneId::Terminal(1)))]
    #[case(many_stars(), PaneId::Terminal(1), Some(&PaneId::Terminal(2)))]
    #[case(many_stars(), PaneId::Terminal(3), Some(&PaneId::Terminal(1)))]
    fn next(#[case] star: Star, #[case] current_id: PaneId, #[case] expected_id: Option<&PaneId>) {
        let got = star.next(&current_id);
        assert_eq!(expected_id, got);
    }

    #[rstest]
    #[case(empty_star(), PaneId::Terminal(2), None)]
    #[case(one_star(), PaneId::Terminal(1), Some(&PaneId::Terminal(1)))]
    #[case(one_star(), PaneId::Terminal(10), Some(&PaneId::Terminal(1)))]
    #[case(many_stars(), PaneId::Terminal(10), Some(&PaneId::Terminal(1)))]
    #[case(many_stars(), PaneId::Terminal(1), Some(&PaneId::Terminal(3)))]
    #[case(many_stars(), PaneId::Terminal(3), Some(&PaneId::Terminal(2)))]
    fn eprevious(
        #[case] star: Star,
        #[case] current_id: PaneId,
        #[case] expected_id: Option<&PaneId>,
    ) {
        let got = star.previous(&current_id);
        assert_eq!(expected_id, got);
    }
}

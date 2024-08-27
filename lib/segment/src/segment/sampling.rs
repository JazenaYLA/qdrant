use std::sync::atomic::{AtomicBool, Ordering};

use common::iterator_ext::IteratorExt;
use rand::seq::{IteratorRandom, SliceRandom};

use super::Segment;
use crate::index::PayloadIndex;
use crate::types::{Filter, PointIdType};

impl Segment {
    pub(super) fn filtered_read_by_index_shuffled(
        &self,
        limit: usize,
        condition: &Filter,
        is_stopped: &AtomicBool,
    ) -> Vec<PointIdType> {
        let payload_index = self.payload_index.borrow();
        let id_tracker = self.id_tracker.borrow();

        let cardinality_estimation = payload_index.estimate_cardinality(condition);
        let ids_iterator = payload_index
            .iter_filtered_points(condition, &*id_tracker, &cardinality_estimation)
            .check_stop(|| is_stopped.load(Ordering::Relaxed))
            .filter_map(|internal_id| id_tracker.external_id(internal_id));

        let mut rng = rand::thread_rng();
        let mut shuffled = ids_iterator.choose_multiple(&mut rng, limit);
        shuffled.shuffle(&mut rng);

        shuffled
    }

    pub fn filtered_read_by_random_stream(
        &self,
        limit: usize,
        condition: &Filter,
        is_stopped: &AtomicBool,
    ) -> Vec<PointIdType> {
        let payload_index = self.payload_index.borrow();
        let filter_context = payload_index.filter_context(condition);
        self.id_tracker
            .borrow()
            .iter_random()
            .check_stop(|| is_stopped.load(Ordering::Relaxed))
            .filter(move |(_, internal_id)| filter_context.check(*internal_id))
            .map(|(external_id, _)| external_id)
            .take(limit)
            .collect()
    }

    pub(super) fn read_by_random_id(&self, limit: usize) -> Vec<PointIdType> {
        self.id_tracker
            .borrow()
            .iter_random()
            .map(|x| x.0)
            .take(limit)
            .collect()
    }
}
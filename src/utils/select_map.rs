use core::ops::Fn;
use core::pin::Pin;
use futures::stream::{Fuse, FusedStream, Stream, StreamExt};
use futures::{Future, FutureExt};
use futures_core::task::{Context, Poll};
use pin_project_lite::pin_project;

#[cfg(test)]
#[path = "./select_map_test.rs"]
pub mod select_map_test;

pub trait SelectMapExt: Stream {
    fn select_map<S2, M, Fut>(self, map: M) -> SelectMap<Self, S2, M, Fut>
    where
        S2: Stream<Item = Self::Item> + Unpin,
        M: Fn(&Self::Item) -> Fut,
        Fut: Future<Output = Option<S2>> + Unpin,
        Self: Sized,
    {
        let minimal_primaries = self.size_hint().0;
        SelectMap {
            primary_stream: self.fuse(),
            pending_secondary_streams: Vec::with_capacity(minimal_primaries),
            secondary_streams: Vec::with_capacity(minimal_primaries),
            map,
        }
    }
}

impl<T: ?Sized> SelectMapExt for T where T: Stream {}

pin_project! {
#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct SelectMap<S1, S2, M, Fut> {
    #[pin]
    primary_stream: Fuse<S1>,
    pending_secondary_streams: Vec<Fut>,
    secondary_streams: Vec<Fuse<S2>>,
    map: M,
}
}

impl<S1, S2, M, Fut> Stream for SelectMap<S1, S2, M, Fut>
where
    S1: Stream,
    S2: Stream<Item = S1::Item> + Unpin,
    M: Fn(&S1::Item) -> Fut,
    Fut: Future<Output = Option<S2>> + Unpin,
{
    type Item = S1::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        let mut all_done = match this.primary_stream.poll_next(cx) {
            Poll::Ready(Some(item)) => {
                this.pending_secondary_streams.push((this.map)(&item));
                return Poll::Ready(Some(item));
            }
            Poll::Ready(None) => true,
            Poll::Pending => false,
        };

        let mut non_pending_secondary_streams = Vec::new();
        for (pending_index, pending_secondary_stream) in
            this.pending_secondary_streams.iter_mut().enumerate()
        {
            match pending_secondary_stream.poll_unpin(cx) {
                Poll::Ready(Some(secondary_stream)) => {
                    non_pending_secondary_streams.push(pending_index);
                    this.secondary_streams.push(secondary_stream.fuse())
                }
                Poll::Ready(None) => non_pending_secondary_streams.push(pending_index),
                Poll::Pending => all_done = false,
            }
        }
        clean_vec(
            this.pending_secondary_streams,
            non_pending_secondary_streams,
        );

        let mut empty_secondary_streams = Vec::new();
        for (stream_index, secondary_stream) in this.secondary_streams.iter_mut().enumerate() {
            match secondary_stream.poll_next_unpin(cx) {
                Poll::Ready(Some(item)) => {
                    clean_vec(this.secondary_streams, empty_secondary_streams);
                    return Poll::Ready(Some(item));
                }
                Poll::Ready(None) => empty_secondary_streams.push(stream_index),
                Poll::Pending => all_done = false,
            }
        }
        clean_vec(this.secondary_streams, empty_secondary_streams);

        if all_done {
            return Poll::Ready(None);
        }
        Poll::Pending
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (
            self.primary_stream.size_hint().0.saturating_add(
                self.secondary_streams
                    .iter()
                    .map(|secondary_stream| secondary_stream.size_hint().0)
                    .sum(),
            ),
            None,
        )
    }
}

impl<S1, S2, M, Fut> FusedStream for SelectMap<S1, S2, M, Fut>
where
    S1: Stream,
    S2: Stream<Item = S1::Item> + Unpin,
    M: Fn(&S1::Item) -> Fut,
    Fut: Future<Output = Option<S2>> + Unpin,
{
    fn is_terminated(&self) -> bool {
        self.primary_stream.is_terminated()
            && self.pending_secondary_streams.is_empty()
            && self
                .secondary_streams
                .iter()
                .all(|secondary_stream| secondary_stream.is_terminated())
    }
}

/// `indexes` have to be valid and sorted ascending
fn clean_vec<T>(vec: &mut Vec<T>, indexes: Vec<usize>) {
    for index in indexes.into_iter().rev() {
        drop(vec.remove(index));
    }
}

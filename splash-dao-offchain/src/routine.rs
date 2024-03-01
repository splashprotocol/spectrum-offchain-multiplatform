use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use async_stream::stream;
use either::Either;
use futures::{Stream, StreamExt};
use futures::channel::mpsc;
use futures::stream::FusedStream;
use futures_timer::Delay;

use crate::time::{NetworkTime, NetworkTimeProvider};

#[async_trait::async_trait]
pub trait Attempt<T, Tr> {
    /// Make an attempt to execute next step of the routine.
    /// Returns timestamp when the next attempt should be performed on failure.
    async fn attempt(self) -> Result<Option<Tr>, NetworkTime>;
}

#[inline]
pub fn transit<T>(tx: T) -> Result<Option<T>, NetworkTime> {
    Ok(Some(tx))
}

#[inline]
pub fn postpone<T>(t: NetworkTime) -> Result<Option<T>, NetworkTime> {
    Err(t)
}

#[async_trait::async_trait]
pub trait ApplyEvent<E> {
    async fn apply_event(&self, event: E);
}

#[async_trait::async_trait]
pub trait ApplyTransition<Tr> {
    async fn apply_tr(&self, tr: Tr);
}

#[async_trait::async_trait]
pub trait StateRead<S> {
    async fn read_state(&self) -> S;
}

pub struct Routine<S, T, E, C> {
    inbox: mpsc::Receiver<E>,
    waker: Option<Delay>,
    behaviour: S,
    ntp: T,
    context: C,
}

impl<P, T, E, C> Routine<P, T, E, C> {
    async fn postpone_until(&mut self, until: NetworkTime)
    where
        T: NetworkTimeProvider,
    {
        let now = self.ntp.network_time().await;
        let delay = until - now;
        let _ = self.waker.insert(Delay::new(Duration::from_millis(delay)));
    }
}

pub struct Wake;

impl<P, T, E, C> Stream for Routine<P, T, E, C>
where
    P: Unpin,
    T: Unpin,
    E: Unpin,
    C: Unpin,
{
    type Item = Either<E, Wake>;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        loop {
            if let Poll::Ready(Some(event)) = Stream::poll_next(Pin::new(&mut self.inbox), cx) {
                return Poll::Ready(Some(Either::Left(event)));
            }
            if let Some(mut timer) = self.waker.take() {
                match Future::poll(Pin::new(&mut timer), cx) {
                    Poll::Ready(_) => return Poll::Ready(Some(Either::Right(Wake))),
                    _ => {
                        let _ = self.waker.insert(timer);
                    }
                }
            }
            break;
        }
        Poll::Pending
    }
}

impl<P, T, E, C> FusedStream for Routine<P, T, E, C>
where
    P: Unpin,
    T: Unpin,
    E: Unpin,
    C: Unpin,
{
    fn is_terminated(&self) -> bool {
        false
    }
}

pub fn routine<B, T, E, C, S, Tr>(mut this: Routine<B, T, E, C>) -> impl Stream<Item = ()>
where
    B: Unpin + ApplyEvent<E> + ApplyTransition<Tr> + StateRead<S>,
    T: Unpin + NetworkTimeProvider,
    E: Unpin,
    C: Unpin + Copy,
    S: Attempt<T, Tr>,
{
    stream! {
        loop {
            // We prioritize external updates.
            if let Either::Left(event) = this.select_next_some().await {
                this.behaviour.apply_event(event).await;
            }
            let state = this.behaviour.read_state().await;
            match state.attempt().await {
                Ok(Some(tr)) => this.behaviour.apply_tr(tr).await,
                Err(postpone_until) => this.postpone_until(postpone_until).await,
                _ => {}
            }
        }
    }
}

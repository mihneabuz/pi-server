use std::{
    future::Future,
    pin::Pin,
    sync::atomic::Ordering,
    task::{ready, Context, Poll},
};

use axum::{http::Request, response::Response};
use pin_project_lite::pin_project;
use tower::{Layer, Service};

use crate::statistics;

use super::Middleware;

#[derive(Clone)]
struct StatsService<S> {
    inner: S,
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for StatsService<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = StatsServiceFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        StatsServiceFuture {
            inner: self.inner.call(req),
        }
    }
}

pin_project! {
    struct StatsServiceFuture<F> {
        #[pin]
        inner: F,
    }
}

impl<F, ResBody, E> Future for StatsServiceFuture<F>
where
    F: Future<Output = Result<Response<ResBody>, E>>,
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let response = ready!(this.inner.poll(cx))?;

        statistics::SERVED.fetch_add(10, Ordering::Relaxed);

        Poll::Ready(Ok(response))
    }
}

#[derive(Clone)]
struct StatsLayer;
impl<S> Layer<S> for StatsLayer {
    type Service = StatsService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        StatsService { inner }
    }
}

pub struct Stats;

impl Middleware for Stats {
    fn attach<S>(self, router: axum::Router<S>) -> axum::Router<S>
    where
        S: Clone + Send + Sync + 'static,
    {
        router.layer(StatsLayer)
    }
}

use axum::{routing::MethodRouter, Router};

pub trait RouterExt<S> {
    fn route_iter<I>(self, iter: I) -> Self
    where
        I: Iterator<Item = (String, MethodRouter<S>)>;
}

impl<S> RouterExt<S> for Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    fn route_iter<I>(mut self, iter: I) -> Self
    where
        I: Iterator<Item = (String, MethodRouter<S>)>,
    {
        for (path, handler) in iter.into_iter() {
            self = self.route(&path, handler);
        }

        self
    }
}

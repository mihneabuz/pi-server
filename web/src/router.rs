use axum::{routing::MethodRouter, Router};

use crate::pages::DynamicModule;

pub trait RouterExt<S> {
    fn route_iter<I>(self, iter: I) -> Self
    where
        I: Iterator<Item = (String, MethodRouter<S>)>;

    fn merge_iter<I>(self, iter: I) -> Self
    where
        I: Iterator<Item = Router<S>>;

    fn merge_module<M>(self, module: M) -> Self
    where
        M: DynamicModule<S>;

    fn merge_modules<I, M>(self, iter: I) -> Self
    where
        M: DynamicModule<S>,
        I: Iterator<Item = M>;
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

    fn merge_iter<I>(mut self, iter: I) -> Self
    where
        I: Iterator<Item = Router<S>>,
    {
        for router in iter.into_iter() {
            self = self.merge(router);
        }

        self
    }

    fn merge_module<M>(self, module: M) -> Self
    where
        M: DynamicModule<S>,
    {
        self.merge(module.app())
    }

    fn merge_modules<I, M>(self, iter: I) -> Self
    where
        M: DynamicModule<S>,
        I: Iterator<Item = M>,
    {
        self.merge_iter(iter.map(|module| module.app()))
    }
}

use ntex::service::{Middleware, Service, ServiceCtx};
use ntex::web;
use fastrace::prelude::*;

pub struct Tracer;

impl<S> Middleware<S> for Tracer {
    type Service = TracerMiddleware<S>;

    fn create(&self, service: S) -> Self::Service {
        TracerMiddleware { service }
    }
}

pub struct TracerMiddleware<S> {
    service: S,
}

impl<S, Err> Service<web::WebRequest<Err>> for TracerMiddleware<S>
where
    S: Service<web::WebRequest<Err>, Response = web::WebResponse, Error = web::Error>,
    Err: web::ErrorRenderer,
{
    type Response = web::WebResponse;
    type Error = web::Error;

    ntex::forward_ready!(service);

    async fn call(
        &self,
        req: web::WebRequest<Err>,
        ctx: ServiceCtx<'_, Self>,
    ) -> Result<Self::Response, Self::Error> {
        let method = req.method().to_string();
        let path = req.path().to_string();
        
        let span_name = format!("{} {}", method, path);

        let root = Span::root(span_name, SpanContext::random());
        let guard = root.set_local_parent();
        
        let res = ctx.call(&self.service, req).await?;
        
        drop(guard);
        
        fastrace::flush();

        Ok(res)
    }
}
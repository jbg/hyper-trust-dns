use std::{
    future::Future,
    io::Error,
    net::IpAddr,
    pin::Pin,
    vec,
};

use futures::{
    compat::Future01CompatExt,
    future::TryFutureExt,
};
use hyper::client::connect::dns::{Name, Resolve};
use trust_dns_resolver::AsyncResolver;


#[derive(Clone)]
pub struct HyperTrustDnsResolver(AsyncResolver);

impl HyperTrustDnsResolver {
    pub fn new(resolver: AsyncResolver) -> Self {
        HyperTrustDnsResolver(resolver)
    }
}

impl Resolve for HyperTrustDnsResolver {
    type Addrs = vec::IntoIter<IpAddr>;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Addrs, Error>> + Send>>;

    fn resolve(&self, name: Name) -> Self::Future {
        Box::pin(
            self.0
                .lookup_ip(name.as_str())
                .compat()
                .map_ok(|res|
                    res.iter()
                       .collect::<Vec<_>>()
                       .into_iter()
                )
                .map_err(|e| e.into())
        )
    }
}

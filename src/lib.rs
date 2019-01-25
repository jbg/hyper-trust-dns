use std::io::Error;
use std::net::IpAddr;
use std::vec;

use hyper::client::connect::dns::{Name, Resolve};
use hyper::rt::Future;
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
    type Future = Box<Future<Item=Self::Addrs, Error=Error> + Send>;

    fn resolve(&self, name: Name) -> Self::Future {
        Box::new(
            self.0
                .lookup_ip(name.as_str())
                .map(|res|
                    res.iter()
                       .collect::<Vec<_>>()
                       .into_iter()
                )
                .map_err(|e| e.into())
        )
    }
}
# hyper-trust-dns

A simple integration between [hyper](https://github.com/hyperium/hyper) and the [trust-dns](https://github.com/bluejekyll/trust-dns) resolver.

If you just want an HTTP(S) client, you probably want [reqwest](https://github.com/seanmonstar/reqwest), which has trust-dns support.

If you need to use hyper directly for any reason, this crate allows you to use trust-dns as the DNS resolver.

In this example, [hyper-rustls](https://github.com/ctz/hyper-rustls) is also used for TLS support:

```rust
use hyper::{client::HttpConnector, Client};
use hyper_rustls::HttpsConnector;
use hyper_trust_dns::HyperTrustDnsResolver;
use rustls::ClientConfig;
use tokio::runtime::Runtime;
use trust_dns_resolver::AsyncResolver;


let mut rt = Runtime::new().expect("failed to init tokio runtime");
let (resolver, background) = AsyncResolver::from_system_conf().expect("failed to init dns resolver");
rt.spawn(background);

let hyper_resolver = HyperTrustDnsResolver::new(resolver);
let mut http_connector = HttpConnector::new_with_resolver(hyper_resolver);
http_connector.enforce_http(false);

let mut client_config = ClientConfig::new();
client_config.root_store.add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);
client_config.ct_logs = Some(&ct_logs::LOGS);

let https_connector = HttpsConnector::from((http_connector, client_config));

let client = Client::builder()
                    .executor(rt.executor())
                    .build(https_connector);
```

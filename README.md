NeonRabbitGW
-------------------

A common micro-service API gateway, support normal http/rpc service, can support dapr framework also. </br>
Production use in project [NeonRabbit](https://github.com/halokid/NeonRabbit) 

# Feature

- [x] support invoke Dapr service 
- [x] support Dapr sidebar, single two models
- [x] Dapr service auto descovery
- [ ] support HTTP/HTTP2 proxy
- [ ] support websockety
- [ ] support traffic cache 
- [ ] rate limit
- [ ] security configuration, ip whitelist, ip block, url block for ip etc.
- [ ] support plugin system



# Usage
running model is config by `config.rs` model setting
```shell
# single model: connect to regiter centre directly, not depend on dapr
# can not tracking the Gateway service in Dapr default `zipkin`, add tracking
# process by `NeonRabbitGW` self
cargo run

# dapr model:
dapr.exe run --app-id neon_gw  --app-protocol http  --app-port 8080  --dapr-http-port 4500   -- cargo.exe run

```

# Compnent
when developing, need run the compnent of project

```shell
# Consul
consul.exe agent -dev -ui -client 0.0.0.0

# Zipkin
java -jar zipkin-server-2.24.0-exec.jar
```




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


# Call Sample

```shell

# gw -->>> schedule -->>> sp
http://localhost:8080/neon_schedule/job

```

# Devops

```shell

# Jenkis
java -jar jenkins-cli.jar -s http://xxxxxx:8080/ -auth xx:xxxxx  delete-builds xxxxx 0-40

# Docker Registry
curl -X GET http://xxxxxxxx:5000/v2/_catalog
curl -X GET http://xxxxxxx:5000/v2/neon-rabbit-gw/tags/list

# K8s
kubectl create namespace neon

# Running Dapr Sidecar with a Custom Configuration File
dapr run --app-id myapp --config ./path/to/your/custom-config.yaml


```


# Front
```shell 
# cuz Dapr has dashboard,  has some functions can manage service, so can use to instead of this frontend
cd neongw-frontend

yarn start

```



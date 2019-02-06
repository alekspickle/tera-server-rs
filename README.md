## Simple server to learn rust

listenfd - this is crate for autoreload server during development

so the run script is a bit different:

```bash
 systemfd --no-pid -s http::3000 -- cargo watch -x run 
 ```


and you can see your changes without manually recompiling server.
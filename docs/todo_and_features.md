<!-- START OF TOC !DO NOT EDIT THIS CONTENT MANUALLY-->
**Table of Contents**  *generated with [mtoc](https://github.com/containerscrew/mtoc)*
- [List of TODO and comming features](#list-of-todo-and-comming-features)
<!-- END OF TOC -->
# List of TODO and comming features

List of things to improve and comming features:

- [1] Prometheus metrics
- [2] Log domain endpoint using uprobes and openssl
- [3] Monitor networking in cgroups (for containers, like podman, docker...)
- [4] SSL/TLS http(s) sniffing usind uprobes/ureprobes (openssl)
- [5] Add filters: user can filter by protocol, dest ip ...etc
- [6] Multiple interface attachment. Early in development, in order to get a simple first version, I implemented nflux so that it could attach programs to multiple system interfaces. For simplicity in the code and log management, I decided to remove it.
- [7] TESTING!
- [8] Ipv6 traffic
- [9] Autodetect openssl nss libraries in `nflux tlstrace`

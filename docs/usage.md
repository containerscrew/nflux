<!-- START OF TOC !DO NOT EDIT THIS CONTENT MANUALLY-->
**Table of Contents**  *generated with [mtoc](https://github.com/containerscrew/mtoc)*
- [Usage](#usage)
  - [netrace](#netrace)
    - [Changing default interface](#changing-default-interface)
    - [Sniffing (only) egress traffic](#sniffing-only-egress-traffic)
    - [Sniffing (only) ingress traffic](#sniffing-only-ingress-traffic)
    - [Packet logging](#packet-logging)
    - [Available procotols](#available-procotols)
  - [tlstrace](#tlstrace)
    - [Openssl](#openssl)
    - [Example using tlstrace](#example-using-tlstrace)
<!-- END OF TOC -->
# Usage

Global flags:

```shell
sudo nflux --help
sudo nflux --log-level warn
sudo nflux --log-format json
sudo nflux netrace --help
sudo nflux tlstrace --help
```

Then:

```shell
sudo nflux netrace FLAGS
# Or
sudo nflux tlstrace FLAGS
```

## netrace

By default, everything is enabled. Which means:

- Egress traffic
- Ingress traffic
- UDP/TCP/ICMP protocols
- Full packet logging

Let's see in the following sections how to customize `nflux netrace`.

### Changing default interface

_The program automatically detects your default iface._

```shell
sudo nflux netrace -i eth0
```

### Sniffing (only) egress traffic

```shell
sudo nflux netrace --disable-ingress
```

### Sniffing (only) ingress traffic

```shell
sudo nflux netrace --enable-egress
```

### Packet logging

By default `nflux netrace` will log **all packets** (egress/ingress) entering the NIC (Network Interface). If you use `--disable-full-log`, you can use `--log-interval` to set the time interval in which the same `ip->port` connection will be logged.

For example:

```shell
ping 1.1.1.1
```

Every packet of type `icmp` to the ip `1.1.1.1` will be logged in the terminal.

Or:

```shell
curl http://external-ip
```

So, if you don't want to log every packet, you can should run the command:

```shell
sudo nlux netrace --disable-full-log # default to 5 seconds
```

Or you can also change the `--log-interval`:

```shell
sudo nflux netrace --disable-full-log --log-inveral 3 # every 3 seconds
```

### Available procotols

`UDP/TCP/ICMP` available by default.

To disable protocols like `udp`, `icmp`, `tcp`:

```shell
sudo nflux netrace --disable-udp --disable-icmp --disable-tcp
```

## tlstrace

```shell
+-----------------------------------------------+
|                 Application                   |
|      (e.g. Web Browser, Client Software)      |
+--------------------+--------------------------+
|      write()      |         read()            |
|        ↓          |          ↑                |
+--------------------+--------------------------+
|                TLS Library                    |
|       (e.g., libssl.so, OpenSSL)              |
+--------------------+--------------------------+
|     SSL_write()   |       SSL_read()          |
|        ↓          |          ↑                |
+--------------------+--------------------------+
|              Linux Kernel                     |
+--------------------+--------------------------+
|      send()       |         recv()            |
|        ↓          |          ↑                |
+-----------------------------------------------+
```

`tlstrace` implementations:

- Openssl ✅
- NSS ❌
- Boring SSL ❌

### Openssl

Before running `tlstrace` run the following command:

```shell
$ ldconfig -p | grep libssl
        libssl3.so (libc6,x86-64) => /lib64/libssl3.so
        libssl3.so (libc6) => /lib/libssl3.so
        libssl.so.3 (libc6,x86-64) => /lib64/libssl.so.3
        libssl.so.3 (libc6) => /lib/libssl.so.3
        libssl.so (libc6,x86-64) => /lib64/libssl.so
```

In this previous command, `/lib64/libssl.so` is the library that `tlstrace` needs for my fedora machine. The path changes depending on your distro.

> **/lib64/libssl.so** is the default path. Not necessary to specify `--openssl-path` flag.

Specifying other path:

```shell
sudo nflux tlstrace --openssl-path '/other/path'
```

Verify `SSL_read` and `SSL_write` exists:

```shell
nm -D /lib64/libssl.so | grep SSL_write
```

### Example using tlstrace

`curl` uses openssl by default to encrypt/decrypt the data it sends.

```shell
$ curl -V

curl 8.9.1 (x86_64-redhat-linux-gnu) libcurl/8.9.1 OpenSSL/3.2.4 zlib/1.3.1.zlib-ng libidn2/2.3.8 nghttp2/1.62.1
Release-Date: 2024-07-31
Protocols: file ftp ftps http https ipfs ipns
Features: alt-svc AsynchDNS GSS-API HSTS HTTP2 HTTPS-proxy IDN IPv6 Kerberos Largefile libz SPNEGO SSL threadsafe UnixSockets
```

Run the following command:

```shell
curl https://iproxy.containerscrew.com/me --http1.1
```

> curl without specifying --http1.1 uses http2. In the log you will see data encrypted with the HPACK algorithm.

<!-- START OF TOC !DO NOT EDIT THIS CONTENT MANUALLY-->
**Table of Contents**  *generated with [mtoc](https://github.com/containerscrew/mtoc)*
- [Usage](#usage)
  - [Changing default interface](#changing-default-interface)
  - [Sniffing (only) egress traffic](#sniffing-only-egress-traffic)
  - [Sniffing (only) ingress traffic](#sniffing-only-ingress-traffic)
  - [Packet logging](#packet-logging)
  - [Listen specific port](#listen-specific-port)
  - [Exclude ports from the log](#exclude-ports-from-the-log)
  - [Available procotols](#available-procotols)
  - [Enable timestamp in log output](#enable-timestamp-in-log-output)
  - [Examples](#examples)
<!-- END OF TOC -->
# Usage

Global flags:

```shell
sudo nflux --help
sudo nflux --log-level warn
sudo nflux --log-format json
```

Then:

```shell
sudo nflux FLAGS
```

By default, everything is enabled. Which means:

- Egress traffic
- Ingress traffic
- UDP/TCP/ICMP protocols
- Full packet logging

Let's see in the following sections how to customize `nflux`.

## Changing default interface

_The program automatically detects your default iface._

```shell
sudo nflux -i eth0
```

## Sniffing (only) egress traffic

```shell
sudo nflux --disable-ingress
```

## Sniffing (only) ingress traffic

```shell
sudo nflux --disable-egress
```

## Packet logging

By default `nflux` will log **all packets** (egress/ingress) entering the NIC (Network Interface or virtual interface (like tun0)). If you use `--disable-full-log`, you can use `--log-interval` to set the time interval in which the same `ip->port` connection will be logged.

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
sudo nflux --disable-full-log # default to 5 seconds
```

Or you can also change the `--log-interval`:

```shell
sudo nflux --disable-full-log --log-inveral 3 # every 3 seconds
```

## Listen specific port

Example, only listening port `3306`:

```shell
sudo nflux --listen-port 3306
```

## Exclude ports from the log

```shell
sudo nflux --exclude-ports 22,443
```

## Available procotols

`UDP/TCP/ICMP` available by default.

To disable protocols like `udp`, `icmp`, `tcp`:

```shell
sudo nflux --disable-udp
sudo nflux --disable-icmp
sudo nflux --disable-tcp
```

## Enable timestamp in log output

```shell
sufo nflux --disable-ingress --with-timer
```

> Timestamp is disabled in the logger by default for brevity

## Examples

```shell
sudo nflux --disable-full-log --log-interval 10 --exclude-ports 22 --disable-udp --with-timer
sudo nflux --disable-egress --with-timer
```

<!-- START OF TOC !DO NOT EDIT THIS CONTENT MANUALLY-->
**Table of Contents**  *generated with [mtoc](https://github.com/containerscrew/mtoc)*
<!-- END OF TOC -->
The main idea of this tool was to be able to create a basic firewall and a tool for network monitoring, using XDP and TC. However, due to the complexity involved in well implementing a firewall to control TCP connections (and its SYN/ACK/FIN/RST...etc states) among others, nflux will become a CLI to monitor eggress/ingress connections using TC. As a first introduction, then perhaps more features can be added.

Name:           nflux
Version:        0.1.0
Release:        1%{?dist}
Summary:        eBPF network monitoring tool 🐝

License:        MIT
URL:            https://github.com/tu-usuario/nflux
Source0:        %{name}-%{version}.tar.gz

BuildArch:      x86_64
Requires:       iproute, libbpf, bpftool

%description
eBPF network monitoring tool 🐝
It is powered by Aya-rs and provides high-performance network observability.

%prep
%setup -q

%build
cargo build --release --package nflux

%install
mkdir -p %{buildroot}/usr/bin
mkdir -p %{buildroot}/usr/share/doc/%{name}
mkdir -p %{buildroot}/usr/share/man/man1

install -m 0755 target/release/nflux %{buildroot}/usr/bin/nflux
install -m 0644 README.md LICENSE %{buildroot}/usr/share/doc/%{name}/
install -m 0644 nflux.1 %{buildroot}/usr/share/man/man1/nflux.1
gzip -9 %{buildroot}/usr/share/man/man1/nflux.1

%files
%license LICENSE
%doc README.md
/usr/bin/nflux
/usr/share/doc/%{name}/README.md
/usr/share/man/man1/nflux.1.gz

%changelog
* Mon Apr 01 2025 Daniels <info@containerscrew.com> - 0.1.0-1
- First release of nflux

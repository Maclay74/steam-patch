%global _name   steam-patch

Name:           steam-patch
Version:        1.0.0
Release:        1%{?dist}
Summary:        Steam Patch for ASUS ROG ALLY face buttons, tdp and GPU clock control

License:        GPL3
URL:            https://github.com/corando98/steam-patch
Source0:        steam-patch-main.zip
Source1:        steam-patch.service
Source2:        restart-steam-patch-on-boot.service
Source3:        steamos-priv-write-updated

BuildRequires:  cargo rust
Recommends:     steam gamescope-session
Provides:       steam-patch
Conflicts:      steam-patch

%description
Steam Patch for ASUS ROG ALLY

%prep
rm -rf %{_builddir}/steam-patch
cd $RPM_SOURCE_DIR
rm -f steam-patch-main.zip
wget https://github.com/corando98/steam-patch/archive/refs/heads/main.zip
mv main.zip steam-patch-main.zip
unzip $RPM_SOURCE_DIR/steam-patch-main.zip -d %{_builddir}
mkdir -p %{_builddir}/steam-patch
cp -rf %{_builddir}/steam-patch-main/* %{_builddir}/steam-patch
rm -rf %{_builddir}/steam-patch-main
cp -f %{_builddir}/steam-patch/{steam-patch.service,restart-steam-patch-on-boot.service} $RPM_SOURCE_DIR

%build
cd %{_builddir}/steam-patch
cargo build -r

%install
mkdir -p %{buildroot}/usr/bin
cp %{_builddir}/steam-patch/target/release/steam-patch %{buildroot}/usr/bin/steam-patch

mkdir -p %{buildroot}/etc/systemd/system/
mkdir -p %{buildroot}/usr/bin/steamos-polkit-helpers/

install -m 644 %{SOURCE1} %{buildroot}/etc/systemd/system/
install -m 644 %{SOURCE2} %{buildroot}/etc/systemd/system/
install -m 747 %{SOURCE3} %{buildroot}/usr/bin/steamos-polkit-helpers/

%post
sed -i "s/USER/${SUDO_USER}/g" /etc/systemd/system/steam-patch.service
sed -i 's/\$//g' /etc/systemd/system/steam-patch.service
systemctl daemon-reload
systemctl enable steam-patch.service
systemctl start steam-patch.service
systemctl enable restart-steam-patch-on-boot.service
systemctl start restart-steam-patch-on-boot.service
mv /usr/bin/steamos-polkit-helpers/steamos-priv-write /usr/bin/steamos-polkit-helpers/steamos-priv-write-bkp
mv /usr/bin/steamos-polkit-helpers/steamos-priv-write-updated /usr/bin/steamos-polkit-helpers/steamos-priv-write

%preun
systemctl stop steam-patch.service
systemctl disable steam-patch.service
systemctl stop restart-steam-patch-on-boot.service
systemctl disable restart-steam-patch-on-boot.service
systemctl daemon-reload
mv /usr/bin/steamos-polkit-helpers/steamos-priv-write /usr/bin/steamos-polkit-helpers/steamos-priv-write-updated
mv /usr/bin/steamos-polkit-helpers/steamos-priv-write-bkp /usr/bin/steamos-polkit-helpers/steamos-priv-write

%files
/etc/systemd/system/steam-patch.service
/etc/systemd/system/restart-steam-patch-on-boot.service
/usr/bin/steam-patch
/usr/bin/steamos-polkit-helpers/steamos-priv-write-updated

%changelog
* Fri Nov 03 2023 Diego Garcia <diegocorando@gmail.com> [1.0.0-1]
- Initial package

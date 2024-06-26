%define __spec_install_post %{nil}
%define __os_install_post %{_dbpath}/brp-compress
%define debug_package %{nil}

Name: river-bsp-layout
Summary: Binary space partitioned layout for the tiling Wayland compositor River.
Version: @@VERSION@@
Release: @@RELEASE@@%{?dist}
License: gpl-3.0
Group: Applications/System
Source0: %{name}-%{version}.tar.gz
URL: https://github.com/areif-dev/river-bsp-layout

BuildRoot: %{_tmppath}/%{name}-%{version}-%{release}-root

%description
%{summary}

%prep
%setup -q

%install
rm -rf %{buildroot}
mkdir -p %{buildroot}
cp -a * %{buildroot}

%clean
rm -rf %{buildroot}

%files
%defattr(-,root,root,-)
%{_bindir}/*

Name:           waylyrics
Version:        {{{ git_repo_version }}}
Release:        0
Summary:        Simple desktop lyrics made with GTK4 and love.
License:        MIT
Group:          Productivity/Multimedia/Sound/Utilities
Url:            https://github.com/waylyrics/waylyrics
Source0:        https://github.com/waylyrics/waylyrics/archive/master.tar.gz
BuildRequires:  cargo >= 1.73.0
BuildRequires:  libgraphene-devel
BuildRequires:  gtk4-devel
BuildRequires:  openssl-devel
BuildRequires:  dbus-1-devel
BuildRequires:  mimalloc-devel

%define debug_package %{nil}

%description
Simple universal on screen lyrics made with GTK4 and love.

%prep
%setup -q -n waylyrics-master
cargo --version

%build
export WAYLYRICS_THEME_PRESETS_DIR=%{_datadir}/waylyrics/themes
cargo build --release --locked --target-dir target

%install
export WAYLYRICS_THEME_PRESETS_DIR=%{_datadir}/waylyrics/themes
cargo install --path . --root=%{buildroot}%{_prefix}

install -Dm644 "io.poly000.waylyrics.desktop" -t %{buildroot}%{_datadir}/applications/
install -d %{buildroot}%{_datadir}/waylyrics

install -Dm644 io.poly000.waylyrics.gschema.xml %{buildroot}%{_datadir}/glib-2.0/schemas/

cp -r themes %{buildroot}%{_datadir}/waylyrics/

rm %{buildroot}/usr/.crates.toml %{buildroot}/usr/.crates2.json

%check

%files
%{_bindir}/waylyrics
%{_datadir}/applications/io.poly000.waylyrics.desktop
%{_datadir}/waylyrics/
%{_datadir}/glib-2.0/schemas/io.poly000.waylyrics.gschema.xml

%changelog
{{{ git_repo_changelog }}}
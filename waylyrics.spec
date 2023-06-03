Name:           waylyrics
Version:        {{{ git_repo_version }}}
Release:        0
Summary:        Simple universal on screen lyrics made with GTK4 and love.
License:        MIT
Group:          Productivity
Url:            https://github.com/poly000/waylyrics
Source0:        https://github.com/poly000/waylyrics/archive/master.tar.gz
BuildRequires:  cargo
BuildRequires:  libgraphene-devel
BuildRequires:  gtk4-devel
BuildRequires:  openssl-devel
BuildRequires:  dbus-1-devel

%define debug_package %{nil}

%description
Simple universal on screen lyrics made with GTK4 and love.

%prep
%setup -q -n waylyrics-master
cargo --version

%build
export RUSTC_BOOTSTRAP=1
export WAYLYRICS_DEFAULT_CONFIG=/usr/share/waylyrics/config.toml
export WAYLYRICS_THEME_PRESETS_DIR=/usr/share/waylyrics/themes
cargo build --release --locked --target-dir target

%install
export RUSTC_BOOTSTRAP=1
export WAYLYRICS_DEFAULT_CONFIG=/usr/share/waylyrics/config.toml
export WAYLYRICS_THEME_PRESETS_DIR=/usr/share/waylyrics/themes
cargo install --path . --root=%{buildroot}%{_prefix}
cargo run --bin gen_config_example
install -m644 config.toml %{buildroot}/usr/share/waylyrics/config.toml
install -m644 io.poly000.waylyrics.gschema.xml %{buildroot}/usr/share/glib-2.0/schemas/
install -dm755 %{buildroot}/usr/share/waylyrics/themes
cp -r themes/* %{buildroot}/usr/share/waylyrics/themes/

%check

%files
%{_bindir}/waylyrics
/usr/share/waylyrics/
/usr/share/glib-2.0/schemas/
/usr/share/waylyrics/themes/

%changelog
{{{ git_repo_changelog }}}
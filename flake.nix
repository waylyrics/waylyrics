{
  description = "Waylyrics: the furry way to show desktop lyrics.";
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.naersk.url = "github:nix-community/naersk";

  outputs = {nixpkgs, flake-utils, naersk, ...}:
    flake-utils.lib.eachDefaultSystem (system:
    let pkgs = import nixpkgs {
          inherit system;
        };
        naersk' = pkgs.callPackage naersk {};
        nativeBuildInputs = with pkgs; [
          wrapGAppsHook4
          pkg-config
        ];
        buildInputs = with pkgs; [
          openssl
          dbus
        ];
    in {
      defaultPackage = naersk'.buildPackage {
        inherit nativeBuildInputs buildInputs;
        
        src = ./.;

        WAYLYRICS_THEME_PRESETS_DIR="${placeholder "out"}/share/waylyrics/themes";

        postInstall = ''
install -Dm644 ./metainfo/io.github.waylyrics.Waylyrics.gschema.xml -t $out/share/glib-2.0/schemas/
install -Dm644 ./metainfo/"io.github.waylyrics.Waylyrics.desktop" -t $out/share/applications/
glib-compile-schemas $out/share/glib-2.0/schemas/
install -dm755 $out/share/waylyrics/themes
cp -r ./themes/* $out/share/waylyrics/themes/
cp -r ./res/icons/* $out/share/icons/

cd locales
for po in $(find . -type f -name '*.po')
do
    mkdir -p $out/share/locale/''${po#/*}
    msgfmt -o $out/share/locale/''${po%.po}.mo ''${po}
done
'';
      };
      devShell = pkgs.mkShell {
        inherit buildInputs;
        nativeBuildInputs = with pkgs; [
          rustc
          cargo
          rust-analyzer
        ] ++ nativeBuildInputs;
      };
    }
  );
}

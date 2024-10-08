{
	inputs = {
		nixpkgs.url = "nixpkgs";
		flake-utils.url = "github:numtide/flake-utils";
	};

	outputs = { self, nixpkgs, flake-utils }:
		flake-utils.lib.eachDefaultSystem (system:
			let
				pkgs = nixpkgs.legacyPackages.${system};

				libraries = with pkgs;[
					webkitgtk
					gtk3
					cairo
					gdk-pixbuf
					pkg-config
					glib
					dbus
					openssl_3
					librsvg
				];

				packages = with pkgs; [
					curl
					wget
					pkg-config
					dbus
					openssl_3
					glib
					gtk3
					libsoup
					webkitgtk
					librsvg
				];
			in
			{
				devShell = pkgs.mkShell {
					buildInputs = packages;

					shellHook =
						''
							export RUST_SRC_PATH=/run/current-system/sw/lib/rustlib/src/rust/src
							export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath libraries}:$LD_LIBRARY_PATH
							export XDG_DATA_DIRS=${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS
						'';
				};
				packages.default = pkgs.rustPlatform.buildRustPackage rec {
					pname = "cwe-client-cli";
					version = "0.3.3";
					buildInputs = libraries ++ packages;
					nativeBuildInputs = libraries ++ packages;
					src = ./.;
					cargoSha256 = "sha256-iQ8b5+VNU89RiO4t4rI9ULwFv6KS71oVycHT7BeF8jQ=";
				};
			});
}

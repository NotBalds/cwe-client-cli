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
					pkg-config
					openssl_3
				];

				packages = with pkgs; [
					pkg-config
					openssl_3
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
					version = "1.0.0";
					buildInputs = libraries ++ packages;
					nativeBuildInputs = libraries ++ packages;
					src = ./.;
					cargoHash = "sha256-36OAMxgGlCyCm5unM84AgmAt9y0qBN1EyJCC0guC6xg=";
				};
			});
}

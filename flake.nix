{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      supportedSystems = [ "x86_64-linux" "aarch64-linux" ];
      eachSystem = f:
        nixpkgs.lib.genAttrs supportedSystems (system: f {
          pkgs = import nixpkgs { inherit system; };
        });
    in
    {
      packages = eachSystem ({ pkgs }: {
        hello-gpui = pkgs.callPackage ./nix { };
      });

      devShells = eachSystem ({ pkgs }: {
        default = pkgs.mkShell (with pkgs; rec {
          nativeBuildInputs = [
            pkg-config
          ];

          buildInputs = [
            openssl
            fontconfig
            alsa-lib
            libxkbcommon
            xorg.libxcb
            wayland
            vulkan-loader
            freetype
          ];

          LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
        });
      });
    };
}

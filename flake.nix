{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = {
    self,
    flake-utils,
    naersk,
    nixpkgs,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = (import nixpkgs) {
          inherit system;
        };

        naersk' = pkgs.callPackage naersk {};

        so-logo-ascii-generator-pkg = naersk'.buildPackage {
          src = ./.;
          cargoBuildOptions = opts: opts ++ ["--features=build-binary"];
        };
      in {
        packages = {
          so-logo-ascii-generator = so-logo-ascii-generator-pkg;
          default = so-logo-ascii-generator-pkg;
        };

        # For `nix build` & `nix run`:
        defaultPackage = so-logo-ascii-generator-pkg;

        # For `nix develop`:
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [rustc cargo];
        };
      }
    )
    // {
      meta = {
        description = "Rust crate and cli to generate solaaradotnet branded logos.";
        homepage = "https://github.com/solaaradotnet/so-logo-ascii-generator.git";
        license = nixpkgs.lib.licenses.bsd3;
        maintainers = [
          {
            name = "Solaara Evermore";
            github = "iamsolaara";
            githubId = 17319563;
          }
        ];
      };
    };
}

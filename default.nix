with import <nixpkgs> {};
stdenv.mkDerivation {
  name = "gittemplate";
  # used by the git2 crate
  buildInputs = [ openssl pkg-config ];
}

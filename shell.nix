{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    gcc
	graphviz
  ];

  shellHook = ''
  	#export 
  '';
}

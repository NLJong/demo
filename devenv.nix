{pkgs, ...}: {
  packages = [
    pkgs.mdbook
    pkgs.mdbook-alerts
  ];

  processes = {
    mdbook.exec = "mdbook serve";
  };
}

pkg_name=find-file
pkg_origin=saffronsnial
pkg_version=2.0.0
pkg_license=('unlicense')
pkg_source=nosuchfile.tar.gz
pkg_bin_dirs=(bin)
pkg_build_deps=(core/rust)

do_build() {
  cargo install find-file --root $pkg_prefix --vers $pkg_version --verbose
}

do_install() {
  return 0
}

do_download() {
  return 0
}

do_verify() {
  return 0
}

do_unpack() {
  return 0
}

do_prepare() {
  return 0
}


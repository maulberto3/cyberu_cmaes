ndarray Github says it's compatible with various backends: 

*The following versions have been verified to work together. For ndarray 0.15 or later, there is no tight coupling to the blas-src version, so version selection is more flexible.

ndarray	blas-src	openblas-src	netlib-src
0.15	0.8	0.10	0.8
0.15	0.7	0.9	0.8
0.14	0.6.1	0.9.0	 
0.13	0.2.0	0.6.0*

Then, ndarray-linalg says something similar, 

*Backend Features
There are three LAPACK source crates:

openblas-src
netlib-src
intel-mkl-src
ndarray_linalg must link just one of them for LAPACK FFI.

[dependencies]
ndarray = "0.14"
ndarray-linalg = { version = "0.13", features = ["openblas-static"] }
Supported features are following:

Feature	Link type	Requirements	Description
openblas-static	static	gcc, gfortran, make	Build OpenBLAS in your project, and link it statically
openblas-system	dynamic/static	libopenblas-dev	Seek OpenBLAS in system, and link it
netlib-static	static	gfortran, make	Same as openblas-static except for using reference LAPACK
netlib-system	dynamic/static	liblapack-dev	Same as openblas-system except for using reference LAPACK
intel-mkl-static	static	(pkg-config)	Seek static library of Intel MKL from system, or download if not found, and link it statically
intel-mkl-system	dynamic	(pkg-config)	Seek shared library of Intel MKL from system, and link it dynamically*

So, seems that openblas is the way to go.

To install libopenblas-dev do:

*apt search openblas
sudo apt install libopenblas-dev*

Seems like this will be installed in wsl2 in /usr/share:

*dpkg-query -L libopenblas-dev
/.
/usr
/usr/share
/usr/share/doc
/usr/share/doc/libopenblas-dev
/usr/share/doc/libopenblas-dev/BACKERS.md
/usr/share/doc/libopenblas-dev/CONTRIBUTORS.md.gz
/usr/share/doc/libopenblas-dev/README.md.gz
/usr/share/doc/libopenblas-dev/USAGE.md.gz
/usr/share/doc/libopenblas-dev/copyright
/usr/share/lintian
/usr/share/lintian/overrides
/usr/share/lintian/overrides/libopenblas-dev
/usr/share/doc/libopenblas-dev/changelog.Debian.gz*

Full log install is this:

*Reading package lists... Done
Building dependency tree
Reading state information... Done
The following additional packages will be installed:
  libopenblas-pthread-dev libopenblas0
The following NEW packages will be installed:
  libopenblas-dev libopenblas-pthread-dev libopenblas0
0 upgraded, 3 newly installed, 0 to remove and 0 not upgraded.
Need to get 4548 kB of archives.
After this operation, 57.2 MB of additional disk space will be used.
Do you want to continue? [Y/n] Y
Get:1 http://archive.ubuntu.com/ubuntu focal-updates/universe amd64 libopenblas0 amd64 0.3.8+ds-1ubuntu0.20.04.1 [5892 B]
Get:2 http://archive.ubuntu.com/ubuntu focal-updates/universe amd64 libopenblas-pthread-dev amd64 0.3.8+ds-1ubuntu0.20.04.1 [4526 kB]
Get:3 http://archive.ubuntu.com/ubuntu focal-updates/universe amd64 libopenblas-dev amd64 0.3.8+ds-1ubuntu0.20.04.1 [16.4 kB]
Fetched 4548 kB in 2s (2503 kB/s)
Selecting previously unselected package libopenblas0:amd64.
(Reading database ... 93229 files and directories currently installed.)
Preparing to unpack .../libopenblas0_0.3.8+ds-1ubuntu0.20.04.1_amd64.deb ...
Unpacking libopenblas0:amd64 (0.3.8+ds-1ubuntu0.20.04.1) ...
Selecting previously unselected package libopenblas-pthread-dev:amd64.
Preparing to unpack .../libopenblas-pthread-dev_0.3.8+ds-1ubuntu0.20.04.1_amd64.deb ...
Unpacking libopenblas-pthread-dev:amd64 (0.3.8+ds-1ubuntu0.20.04.1) ...
Selecting previously unselected package libopenblas-dev:amd64.
Preparing to unpack .../libopenblas-dev_0.3.8+ds-1ubuntu0.20.04.1_amd64.deb ...
Unpacking libopenblas-dev:amd64 (0.3.8+ds-1ubuntu0.20.04.1) ...
Setting up libopenblas-pthread-dev:amd64 (0.3.8+ds-1ubuntu0.20.04.1) ...
update-alternatives: using /usr/lib/x86_64-linux-gnu/openblas-pthread/libblas.so to provide /usr/lib/x86_64-linux-gnu/libblas.so (libblas.so-x86_64-linux-gnu) in auto mode
update-alternatives: using /usr/lib/x86_64-linux-gnu/openblas-pthread/liblapack.so to provide /usr/lib/x86_64-linux-gnu/liblapack.so (liblapack.so-x86_64-linux-gnu) in auto mode
update-alternatives: using /usr/lib/x86_64-linux-gnu/openblas-pthread/libopenblas.so to provide /usr/lib/x86_64-linux-gnu/libopenb
las.so (libopenblas.so-x86_64-linux-gnu) in auto mode
Setting up libopenblas0:amd64 (0.3.8+ds-1ubuntu0.20.04.1) ...
Setting up libopenblas-dev:amd64 (0.3.8+ds-1ubuntu0.20.04.1) ...*

After several tests, it seems that the apt install libopenblas-dev with cargo.toml ndarray-linalg = { version = "0.16", features = ["openblas-system"] } really works and linalg examples run well. Other combinations dont work at all, i.e. ["openblas-static"]

Similarly, all but configuration:

*ndarray = { version = "0.15", features = ["blas"] }
blas-src = { version = "0.10", features = ["openblas"] }
openblas-src = { version = "0.10", features = ["cblas", "system"] }*

And also doing `use blas_src;` is the onlyway to make ndarray work with blas.




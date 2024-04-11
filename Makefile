PREFIX ?= /usr/local
PROFILE ?= dev
BUILDTYPE ?= debug
SFML ?= 2.6.1
INCLUDEDIR ?= $(PREFIX)/include
LIBDIR ?= $(PREFIX)/lib
BUILDDIR ?= builddir
CARGOPACKS ?= -p ege1 -p first_word -p integral -p last_word -p longman -p longman2 -p mask1 -p \
mcko -p pop -p probnik -p resheto -p speedometer -p tumba-umba

.PHONY: rust rust-with-sfml rust-speedometer c-setup c py clean clean-all compile-sfml test

all: rust-with-sfml c py

ifdef MOLD
CARGO := mold -run cargo b -v --profile $(PROFILE) --manifest-path rusted/Cargo.toml
else
CARGO := cargo b -v --profile $(PROFILE) --manifest-path rusted/Cargo.toml
endif

ifdef PY_OPT
OPT_OPTS := -o 0 -o 1
endif

compile-sfml:
ifdef SFML_SOURCE
	(cd ~ && curl -LO https://github.com/SFML/SFML/archive/$(SFML).tar.gz && \
	tar -xzf $(SFML).tar.gz && mkdir -p build && cd build && cmake \
	-DCMAKE_BUILD_TYPE=Release \
	-DCMAKE_INSTALL_PREFIX=$(PREFIX) \
	-DBUILD_SHARED_LIBS=ON \
	../SFML-$(SFML))
	(cd ~/build && cmake --build . && sudo cmake --install .)
endif

rust:
	$(CARGO) $(CARGOPACKS)

rust-speedometer:
	$(CARGO) -p speedometer

c-setup:
	cd dad-is-great-in-C && meson setup --buildtype=$(BUILDTYPE) $(BUILDDIR)

c: c-setup
	(cd dad-is-great-in-C && \
	meson setup --reconfigure --buildtype=$(BUILDTYPE) $(BUILDDIR) && \
	cd $(BUILDDIR) && meson compile)

rust-with-sfml: compile-sfml
ifdef SFML_SOURCE
	SFML_LIBS_DIR=$(LIBDIR) SFML_INCLUDE_DIR=$(INCLUDEDIR) $(CARGO)
else
	$(CARGO)
endif

py:
	(cd pie && python3 -m pip install -r requirements.txt \
	&& python3 -m compileall $(OPT_OPTS) .)

clean:
	(rm -rf pie/__pycache__)
	(ninja -C dad-is-great-in-C/$(BUILDDIR) clean)
	(cd rusted && cargo clean)

clean-all: clean
	(rm -rf dad-is-great-in-C/$(BUILDDIR))

test: c rust py
	(cargo test $(CARGOPACKS) --manifest-path rusted/Cargo.toml || echo "check the error!")
	(./dad-is-great-in-C/builddir/ogntest || echo "check the error!")
	(python3 pie/mcko.py || echo "check the error!")

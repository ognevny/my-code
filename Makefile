PREFIX ?= /usr/local
PROFILE ?= dev
SFML ?= 2.6.1
INCLUDEDIR ?= $(PREFIX)/include
LIBDIR ?= $(PREFIX)/lib

.PHONY: rust rust-with-sfml c py clean rust-speedometer compile-sfml

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
	$(CARGO) -p first_word -p integral -p last_word -p longman -p mask1 -p mcko -p pop -p \
	probnik -p resheto -p speedometer -p tumba-umba

rust-speedometer:
	$(CARGO) -p speedometer

c:
	(cd dad-is-great-in-C && meson setup builddir && cd builddir && meson compile)

rust-with-sfml: compile-sfml
ifdef SFML_SOURCE
	SFML_LIBS_DIR=$(LIBDIR) SFML_INCLUDE_DIR=$(INCLUDEDIR) $(CARGO)
else
	$(CARGO)
endif

py:
	(cd pie && pip install -r requirements.txt && python -m compileall $(OPT_OPTS) .)

clean:
	(rm -rf pie/__pycache__)
	(rm -rf dad-is-great-in-c/build*)
	(cd rusted && cargo clean)

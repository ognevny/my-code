PROFILE ?= dev
SFML ?= 2.6.1

.PHONY: rust rust-with-sfml c py clean

all: rust-with-sfml c py

rust:
ifdef MOLD
	(mold -run cargo b -v --profile $(PROFILE) --manifest-path rusted/Cargo.toml -p first_word \
	-p integral -p last_word -p longman -p mask1 -p mcko -p pop -p probnik -p resheto -p \
	speedometer -p tumba-umba)
else
	(cargo b -v --profile $(PROFILE) --manifest-path rusted/Cargo.toml -p first_word -p \
	integral -p last_word -p longman -p mask1 -p mcko -p pop -p probnik -p resheto -p \
	speedometer -p tumba-umba)
endif

c:
	(cd dad-is-great-in-C && meson setup build && cd build && meson compile)

rust-with-sfml:
ifdef MOLD
ifdef SFML_SOURCE
		(cd ~ && curl -LO https://github.com/SFML/SFML/archive/refs/tags/$(SFML).tar.gz && \
		tar -xzf $(SFML).tar.gz && mkdir -p build && cd build && cmake \
		-DCMAKE_BUILD_TYPE=Release \
		-DCMAKE_INSTALL_PREFIX=/usr/local \
		-DBUILD_SHARED_LIBS=ON \
		-DCMAKE_LINKER=/usr/bin/mold \
		../SFML-$(SFML))
		(cd ~/build && cmake --build . && sudo cmake --install .)
endif
	(SFML_LIBS_DIR=/usr/local/lib SFML_INCLUDE_DIR=/usr/local/include \
	mold -run cargo b -v --profile $(PROFILE) --manifest-path rusted/Cargo.toml)
else
ifdef SFML_SOURCE
		(cd ~ && curl -LO https://github.com/SFML/SFML/archive/refs/tags/$(SFML).tar.gz &&
		tar -xzf $(SFML).tar.gz && mkdir -p build && build && cmake -DCMAKE_BUILD_TYPE=Release \
		-DCMAKE_INSTALL_PREFIX=/usr/local \
		-DBUILD_SHARED_LIBS=ON \
		../SFML-$(SFML))
		(cd ~/build && cmake --build . && sudo cmake --install .)
endif
	(SFML_LIBS_DIR=/usr/local/lib SFML_INCLUDE_DIR=/usr/local/include \
	cargo b -v --profile $(PROFILE) --manifest-path rusted/Cargo.toml)
endif

py:
ifndef PY_OPT
	(cd pie && pip install -r requirements.txt && python -m compileall .)
else
	(cd pie && pip install -r requirements.txt && python -m compileall -o 0 -o 1 .)
endif

clean:
	(cd pie && rm -rf __pycache__)
	(cd dad-is-great-in-c && rm -rf build)
	(cd rusted && cargo clean)

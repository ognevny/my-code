PROFILE ?= dev
SFML ?= 2.6.1

rust:
ifdef MOLD
	mold -run cargo b -v --profile $(PROFILE) --manifest-path rusted/Cargo.toml -p first_word \
	-p integral -p last_word -p longman -p mask1 -p mcko -p pop -p probnik -p resheto -p \
	speedometer -p tumba-umba
else
	cargo b -v --profile $(PROFILE) --manifest-path rusted/Cargo.toml -p first_word -p \
	integral -p last_word -p longman -p mask1 -p mcko -p pop -p probnik -p resheto -p \
	speedometer -p tumba-umba
endif

c:
	cd dad-is-great-in-C && meson setup build && cd build && meson compile

nightly:
ifdef MOLD
	cd ~ && curl -LO https://github.com/SFML/SFML/archive/refs/tags/$(SFML).tar.gz
	cd ~ && tar -xzf $(SFML).tar.gz && mkdir -p build && build && cmake \
	-DCMAKE_BUILD_TYPE=Release \
	-DCMAKE_INSTALL_PREFIX=/usr \
	-DBUILD_SHARED_LIBS=ON \
	-DCMAKE_LINKER=/usr/bin/mold \
	../SFML-$(SFML)
	cd ~/build && cmake --build . && sudo cmake --install .
	mold -run cargo b -v --profile $(PROFILE) --manifest-path rusted/Cargo.toml
else
	cd ~ && curl -LO https://github.com/SFML/SFML/archive/refs/tags/$(SFML).tar.gz &&
	tar -xzf $(SFML).tar.gz && mkdir -p build && build && cmake -DCMAKE_BUILD_TYPE=Release \
	-DCMAKE_INSTALL_PREFIX=/usr \
	-DBUILD_SHARED_LIBS=ON \
	../SFML-$(SFML)
	cd ~/build && cmake --build . && sudo cmake --install .
	cargo b -v --profile $(PROFILE) --manifest-path rusted/Cargo.toml
endif

py:
ifndef PY_OPT
	cd pie && pip install -r requirements.txt && python -m compileall .
else
	cd pie && pip install -r requirements.txt && python -m compileall -o 0 -o 1 .
endif

clean:
	cd pie && rm -rf __pycache__
	cd dad-is-great-in-c && rm -rf build
	cd rusted && cargo clean

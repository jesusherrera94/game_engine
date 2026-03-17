OUTPUT_DIR = ./c_output

OPENGL_WRAPPER_LIB_DIR = ./opengl_wrapper_lib
OPENGL_WRAPPER_LIB_SRC = $(OPENGL_WRAPPER_LIB_DIR)/opengl_wrapper_lib.c
OPENGL_WRAPPER_LIB_OBJ = $(OUTPUT_DIR)/opengl_wrapper_lib.o
OPENGL_WRAPPER_LIB = openglwrapper
OPENGL_WRAPPER_LIB_SO = $(OUTPUT_DIR)/lib$(OPENGL_WRAPPER_LIB).so

TEST_GAME_DIR = ./c_test_game
TEST_GAME_SRC = $(TEST_GAME_DIR)/test_game.c
TEST_GAME_EXE = $(OUTPUT_DIR)/test_game_exe

# OS detection
UNAME_S := $(shell uname -s)

ifeq ($(UNAME_S), Darwin)
    BREW_PREFIX = /opt/homebrew
    EXTRA_CFLAGS = -I$(BREW_PREFIX)/include
    EXTRA_LDFLAGS = -L$(BREW_PREFIX)/lib
    GL_LIBS = -framework OpenGL -framework Cocoa -framework IOKit -framework CoreVideo
else
    EXTRA_CFLAGS =
    EXTRA_LDFLAGS =
    GL_LIBS = -lGL
endif

.PHONY: build-c
build-c:
	@echo "Building OpenGL Wrapper library..."
	gcc -c -fPIC $(EXTRA_CFLAGS) $(OPENGL_WRAPPER_LIB_SRC) -o $(OPENGL_WRAPPER_LIB_OBJ)
	gcc -shared -o $(OPENGL_WRAPPER_LIB_SO) $(OPENGL_WRAPPER_LIB_OBJ) $(EXTRA_LDFLAGS) -lglfw $(GL_LIBS)

.PHONY: run-c
run-c: build-c
	@echo "Running Test Game..."
	gcc $(TEST_GAME_SRC) -o $(TEST_GAME_EXE) $(EXTRA_CFLAGS) -L$(OUTPUT_DIR) $(EXTRA_LDFLAGS) -l$(OPENGL_WRAPPER_LIB) -Wl,-rpath,$(OUTPUT_DIR)
	$(TEST_GAME_EXE)

.PHONY: test-rust
test-rust:
	@echo "Running Rust Tests Serially..."
	cargo test --manifest-path ./my_game_engine/Cargo.toml tests::test_simple_game_loop -- --nocapture --ignored
	cargo test --manifest-path ./my_game_engine/Cargo.toml tests::test_sprite_rendering -- --nocapture --ignored
	cargo test --manifest-path ./my_game_engine/Cargo.toml tests::test_screen_clearing -- --nocapture --ignored
	cargo test --manifest-path ./my_game_engine/Cargo.toml tests::test_key_presses -- --nocapture --ignored
	cargo test --manifest-path ./my_game_engine/Cargo.toml tests::test_sprite_position_update -- --nocapture --ignored

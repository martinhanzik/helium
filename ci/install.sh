set -ex

main() {
  mkdir -p deps
  cd deps
  wget https://www.libsdl.org/release/SDL2-2.0.4.tar.gz -O sdl2.tar.gz
  tar xzf sdl2.tar.gz

  cd SDL2-2.0.4
  ./configure
  make
  sudo make install
  cd ..

  wget -q http://www.libsdl.org/projects/SDL_ttf/release/SDL2_ttf-2.0.12.tar.gz
  tar xzf SDL2_ttf-*.tar.gz

  cd SDL2_ttf-*
  ./configure
  make
  sudo make install
  cd ..
}

main
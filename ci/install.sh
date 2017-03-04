set -ex

main() {
  if [ $TRAVIS_OS_NAME = linux ]; then
      mkdir -p deps
      cd deps
      if [ ! -d "SDL2-2.0.4" ]; then
        wget https://www.libsdl.org/release/SDL2-2.0.4.tar.gz -O sdl2.tar.gz
        tar xzf sdl2.tar.gz

        cd SDL2-*
        ./configure
        make
        sudo make install
        cd ..
      fi

      if [ ! -d "SDL2_ttf-2.0.12" ]; then
        wget -q http://www.libsdl.org/projects/SDL_ttf/release/SDL2_ttf-2.0.12.tar.gz
        tar xzf SDL2_ttf-*.tar.gz

        cd SDL2_ttf-*
        ./configure
        make
        sudo make install
        cd ..
      fi
  else
    brew install freetype sdl2 sdl2_ttf
  fi
}

main
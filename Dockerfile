FROM archlinux
RUN pacman --noconfirm -Syu
RUN pacman --noconfirm -S git docker
RUN pacman --noconfirm -S rustup && rustup default stable
RUN git clone https://github.com/studsovet/Action-DDB && cd Action-DDB && cargo run

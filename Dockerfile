FROM archlinux
RUN pacman --noconfirm -Syu
RUN pacman --noconfirm -S git docker
RUN pacman --noconfirm -S clang rustup && rustup default stable
RUN systemctl start docker.service && git clone https://github.com/studsovet/action_ddb && cd action_ddb && cargo run

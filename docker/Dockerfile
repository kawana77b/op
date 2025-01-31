FROM rust:1.84.1-slim-bullseye

# Create the user
ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID

RUN groupadd --gid $USER_GID $USERNAME \
  && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME

# Install Dependencies
RUN apt-get update && apt-get install -y --no-install-recommends  \
  build-essential \
  mingw-w64 \
  musl-dev \
  musl-tools \
  git \
  zip \
  unzip 

USER $USERNAME

WORKDIR /workspaces

ENV SHELL=/bin/bash

CMD ["/bin/bash"]
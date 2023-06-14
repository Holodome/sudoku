FROM rust:1.70-slim-bullseye as build-deps

RUN apt-get update && apt-get install -y \
    protobuf-compiler

RUN mkdir /usr/src/proto
COPY proto /usr/src/proto

RUN mkdir /usr/src/app
WORKDIR /usr/src/app
COPY sudoku-gen .

FROM build-deps as build

RUN cargo build --release

FROM build as result

COPY --from=build /usr/src/app/target/release/sudoku-gen /sudoku-gen
ENTRYPOINT ["/sudoku-gen"]
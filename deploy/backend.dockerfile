FROM sbtscala/scala-sbt:eclipse-temurin-jammy-19.0.1_10_1.9.0_3.3.0 AS build-deps

RUN apt-get update && apt-get install -y \
    protobuf-compiler

RUN mkdir /usr/src/proto
COPY proto /usr/src/proto

RUN mkdir /usr/src/app
WORKDIR /usr/src/app
COPY backend .

FROM build-deps as build

RUN sbt compile clean package

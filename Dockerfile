FROM jimmycuadra/rust

COPY src/ /source/src
COPY ui/ /source/ui
COPY Cargo* /source/

EXPOSE 3000

CMD cargo run
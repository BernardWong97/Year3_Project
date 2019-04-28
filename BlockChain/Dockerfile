FROM scratch

COPY ./blockchain.backup /
COPY ./settings_file.txt /
ADD target/x86_64-unknown-linux-musl/release/app /
EXPOSE 8000

CMD ["/app"]

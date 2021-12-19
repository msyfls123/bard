./target/x86_64-unknown-linux-musl/release/main &
sleep 1
curl 127.0.0.1:8000
sleep 3 
kill "$!"

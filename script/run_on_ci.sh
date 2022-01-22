./target/release/server &
sleep 3
curl 127.0.0.1:8000/vertex
sleep 3 
kill "$!"

./target/release/server &
sleep 3
curl -H "Content-Type: application/json" -X POST -d '{ "t": "animal", "properties": { "name": "snoopy" } }' 127.0.0.1:8000/vertex
sleep 2
curl 127.0.0.1:8000/vertex
sleep 3 
kill "$!"

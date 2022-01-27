rm -rf static

if [ "$1" == "-w" ]; then
    npm run --prefix client watch
else
    npm run --prefix client compile
fi
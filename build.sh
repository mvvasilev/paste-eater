rm -rf build

cargo build --release
(cd paste-eater-frontend && yarn build)

mkdir -p build/paste-eater-frontend/build
cp -r target/release/paste-eater build/
cp -r paste-eater-frontend/build/* build/paste-eater-frontend/build
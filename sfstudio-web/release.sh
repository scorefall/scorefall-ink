cargo web deploy --release
rm -r ../scorefall.github.io/*
cp target/deploy/* ../scorefall.github.io/ -r

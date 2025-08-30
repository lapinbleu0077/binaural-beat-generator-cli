cd ..
cargo doc --no-deps --document-private-items
rm -rv ./docs/*
cp -rv ./target/doc/* ./docs/
echo "<meta http-equiv='refresh' content='0; url=binaural_beat_generator_cli'>" > ./docs/index.html
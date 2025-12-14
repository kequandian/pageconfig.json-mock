# Build the project
echo "Building the project..."
docker run --rm -v $PWD:/usr/src -w /usr/src rust:dev cargo build --release

# Check if build succeeded and run the executable inside container
if [ $? -eq 0 ]; then
    echo "Build successful! Running the executable..."
    docker run --rm -v $PWD:/usr/src -w /usr/src rust:dev ./target/release/json-mock-rust
else
    echo "Build failed!"
    exit 1
fi

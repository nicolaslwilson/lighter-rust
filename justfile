TAG := "lighter-rust"
UID := shell("id -u")
GID := shell("id -g")
DOCKER_RUN := "docker run --interactive --rm --volume " + shell("pwd") + ":/app -e USER=satoshi"

# Recipes
builder:
    docker build \
        --tag {{TAG}} \
        --build-arg UID={{UID}} \
        --build-arg GID={{GID}} \
        -f Dockerfile \
        .

clean:
    rm -rf target

build profile="": builder
    {{DOCKER_RUN}} --tty {{TAG}} cargo build --{{profile}}

test test_name="": build
    {{DOCKER_RUN}} --tty {{TAG}} cargo test -- --test-threads=1 --nocapture {{test_name}}

doc:
    {{DOCKER_RUN}} --tty {{TAG}} cargo doc --no-deps
    open target/doc/lighter_rust/index.html

openapi-gen things="apis,models,docs":
    docker run --rm \
        -v ${PWD}:/local openapitools/openapi-generator-cli generate \
        -i /local/openapi.json \
        -g rust \
        -o /local/ \
        --global-property {{things}}
    find patches -type f -name '*.patch' -exec sh -c 'patch -p1 --no-backup-if-mismatch -N -d src < "$1"' _ {} \;
    cargo fmt

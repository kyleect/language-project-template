docker_image := "language-project-template:0.0.0"

# List commands
default:
    just --list

# Run tests
test:
    cargo test

# Run tests with coverage enabled
coverage:
    ./coverage.sh

# Build docker image 
build-docker:
    docker build -t {{docker_image}} .

# Run docker image
run-docker *cli_args:
    docker run -it --rm --read-only {{docker_image}} {{cli_args}}
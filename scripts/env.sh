RUSTFLAGS="-A dead_code"
PROJECT_ROOT=$(cd `dirname $0`/..; pwd)

# Will use docker to run the script if rustc is not detected
DOCKER_IMAGE=newretail.alpha.elenet.me:5000/rust-wasm

RUSTC_EXISTS=`which rustc`
if [ -z "$RUSTC_EXISTS" -a -z "$NO_DOCKER" ]; then
  exec docker run -e USER="$USER" --rm -v "$PROJECT_ROOT":'/src' -it "$DOCKER_IMAGE" $0 $@
  exit
fi


docker run -v "$PWD/config:/server/tes3mp-server-default.cfg" -v "$(realpath "$PWD/../target/debug/libtes3mp_test.so"):/server/data/scripts/test.so" -ti tes3mp/server

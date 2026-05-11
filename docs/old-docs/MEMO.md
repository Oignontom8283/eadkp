xhost +local:docker
docker system prune -a
docker exec -it numworks-app bash
docker-compose up -d --build

rustup target add thumbv7em-none-eabihf

https://docs.rs/eadkp

arm-none-eabi-nm build/example.nwa | grep calcul
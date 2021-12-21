# Running sonic server with docker

 docker run -p 1491:1491 -v $(pwd)/config.cfg:/etc/sonic.cfg -v $(pwd)/store/:/var/lib/sonic/store/ valeriansaliou/sonic:v1.3.2

# Running sonic server with docker

docker run -p 1491:1491 -v c:\Users\Toon\tmp\sonic\config.cfg:/etc/sonic.cfg -v c:\Users\Toon\tmp\sonic\store:/var/lib/sonic/store/ valeriansaliou/sonic:v1.3.2
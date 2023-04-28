cargo build --release
scp target/release/logkouhai root@mx.xy2.dev:~/logkouhai/
ssh root@mx.xy2.dev "systemctl restart logkouhai"
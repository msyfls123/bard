# Setup
```shell
unzip build.zip -d ./bard
mkdir store
cd bard
```

# Build
docker build -t bard:1 .

# Run
docker-compose up -d

# Debug
https://www.runoob.com/docker/docker-mirror-acceleration.html

Ubuntu14.04、Debian7Wheezy
对于使用 upstart 的系统而言，编辑 /etc/default/docker 文件，在其中的 DOCKER_OPTS 中配置加速器地址：

DOCKER_OPTS="--registry-mirror=https://registry.docker-cn.com"

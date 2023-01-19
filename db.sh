docker run -d \
    --name postgres-db \
    --restart=always \
    -e POSTGRES_USER=cos \
    -e POSTGRES_PASSWORD=SkQm8ujbSHs4U80gRoMl \
    -p 5432:5432 \
    -v cos_data:/var/lib/postgresql/data \
    postgres:14-alpine3.16
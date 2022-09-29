Docker compose to get image working
```
services:
  linky-db:
    image: postgres:11-alpine
    restart: always
    environment:
      - POSTGRES_PASSWORD=actix
      - POSTGRES_USER=actix
      - POSTGRES_DB=actix
    ports:
      - 5430:5432
    volumes:
      - pgdata:/var/lib/postgresql/data
  linky-server:
    image: koziollek/linky-server
    ports:
      - 9017:9017
    environment:
      - SERVER_ADDR=0.0.0.0:9017
      - PG.USER=actix
      - PG.PASSWORD=actix
      - PG.HOST=linky-db
      - PG.DBNAME=actix
      - PG.POOL.MAX_SIZE=16
      #Uncomment bellow to skip setting up the database
      # - NO_NEW_SETUP=true
    depends_on:
      - linky-db
volumes:
  pgdata:
```
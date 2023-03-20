# A Link-shortener with a React frontend and Rust backend

The first non trivial project for both Rust and React I have worked on.
It was a great learning experience but the project is on hold for now.

There are some issues which I didnt get around to fix yet:
- This program only works for my url at the moment because the program is compiled with my url hardcoded.
- It is possible to build loops that point to my service that could seriously slow down the application if used maliciously
- I am not utilizing Docker Volumes at the moment. Therefor data is a little unsafe 

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
    image: koziollek/linky-rust
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

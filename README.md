# AutoTrack-backend

`docker-compose up -d --build`

```sh
$ curl -X GET http://localhost:8369/
  Hello, world!

```

```sh
$ curl -X GET http://localhost:8369/users
  [{"user_id":1,"user_email":"test@example.com","user_name":"Test User","user_password":"password","created_at":[2024,160,9,38,58,0,0,0,0],"updated_at":[2024,160,9,38,58,0,0,0,0]}]
```

`docker-compose down -v`

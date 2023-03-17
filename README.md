## Integration rbatis with actix web

1. Create mysql table with `user.sql` in project root
2. Run `cargo run`
3. Create user

```
curl -X "POST" "http://127.0.0.1:9991/user/save" \
     -H 'Content-Type: application/json; charset=utf-8' \
     -d $'{
  "name": "user created"
}'
```

4. Update user

```
curl -X "POST" "http://127.0.0.1:9991/user/update" \
     -H 'Content-Type: application/json; charset=utf-8' \
     -d $'{
  "id": 1,
  "name": "test updated"
}'
```

5. List user

```
curl -X "GET" "http://127.0.0.1:9991/user/list" \
     -H 'Content-Type: application/json; charset=utf-8'
```



6. Show user

```
curl -X "POST" "http://127.0.0.1:9991/user/show" \
     -H 'Content-Type: application/json; charset=utf-8' \
     -d $'{
  "id": 1
}'
```


7. Delete user

```
curl -X "POST" "http://127.0.0.1:9991/user/delete" \
     -H 'Content-Type: application/json; charset=utf-8' \
     -d $'{
  "id": 1
}'
```


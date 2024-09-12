# Json api

You can make a quick ad-hoc json api with the following schema

```yaml
state: This
host: 127.0.0.1:6000
endpoints:
- !get
  to: /todos
  returns:
  - date: may 10
    something: alskdjas
  status: 200
- !get
  to: /todo/1
  returns:
  - date: may 10
    content: today I did this
  status: 201
- !get
  to: /thing
  returns:
  - date: may 10
    content: today I did this
  status: 201
```

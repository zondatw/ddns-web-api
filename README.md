# DDNS web api

## Setup

Create `.env` file and set value which you want, you can reference `.env.example`

## Example

### DDNS

#### Post

```shell
curl -v -X POST "http://127.0.0.1:8080/ddns" -H 'Content-Type: application/json' -d '{"subdomain": "test", "ip": "123.123.123.123"}'
```

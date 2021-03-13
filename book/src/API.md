# Hole Frontend API

## Loginning and Registration

### Registration request
On base of password will be generted keypair for signing and keypair for encoding.
``` json
    {
    request_type: "registration",
    username: "user",
    password: "passwd123",
    }
```


### Login request
```json
    {
    request_type: login,
    username: "name",
    decode_key: "key",
    sign_key: "key",
    }
```

## Adding friends
Each user have :w


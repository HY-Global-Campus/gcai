# GCAI Backend

## How to run the project

With docker: 

```
docker compose up
```
In project root directory or witch cargo withour docker:

```
cargo run
```

In project root /backend (this directory).

You need to copy example.env to .env and insert env variables.


## Example usage
Examples with `curl`.

# Without "own data"

```
curl --location 'https://gcai-backend.jollysky-44df5b53.swedencentral.azurecontainerapps.io/api/chat' \
--header 'Content-Type: application/json' \
--header 'Authorization: Bearer <Auth_Token>' \
--data '{
  "messages": [
    {
      "role": "system",
      "content": "You are an AI assistant that helps people find information."
    },
    {
        "role": "user",
        "content": "Hi! What are monads?"
    }
  ]
}'
```

# With "own data"

```
curl --location 'https://gcai-backend.jollysky-44df5b53.swedencentral.azurecontainerapps.io/api/chat' \
--header 'Content-Type: application/json' \
--header 'Authorization: Bearer <Auth_Token>' \
--data '{
  "messages": [
    {
      "role": "system",
      "content": "You are an AI assistant that helps people find information."
    },
    {
        "role": "user",
        "content": "Hi! What are monads?"
    }
  ],

  "use_own_data": true

}'
```




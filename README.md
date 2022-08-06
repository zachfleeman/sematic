# Sematic NLU API
`Sematic NLU` is an [NLU](https://en.wikipedia.org/wiki/Natural-language_understanding) API this is a no-holds-barred attempt to convert plain English into structured data. The project currently uses no ML, instead leveraging linguistic analysis libraries such as [OpenCog's link-grammar](https://github.com/opencog/link-grammar), and [Duckling](https://github.com/facebook/duckling). There is a large focus on not just determining objects, but also defining the connections between them.

## Status

The project is functional, but is currently in a very early stage. Some good docs are needed, but in general the following objects types are supported:

- Agents: people, e.g. "John Smith"
- Entities: more or less nouns, e.g. "vitamin A"
- Temporal: time, e.g. "today". 
- Actions: verbs, e.g. "eat"
- Events: usually a combination of an action that happens at a given time, e.g. "John Smith eats at 2pm".
- Queries: questions are detected, but connections to other objects are not yet supported.

TODO:
- Locations
- Relations
- Logic


## Example

#### Request

```bash
curl --location --request POST '<api-endpoint>/text-to-json' \
--header 'Authorization: Bearer <auth_token>' \
--header 'Content-Type: application/json' \
--data-raw '{
    "sentences": ["Jane Smith baked a cake for Thomas on January 21 , 1990"], 
}
```


#### Response

```jsonc
{
    "sema_sentences": [
        {
            "agents": [
                {
                    "agent_type": "person",
                    "symbol": "$1",
                    "properties": [
                        {
                            "first_name": "jane"
                        },
                        {
                            "last_name": "smith"
                        }
                    ]
                },
                {
                    "agent_type": "person",
                    "symbol": "$2",
                    "properties": [
                        {
                            "name": "thomas"
                        }
                    ]
                }
            ],
            "entities": [
                {
                    "entity_type": "cake",
                    "symbol": "$4",
                    "properties": []
                }
            ],
            "locations": [],
            "temporal": [
                {
                    "temporal_type": "absolute",
                    "symbol": "$5",
                    "text": "on January 21 , 1990",
                    "properties": [
                        {
                            "iso": "1990-01-21T00:00:00.000-08:00"
                        }
                    ]
                }
            ],
            "relations": [],
            "actions": [
                {
                    "action_type": "bake",
                    "symbol": "$3",
                    "properties": [
                        {
                            "agent": "$1"
                        },
                        {
                            "patient": "$4"
                        },
                        {
                            "recipient": "$2"
                        }
                    ]
                }
            ],
            "events": [
                {
                    "event_type": "event",
                    "symbol": "$6",
                    "properties": [
                        {
                            "occurs": "$5"
                        },
                        {
                            "action": "$3"
                        }
                    ]
                }
            ],
            "queries": []
        }
    ]
}
```

## Dependencies

At the moment this project requires a running [Duckling]() server. The easiest way to get started is to create a server using the `Dockerfile` included in the root of the project.

### Config

example config:

```
Config(
  logging_directive: "actix_web=info",
  tcp_port: 8088,
  allowed_origins: [
    "http://localhost:8088"
  ],
  graceful_shutdown_timeout_sec: 3,
  max_payload_size_bytes: 1024,
  database_connection_pool_size: 5,
  database_connection_timeout_sec: 3,
  database_url: "", // not used for the moment, but will be used for the future.
  use_jwt_auth: false, // set to true to use JWT auth.
  jwt_secret: "<secret>", // used for JWT auth, if turned on.
  data_path: "<path to project>/sema-api/sema-api/data",
  duckling_url: "<duckling-url>/parse",
)
```

## Installation

The easiest way to get started is to run the project inside of a docker container. The project includes a `Dockerfile` to get an image created. after you create an image, you will need to pass in either a `CONFIG` or `CONFIG_PATH` environment variable to the container when it is started.

You will need to build the link-grammar lib. from the project root, run:

```bash
$ ./install_link_grammar.sh
```

## Development

Make sure you have a recent version of Rust installed. 

```bash
# from project root
cargo install cargo-watch
cd sema-api
cargo watch -x run --clear --no-gitignore
```

This should start the server and watch for changes to the project.

## Help

Feel free to create an issue or open a pull request!
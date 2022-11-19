# szk
Rust-based server for text storage.

## Usage
To run locally: `cargo run`

POST a string:
    
`curl --data-binary 'hi there!' http://localhost:8000`
    
`echo \"hi there!\" | curl --data-binary @- http://localhost:8000`

POST a file:
    
`cat file.txt | curl --data-binary @- http://localhost:8000`

GET contents:

`curl /<id>`

DELETE:

`curl -X DELETE /<id>`

View in browser:

`/view/<id>`

## About
Using [Rocket](https://rocket.rs) as framework.

Tutorial [here](https://rocket.rs/v0.5-rc/guide/pastebin-tutorial/).
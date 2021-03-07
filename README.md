# Pokeserv

A fun API server that provides descriptions of given Pokemon in Shakespeare style. It relies on two
services:

* pokeapi.co
* funtranslations.com/api/shakespeare

## The API

Once the server is up, the API endpoint is available `http://HOSTNAME:3000/pokemon/POKEMON_NAME`.
For example here is an sample query sent from the same machine as the one Pokeserv is running on,
along with their results:

```
$ curl http://localhost:3000/pokemon/charizard
{"name":"charizard","description":"Spits fire yond is hot enow to melt boulders. Known to cause forest fires unintentionally."}
$ curl http://localhost:3000/pokemon/butterfree
{"name":"butterfree","description":"In hurlyburly,  't flaps its wings at high speed to release highly toxic dust into the air."}
```

# Building and running from source code

Pokeserv is written in Rust and as such it uses Cargo as its build system. To install the needed
build tools, simply run the following in your terminal, then follow the onscreen instructions:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After that, you can build and run Pokeserv with this command:

```
cargo run
```

Pokeserv is now up and ready to server requests from all network interfaces.

## TODO

* See if there is a better pokeapi.co endpoint to get the Pokemon description than the one we're
  using. See FIXME in `src/poke.rs` file.

* Make it possible to configure through commandline arguments and environment variables:
  * Port and IP to host on.
  * API keys for the used services.

* Add logging.

* Most importly: Rename to something better. Pokemonspear? :)
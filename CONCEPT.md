# Temp Notes
<!-- TODO finalise -->

* cache for each user's personalised product info:
  * key: (user, product)
  * value: personalised product info
* To show results immediately, maintain a generic, non-personalised product info in cache for each service

* openapi with utoipa & friends
* domain trait alla Zed

* recommendation api should be one aggregated request for all products (i.e. the client does not send one request per product but only one for the user)

## Architectural choices

### Hexagonal architecture pattern

Within the home backend service,

<!-- TODO rename these? -->
* core/ contains business logic and works with abstract traits, not specific programs.
* outbound/ contains adapters handling requests coming from inside, like Redis.
* inbound/ contains adapters handling requests coming from outside. Axum will for example live here.

### Caches
<!-- TODO: measure whether L1 is worth it -->
Going with two caches:

* L1 Cache: Fast, short-lived (~10 seconds?), in-memory cache, entries invalidated using a redis channel
* L2 Cache: Redis e​​vent-driven/push cache: products update the home service when sth new happens instead of the latter polling regularly
  * Exception: the generic non-personalised info/recommendations should probably be polled every few minutes as it does not necessarily undergo discrete changes like "user bought product, change recommendations" but more of a "what are the best deals right now".
  * Push cache:
    * pros:
      * cache stays fresh, updates come as soon as they happen
      * no requests to the product backend when there are no changes to report
    * cons:
      * cache updates happen even when the user is not around & might not see them because a subsequent update overwrites it before the user logs in again
      * 

### Server-driven UI

json description of UI, front-end renders that instead of having templates for every product -> "content updates can and should remain dynamic"


## Tech choices

### Home service backend
<!-- TODO yap about Rust -->
* Rust ofc
* Axum:
  * Fast, estabilished, maintained, ergonomic, Tokio/Tower middleware ecosystem
  * 
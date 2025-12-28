# Temp Notes
<!-- TODO finalise -->

* cache for each user's personalised product info:
  * key: (user, product)
  * value: personalised product info
* To show results immediately, maintain a generic, non-personalised product info in cache for each service

* openapi with utoipa & friends
* domain trait alla Zed

* recommendation api should be one aggregated request for all products (i.e. the client does not send one request per product but only one for the user)

* data collection? What e.g. behavioural analytics should maybe be reported back to products & how? Is this in scope of the challenge?
  * scope: "conversion takes more than speed", "creativity"

## Architectural choices

### Hexagonal architecture pattern

Within the home backend service,

* domain/ contains business logic and works with abstract traits, not specific programs.
* outbound/ contains adapters handling requests coming from inside, like Redis.
* inbound/ contains adapters handling requests coming from outside. Axum will for example live here.

This nicely separates infrastructure, public-facing APIs, and business logic, thereby increasing maintainability while reducing potential attack surfaces.
Note that my PoC does not include any kind of authentication or authorisation past the simple nginx password. I trust that your internal and user authentication and authorisation are solved problems.

### OpenAPI

I used the Utoipa crate to automatically generate OpenAPI specs and host a Swagger UI.


### Caches

Going with two caches:

* L1 Cache: Faster, smaller cache. This will contain copies of the hotter L2 entries and everything needed to respond to a user, including the generic widgets. The personalised data lives in here until it is evicted when the cache deems it no longer useful, while the generic data only lives here for a few seconds before it must be re-fetched from L2. This both keeps the generic data fresh and periodically checks the L2 for updates by the product backends. In my implementation an update to L2 also updates the entry in L1 though.
* L2 Cache: Redis/Valkey e​​vent-driven/push cache: products update the home service when sth new happens instead of the latter polling regularly
  * Exceptions:
    * The generic non-personalised info/recommendations should probably be polled every few minutes as it does not necessarily undergo discrete changes like "user bought product, change recommendations" but more of a "what are the best deals right now".
    * When a user who hasn't been around in a while (and thus isn't cached, nor have they done anything to cause the products to report an update to the cache) opens the page, I have to show them the generic widgets at first while I ask the product backends for data on this person. After that it should work like with anyone else.
  * Push cache:
    * pros:
      * cache stays fresh, updates come as soon as they happen
      * no requests to the product backend when there are no changes to report
    * cons:
      * cache updates happen even when the user is not around & might not see them because a subsequent update overwrites it before the user logs in again
    * fazit:
      * I believe this to be a worthy trade-off. Products are in control of how many updates they push and when. More importantly, I wager users returning after a long time are more likely to have juicy contract upgrades or offers waiting for them. Having a cold cache and serving non-personalised recommendations on this first view could be a massive lost opportunity. For completely cold starts on returning users (see example above) this is harder to avoid, although there is nothing stopping the product backends from pushing a particularly interesting offer to this service even if I never asked for this user's data.
      * This way the only people shown generic data (after a cache warmup cycle) should be newly registered users which likely don't have much in terms of personalised data anyway.
  
Both caches essentially map a (Product, User) tuple to a widget. In my implementation the L1 cache uses exactly such a tuple as key while the L2 cache makes use of Valkey's "hashes".

### Server-driven UI

* pre-rendered HTML would be maximally flexible in web browsers but wouldn't work well with the native phone app if that must be written in kotlin and not a glorified web browser.
* SDUI uses JSON description of UI to communicate the UI between back-end and front-ends. A front-end then renders using this more abstract JSON description. The same front end can render very different looking UIs if the JSON specifies it -> "content updates can and should remain dynamic"
  * To avoid reinventing HTML but in JSON, we must still provide certain templates which can be combined recursively, e.g. carousels, cards, buttons, etc.

## Tech choices

### Home service backend
<!-- TODO yap about Rust -->
* Rust
  * Rust offers the legendary speed of C++ with the memory safety of e.g. Java and has an excellent type system. We can have our performant cake and eat it safely.
* Axum:
  * A fast, estabilished, well-maintained, Rust library. Has a nice existing Tokio/Tower middleware ecosystem (Tokio being the used async runtime).
* moka cache
  * Inspired by Java's caffeine
  * Configurable
  * Importantly: per-entry expiry policies

### Web front end

* NextJS & ShadCN:
  * I'm unfortunately not good at creating things that look nice, using these pre-made components made it much easier
  * It also allows me to pick and choose the components I need and bundle them
    * -> No massive files with entire libraries
    * -> Not pulling in 3rd party code (nice for privacy)
  * NextJS (should) then further optimise this in its build process
  * TailwindCSS also only includes what is actually used

### Android app

* Android studio: pretty much the only choice since I have an android phone and must use Kotlin
* I'm going to be honest, I struggled a lot with this and it is incomplete.

## Addressing requirements

### High traffic, fresh data

Also see [[Oha Benchmark]] below.

* Rust: high performance
* Async design: Rust's safety features and the Tokio Runtime allow async programs to effortlessly spread across cores
* Two caches, one smaller in memory for speed. Invalidation on demand/update, per-entry expiry ensure fresh data without spamming product backends

### Flexibility & Personalisation

* Via SDUI, per-user entries, and instant updates, product backends have full control over who sees what and when.

### Cross-platform consistency

* By using SDUI instead of e.g. pre-rendered HTML, the design specification is stored in a platform agnostic manner and can be rendered the same way anywhere.

### High availability

* The home service itself can be run in multiple instances that share the same Valkey data.
* To this end, the part that actually requests data from the product back ends should be a separate program reading a MPSC queue of requests from the home services, coalesce them, send them to the product backends, and update Valkey with the response.
* For maximum availability, Valkey should be run in its Cluster configuration. This way, every shard has at least one replica that can immediately take over.
* Even if only one home service remains, it can still send out the generic data in its L1 cache.
* (note the PoC cannot do this, it simply returns an HTTP error if there was an error anywhere along the way)

#### Oha Benchmark
[oha](https://crates.io/crates/oha) is a quick and easy HTTP stress testing tool. You can use the command below to generate valid, random get_recommendation requests. I posted the results from my laptop (AMD Ryzen 5 7640U (12) @ 4.972GHz) below.
```
$ oha -z 20s -c 1000 --rand-regex-url "http://127.0.0.1:3000/get_recommendations/[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}"

Summary:
  Success rate:	100.00%
  Total:	20014.9471 ms
  Slowest:	89.4656 ms
  Fastest:	1.0282 ms
  Average:	20.2831 ms
  Requests/sec:	49219.5656

  Total data:	5.26 GiB
  Size/request:	5.60 KiB
  Size/sec:	269.23 MiB

Response time histogram:
   1.028 ms [1]      |
   9.872 ms [37287]  |■■
  18.716 ms [355374] |■■■■■■■■■■■■■■■■■■■■■■■
  27.559 ms [490540] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  36.403 ms [89391]  |■■■■■
  45.247 ms [8201]   |
  54.091 ms [2280]   |
  62.934 ms [1025]   |
  71.778 ms [360]    |
  80.622 ms [94]     |
  89.466 ms [11]     |

Response time distribution:
  10.00% in 12.6426 ms
  25.00% in 16.4408 ms
  50.00% in 20.0526 ms
  75.00% in 23.6799 ms
  90.00% in 27.6859 ms
  95.00% in 30.5010 ms
  99.00% in 37.5169 ms
  99.90% in 57.7203 ms
  99.99% in 72.0464 ms


Details (average, fastest, slowest):
  DNS+dialup:	20.1773 ms, 0.0902 ms, 43.5449 ms
  DNS-lookup:	0.0051 ms, 0.0012 ms, 0.2945 ms

Status code distribution:
  [200] 984564 responses

Error distribution:
  [563] aborted due to deadline
```
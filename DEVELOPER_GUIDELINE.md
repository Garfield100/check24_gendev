# Product integration

To integrate your product you must:

1. Add your product to the enum in services/home_backend/domain/src/product.rs
2. Provide an endpoint of the form /{personalisation} (i.e. a user ID or generic) which returns a JSON SDUI string
3. Specify this endpoint in xxx (not in PoC)
4. Profit

## SDUI Json Schema

In order to be rendered properly, JSON must follow [this](clients/web/app/sdui.ts) schema.
Feel free to visit the [playground](TODO) for an example you can play around with live.
Any images must be provided as assets in the [public](clients/web/public) folder.

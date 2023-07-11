# creditcoin-js

**WARNING**: This is an alpha version of creditcoin-js. It is not ready for production use. It will have breaking changes often.

## Errors & Troubleshooting

If after following the build process you run into errors where credicoin-js isn't reflecting the changes in the rust code you may need to clear your cache. The following command (run from root directory) can help:

```shell
cd creditcoin-js && rm -rf lib && yarn install && yarn build && yarn pack && cd ../integration-tests/ && yarn cache clean && rm -rf node_modules && yarn upgrade creditcoin-js && yarn install
```

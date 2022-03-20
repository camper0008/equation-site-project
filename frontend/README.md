
# SOLIDJS Web Frontend

## Troubleshooting

### Problem with Node.js v17.x

If getting errors looking like:

```
Starting the development server...

Error: error:0308010C:digital envelope routines::unsupported
    (stack trace ...)
  throw err;
  ^

Error: error:0308010C:digital envelope routines::unsupported
    (stack trace ...)
  opensslErrorStack: [ 'error:03000086:digital envelope routines::initialization error' ],
  library: 'digital envelope routines',
  reason: 'unsupported',
  code: 'ERR_OSSL_EVP_UNSUPPORTED'
}
```

Run this command:

```
export NODE_OPTIONS=--openssl-legacy-provider
```

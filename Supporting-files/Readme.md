# .env parameters 
## Host
```
# Pick one
HOST=127.0.0.1        # listen on localhost.
HOST=< IP >           # listen on interface with <IP>.
HOST=0.0.0.0          # listen on all interfaces.
```
# TCP port
```
PORT=3000             # Default
PORT=< Port-Number >  # Custom
```
# Content Security Policy
Only allow content from the site's own origin.
```
HTTP_CSP="default-src 'self'"
```
# Permissions Policy
Disable access to browser features not typically needed by a static site.
```
HTTP_PERMISSIONS_POLICY="camera=(), microphone=(), geolocation=()"
```

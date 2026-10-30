# Static-server-r
This is a simple rust http server for static assets. 

## Version
| Version | Release      | Approver   |
|---------|--------------|------------|
| 1.0.0.0 | 2026-Oct-30  | J Mizzi    |

## build 
You should run cargo build from the root of the repository.

Before running the output binary it is recommended to customise the `/supporting-files/.env` This will allow you to set the HOST IP, TCP port to listen on. 

You can also define a content-security-policy and permissions-policy header.

**The content-security-policy and permissions-policy must be in quotes**
```
HTTP_CSP="default-src 'self'"
```
```
HTTP_PERMISSIONS_POLICY="camera=(), microphone=(), geolocation=()"
```
See `/supporting-files/Readme.md` for all perameters.

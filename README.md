# Turnstile 

> Record key-value-environment triplets and fail builds based on increase or decrease

## Usage
```
turnstile --key npm-vulnerabilities --value 12 --env local
```

Requires a backing storage API. 

## Intention 
Turnstile is intended to be used in CI to monitor values over time and optionally fail a build when those values change. Imagine running
performance tests and failing the build if median response times for your app decrease. Alternately, imagine running scanning third-party packages for known vulnerabilities on every build and failing whenever the vulnerability count increases. 
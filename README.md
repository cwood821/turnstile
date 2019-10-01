# Turnstile 

> Record key-value-environment triplets and fail builds based on increase or decrease

## Usage
```
turnstile --key npm-vulnerabilities --value 12
```

Requires a backing storage API. 

## Intention 
Turnstile is intended to be used in CI to monitor values over time and optionally fail a build when those values change. For example, imagine:
- Run performance tests and fail the build if median response times for your app decrease 
- Scan third-party packages for known vulnerabilities and fail whenever the vulnerability count increases
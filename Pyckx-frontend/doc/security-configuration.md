# Security Configuration

This document describes the security headers and configuration options for the PYCKX frontend application.

## Overview

The application implements comprehensive security headers configured in `next.config.mjs`. Security settings can be controlled via environment variables in the `.env` file.

## Security Headers

All security headers are applied to every route (`/(.*)`).

### Always-On Headers

| Header | Value | Purpose |
|--------|-------|---------|
| `X-Frame-Options` | `SAMEORIGIN` | Prevents clickjacking by only allowing same-origin framing |
| `X-Content-Type-Options` | `nosniff` | Prevents MIME type sniffing attacks |
| `Referrer-Policy` | `strict-origin-when-cross-origin` | Controls referrer information sent with requests |
| `X-XSS-Protection` | `1; mode=block` | Enables XSS filter in older browsers |
| `Permissions-Policy` | Restrictive | Disables camera, microphone, geolocation, FLoC |

### Content Security Policy (CSP)

The CSP is configured for same-origin operation:

```
default-src 'self';
script-src 'self' 'unsafe-inline' 'unsafe-eval' https://va.vercel-scripts.com;
style-src 'self' 'unsafe-inline';
img-src 'self' data: blob:;
font-src 'self' data:;
connect-src 'self';
frame-ancestors 'self';
form-action 'self';
base-uri 'self';
object-src 'none';
```

**Notes:**
- `'unsafe-inline'` and `'unsafe-eval'` are required for Next.js and Tailwind CSS
- `https://va.vercel-scripts.com` is allowed for Vercel Analytics
- All API calls (`connect-src`) are restricted to same origin

### Conditional Headers (Production Only)

#### Strict-Transport-Security (HSTS)

When `SECURITY_MODE=production`:

```
Strict-Transport-Security: max-age=31536000; includeSubDomains; preload
```

**⚠️ WARNING:** Only enable in production with a valid SSL certificate. HSTS forces browsers to use HTTPS for 1 year.

## Environment Variables

### SECURITY_MODE

Controls security header behavior.

| Value | HSTS | Use Case |
|-------|------|----------|
| `development` | ❌ Disabled | Local development without HTTPS |
| `production` | ✅ Enabled | Production deployment with HTTPS |

**Default:** `development`

**Example:**
```bash
# .env
SECURITY_MODE=development  # Local dev
SECURITY_MODE=production   # Production with HTTPS
```

## Configuration File

Security headers are defined in `next.config.mjs`:

```javascript
// Read security mode from environment
const securityMode = process.env.SECURITY_MODE || "development";
const isProductionSecurity = securityMode === "production";

// Headers are conditionally included based on security mode
const productionHeaders = isProductionSecurity
  ? [{ key: "Strict-Transport-Security", value: "..." }]
  : [];
```

## Deployment Checklist

### Development
- [ ] Set `SECURITY_MODE=development`
- [ ] No HTTPS required
- [ ] HSTS disabled

### Production
- [ ] Obtain valid SSL certificate
- [ ] Configure HTTPS on server/load balancer
- [ ] Set `SECURITY_MODE=production`
- [ ] Verify HSTS header is present in responses
- [ ] Test all functionality over HTTPS

## Troubleshooting

### HSTS Issues

If you accidentally enabled HSTS without HTTPS:
1. Clear browser cache and HSTS settings
2. In Chrome: `chrome://net-internals/#hsts` → Delete domain
3. Set `SECURITY_MODE=development` and redeploy

### CSP Violations

Check browser console for CSP violation reports. Common issues:
- External scripts blocked → Add to `script-src`
- External images blocked → Add domain to `img-src`
- API calls blocked → Verify same-origin or add to `connect-src`

## Related Documentation

- [Architecture](./architecture.md)
- [Authentication](./authentication.md)
- [API Configuration](./api-configuration.md)

/** @type {import('next').NextConfig} */

// == Security Mode Configuration // ==
// Read from SECURITY_MODE env variable: "development" or "production"
const securityMode = process.env.SECURITY_MODE || "development";
const isProductionSecurity = securityMode === "production";

// == Build Mode Configuration // ==
// Read from BUILD_MODE env variable: "development", "standalone", or "export"
// - "development": Normal dev mode with proxy
// - "standalone": Next.js standalone server (for Node.js deployment)
// - "export": Static HTML export (for Rust backend serving)
const buildMode = process.env.BUILD_MODE || "development";

const nextConfig = {
  // == Output Configuration // ==
  // Set to "export" for static HTML generation (served by Rust backend)
  // Use: bun run build:static (exports to ./out, then copies to backend/static)
  ...(buildMode === "export" && { output: "export", distDir: "out" }),

  // == Body Size Limit // ==
  // Allow larger request bodies for server actions (100MB)
  experimental: {
    serverActions: {
      bodySizeLimit: '100mb',
    },
  },

  // == Remove console.log in production builds // ==
  // This strips all console.log statements when building for production
  compiler: {
    removeConsole: buildMode === "export",
  },

  // == TypeScript Configuration // ==
  // Enforce strict TypeScript compilation - do not ignore errors
  typescript: {
    ignoreBuildErrors: false,
  },

  // == Image Configuration // ==
  images: {
    unoptimized: true,
  },

  // == Trailing Slash // ==
  // Required for static export to work correctly with SPA routing
  trailingSlash: buildMode === "export",

  // == Security Headers // ==
  // Configured for same-origin backend/frontend deployment
  async headers() {
    // Base security headers (always applied)
    const baseHeaders = [
      // Prevent clickjacking attacks
      {
        key: "X-Frame-Options",
        value: "SAMEORIGIN",
      },
      // Prevent MIME type sniffing
      {
        key: "X-Content-Type-Options",
        value: "nosniff",
      },
      // Control referrer information
      {
        key: "Referrer-Policy",
        value: "strict-origin-when-cross-origin",
      },
      // Enable XSS filter in older browsers
      {
        key: "X-XSS-Protection",
        value: "1; mode=block",
      },
      // Restrict browser features/APIs
      {
        key: "Permissions-Policy",
        value: "camera=(), microphone=(), geolocation=(), interest-cohort=()",
      },
      // Content Security Policy - same-origin focused
      // In development, allow connections to backend on different port
      {
        key: "Content-Security-Policy",
        value: [
          "default-src 'self'",
          "script-src 'self' 'unsafe-inline' 'unsafe-eval' https://va.vercel-scripts.com",
          "style-src 'self' 'unsafe-inline'",
          "img-src 'self' data: blob:",
          "font-src 'self' data:",
          // Allow connections to same origin (backend serves frontend)
          "connect-src 'self'",
          "frame-ancestors 'self'",
          "form-action 'self'",
          "base-uri 'self'",
          "object-src 'none'",
        ].join("; "),
      },
    ];

    // Production-only security headers (HTTPS required)
    const productionHeaders = isProductionSecurity
      ? [
        // Enforce HTTPS - only enable with valid SSL certificate
        {
          key: "Strict-Transport-Security",
          value: "max-age=31536000; includeSubDomains; preload",
        },
      ]
      : [];

    // Log security mode for debugging
    console.log(`[Security] Mode: ${securityMode}, HSTS: ${isProductionSecurity ? "enabled" : "disabled"}`);

    return [
      {
        // Apply security headers to all routes
        source: "/(.*)",
        headers: [...baseHeaders, ...productionHeaders],
      },
    ];
  },
};

export default nextConfig;


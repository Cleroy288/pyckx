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
  // Only enable for production builds: BUILD_MODE=export bun run build
  ...(buildMode === "export" && { output: "export", distDir: "out" }),

  // == Body Size Limit // ==
  // Allow larger request bodies for server actions (100MB)
  experimental: {
    serverActions: {
      bodySizeLimit: '100mb',
    },
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
          // Allow connections to self and backend (for large file uploads that bypass proxy)
          `connect-src 'self' ${process.env.NEXT_PUBLIC_BACKEND_DIRECT_URL || "http://localhost:8080"}`,
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

  // == API Proxy Configuration // ==
  // Proxy API requests to backend during development
  async rewrites() {
    // Only apply rewrites in development
    const backendUrl = process.env.BACKEND_URL || "http://localhost:8080";
    
    return {
      // beforeFiles rewrites are checked before pages/public files
      // which allows proxying to work even with /app prefix
      beforeFiles: [
        {
          source: "/app/collection/:path*",
          destination: `${backendUrl}/app/collection/:path*`,
        },
        {
          source: "/app/intello/:path*",
          destination: `${backendUrl}/app/intello/:path*`,
        },
        {
          source: "/auth/:path*",
          destination: `${backendUrl}/auth/:path*`,
        },
        {
          source: "/user/:path*",
          destination: `${backendUrl}/user/:path*`,
        },
        {
          source: "/api/:path*",
          destination: `${backendUrl}/api/:path*`,
        },
      ],
    };
  },
};

export default nextConfig;

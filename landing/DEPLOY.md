# Landing page deployment — `silo.hllmr.com`

The landing page is a single self-contained static file: `landing/index.html` (no build
step, no external assets, no JS deps — inline CSS/JS only). Serve it at
**`silo.hllmr.com`**.

## Recommendation: isolated static container (not the SiloAPI container)

Keep it **separate** from the SiloAPI container. Reasons: the landing page and the API
have independent deploy cadences (marketing copy changes shouldn't redeploy the API, and
vice-versa); the API is Node/Fastify and shouldn't grow a static-file responsibility; and
an isolated `nginx:alpine` serving one HTML file is trivial and rock-solid. It also keeps
`silo.hllmr.com` (marketing) and `silo-api.hllmr.com` (data) cleanly decoupled.

## docker-compose service

```yaml
  silo-landing:
    image: nginx:alpine
    container_name: silo-landing
    restart: unless-stopped
    volumes:
      - ./silo-landing:/usr/share/nginx/html:ro
    # No published port — Nginx Proxy Manager reaches it over the shared docker network.
    networks:
      - proxy
```

Drop `landing/index.html` into `./silo-landing/index.html` on the host (rename to
`index.html` so nginx serves it at `/`). Ensure the service shares whatever docker
network NPM uses to reach the other containers (e.g. `proxy`).

## Nginx Proxy Manager

Add a **Proxy Host**:
- Domain: `silo.hllmr.com`
- Forward to: `silo-landing` : `80` (scheme `http`)
- Enable **Block Common Exploits** and **Websockets** off (not needed).
- SSL: request a Let's Encrypt cert for `silo.hllmr.com`, **Force SSL** + HTTP/2 on.

DNS: point an `A`/`AAAA` (or `CNAME` to the existing host) for `silo` at the VPS.

## Updating

The page is one file. To update: replace `./silo-landing/index.html` with the latest
`landing/index.html` from the repo (`git pull` then copy, or a tiny deploy hook). No
container rebuild needed — nginx serves the new file immediately. Cache: the file sets no
long max-age, but if you add one later, bust on deploy.

## Optional niceties (not required for launch)

- A `/robots.txt` and `/sitemap.xml` (single URL) for SEO.
- Basic security headers via an NPM custom Nginx config (CSP is loose here since the page
  is self-hosted and self-contained; `X-Content-Type-Options: nosniff`,
  `Referrer-Policy: strict-origin-when-cross-origin` are cheap wins).

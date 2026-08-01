# Landing page deployment — `silo.hllmr.com`

The site is a small set of self-contained static files under `landing/` (no build step, no
external asset hosts, inline CSS/JS only). Serve the **whole `landing/` directory** at
**`silo.hllmr.com`**:

- `index.html` — the landing page (references `screenshots/*.png`, `og.png`, favicons).
- `browse/index.html` — the public mod catalog browser at **`silo.hllmr.com/browse/`**. It's
  a client-side page that reads the public SiloAPI (`https://silo-api.hllmr.com`) directly
  from the browser — no server-side rendering, no build. CORS is already open on the API.
- `screenshots/`, `og.png`, `og.svg`, favicons, `robots.txt`, `sitemap.xml`, `llms.txt`.

> **Important:** sync the entire folder, not just `index.html`. The revamped landing embeds
> real screenshots from `screenshots/`, so an `index.html`-only copy would render with broken
> images.

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

Sync the **entire `landing/` folder** into `./silo-landing/` on the host (so
`./silo-landing/index.html`, `./silo-landing/browse/index.html`, `./silo-landing/screenshots/…`
all exist). nginx serves `/` → `index.html` and `/browse/` → `browse/index.html`
automatically. Ensure the service shares whatever docker network NPM uses to reach the other
containers (e.g. `proxy`).

## Nginx Proxy Manager

Add a **Proxy Host**:
- Domain: `silo.hllmr.com`
- Forward to: `silo-landing` : `80` (scheme `http`)
- Enable **Block Common Exploits** and **Websockets** off (not needed).
- SSL: request a Let's Encrypt cert for `silo.hllmr.com`, **Force SSL** + HTTP/2 on.

DNS: point an `A`/`AAAA` (or `CNAME` to the existing host) for `silo` at the VPS.

## Updating

To update: re-sync the `landing/` folder into `./silo-landing/` from the repo (`git pull`
then `rsync -a --delete landing/ ./silo-landing/`, or a tiny deploy hook). No container
rebuild needed — nginx serves the new files immediately. Cache: the files set no long
max-age, but if you add one later, bust on deploy. `/browse/` needs no special handling —
it's static and fetches its data client-side.

## Optional niceties (not required for launch)

- A `/robots.txt` and `/sitemap.xml` (single URL) for SEO.
- Basic security headers via an NPM custom Nginx config (CSP is loose here since the page
  is self-hosted and self-contained; `X-Content-Type-Options: nosniff`,
  `Referrer-Policy: strict-origin-when-cross-origin` are cheap wins).

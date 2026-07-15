<script lang="ts">
  import {
    ghStatus,
    ghSetClientId,
    ghDeviceStart,
    ghDevicePoll,
    ghLogout,
    openExternal,
  } from "../api";
  import type { GhStatus } from "../types";

  let status = $state<GhStatus>({ clientId: null, user: null });
  let clientIdInput = $state("");
  let flow = $state<{ userCode: string; verificationUri: string } | null>(null);
  let message = $state<string | null>(null);
  let poller: ReturnType<typeof setTimeout> | undefined;

  async function refresh() {
    try {
      status = await ghStatus();
      clientIdInput = status.clientId ?? "";
    } catch (e) {
      message = String(e);
    }
  }
  refresh();

  async function saveClientId() {
    try {
      await ghSetClientId(clientIdInput.trim());
      await refresh();
      message = null;
    } catch (e) {
      message = String(e);
    }
  }

  async function connect() {
    message = null;
    try {
      const dc = await ghDeviceStart();
      flow = { userCode: dc.userCode, verificationUri: dc.verificationUri };
      openExternal(dc.verificationUri).catch(() => {});
      let interval = Math.max(dc.interval, 3);
      const tick = async () => {
        try {
          const r = await ghDevicePoll(dc.deviceCode);
          if (r.status === "ok") {
            flow = null;
            await refresh();
            return;
          }
          if (r.status === "slow_down") interval += 5;
          if (r.status === "expired" || r.status === "denied" || r.status === "error") {
            flow = null;
            message =
              r.status === "denied"
                ? "Authorization was denied."
                : r.status === "expired"
                  ? "The code expired — try again."
                  : `Auth error: ${r.error ?? "unknown"}`;
            return;
          }
          poller = setTimeout(tick, interval * 1000);
        } catch (e) {
          flow = null;
          message = String(e);
        }
      };
      poller = setTimeout(tick, interval * 1000);
    } catch (e) {
      message = String(e);
    }
  }

  async function disconnect() {
    try {
      await ghLogout();
      await refresh();
    } catch (e) {
      message = String(e);
    }
  }

  $effect(() => () => clearTimeout(poller));
</script>

<div class="gha">
  {#if status.user}
    <div class="gha-row">
      <div>
        <div class="gha-connected">✓ Connected as {status.user}</div>
        <div class="gha-hint">Authenticated requests: 5,000/hr.</div>
      </div>
      <button class="gha-btn" onclick={disconnect}>Disconnect</button>
    </div>
  {:else if flow}
    <div class="gha-flow">
      <div class="gha-hint">1. A browser opened to <b>github.com/login/device</b>. Enter this code:</div>
      <div class="gha-code tnum">{flow.userCode}</div>
      <div class="gha-hint">Waiting for you to authorize… keep this open.</div>
    </div>
  {:else if status.clientId}
    <div class="gha-row">
      <div class="gha-hint">Connect your GitHub account for higher rate limits and private repos.</div>
      <button class="gha-btn primary" onclick={connect}>Connect GitHub</button>
    </div>
  {:else}
    <div class="gha-setup">
      <div class="gha-hint">
        One-time setup: register a GitHub OAuth App (enable “Device Flow”), then paste its Client ID.
        <button class="gha-link" onclick={() => openExternal("https://github.com/settings/applications/new")}>
          Register an OAuth App ↗
        </button>
      </div>
      <div class="gha-row">
        <input class="gha-input" placeholder="OAuth App Client ID (Iv1.…)" bind:value={clientIdInput} />
        <button class="gha-btn" onclick={saveClientId}>Save</button>
      </div>
    </div>
  {/if}
  {#if message}<div class="gha-msg">{message}</div>{/if}
</div>

<style>
  .gha {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .gha-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
  }
  .gha-hint {
    font-size: 12px;
    color: var(--text-muted);
    line-height: 1.5;
  }
  .gha-connected {
    font-size: 13px;
    font-weight: 600;
    color: var(--primary);
  }
  .gha-btn {
    flex: 0 0 auto;
    border: 1px solid var(--border);
    background: var(--bg);
    color: var(--text);
    padding: 7px 14px;
    border-radius: var(--radius-sm);
    font-size: 12.5px;
    font-weight: 600;
  }
  .gha-btn.primary {
    background: var(--primary);
    border-color: var(--primary);
    color: var(--on-primary);
  }
  .gha-btn:hover:not(.primary) {
    border-color: color-mix(in srgb, var(--primary) 45%, var(--border));
    color: var(--primary);
  }
  .gha-link {
    border: none;
    background: transparent;
    color: var(--info);
    font-size: 12px;
    font-weight: 600;
    padding: 0;
  }
  .gha-link:hover {
    text-decoration: underline;
  }
  .gha-input {
    flex: 1 1 auto;
    min-width: 0;
    border: 1px solid var(--border);
    background: var(--bg);
    color: var(--text);
    border-radius: var(--radius-sm);
    padding: 7px 10px;
    font-size: 12.5px;
    font-family: inherit;
  }
  .gha-setup {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .gha-flow {
    display: flex;
    flex-direction: column;
    gap: 8px;
    align-items: center;
    padding: 6px 0;
  }
  .gha-code {
    font-size: 26px;
    font-weight: 700;
    letter-spacing: 4px;
    color: var(--accent);
    background: var(--bg);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    padding: 8px 16px;
  }
  .gha-msg {
    font-size: 12px;
    color: var(--danger);
  }
</style>

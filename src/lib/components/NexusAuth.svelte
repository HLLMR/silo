<script lang="ts">
  import { nexusStatus, nexusSetKey, nexusLogout, openExternal } from "../api";
  import type { NexusStatus } from "../types";

  let status = $state<NexusStatus>({ user: null });
  let keyInput = $state("");
  let showKey = $state(false);
  let message = $state<string | null>(null);
  let busy = $state(false);

  async function refresh() {
    try {
      status = await nexusStatus();
    } catch (e) {
      message = String(e);
    }
  }
  refresh();

  async function save() {
    message = null;
    busy = true;
    try {
      await nexusSetKey(keyInput.trim());
      keyInput = "";
      showKey = false;
      await refresh();
    } catch (e) {
      message = String(e);
    } finally {
      busy = false;
    }
  }

  async function disconnect() {
    try {
      await nexusLogout();
      await refresh();
    } catch (e) {
      message = String(e);
    }
  }
</script>

<div class="nxa">
  {#if status.user}
    <div class="nxa-row">
      <div>
        <div class="nxa-connected">✓ Connected as {status.user}</div>
        <div class="nxa-hint">Endorse mods from their detail cards.</div>
      </div>
      <button class="nxa-btn" onclick={disconnect}>Disconnect</button>
    </div>
  {:else}
    <div class="nxa-hint">
      Paste your Nexus <b>personal API key</b> to endorse mods through your account.
      The key stays on this machine.
      <button
        class="nxa-link"
        onclick={() => openExternal("https://www.nexusmods.com/users/myaccount?tab=api")}
      >
        Get your key ↗
      </button>
    </div>
    {#if showKey}
      <div class="nxa-row">
        <input
          class="nxa-input"
          type="password"
          placeholder="Nexus personal API key"
          bind:value={keyInput}
        />
        <button class="nxa-btn" onclick={save} disabled={busy || !keyInput.trim()}>
          {busy ? "Checking…" : "Save key"}
        </button>
      </div>
    {:else}
      <button class="nxa-btn primary" onclick={() => (showKey = true)}>Connect Nexus</button>
    {/if}
  {/if}
  {#if message}<div class="nxa-msg">{message}</div>{/if}
</div>

<style>
  .nxa {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .nxa-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
  }
  .nxa-hint {
    font-size: 12px;
    color: var(--text-muted);
    line-height: 1.5;
  }
  .nxa-connected {
    font-size: 13px;
    font-weight: 600;
    color: var(--primary);
  }
  .nxa-btn {
    flex: 0 0 auto;
    border: 1px solid var(--border);
    background: var(--bg);
    color: var(--text);
    padding: 7px 14px;
    border-radius: var(--radius-sm);
    font-size: 12.5px;
    font-weight: 600;
  }
  .nxa-btn.primary {
    background: var(--primary);
    border-color: var(--primary);
    color: var(--on-primary);
  }
  .nxa-btn:hover:not(.primary):not(:disabled) {
    border-color: color-mix(in srgb, var(--primary) 45%, var(--border));
    color: var(--primary);
  }
  .nxa-btn:disabled {
    opacity: 0.6;
  }
  .nxa-link {
    border: none;
    background: transparent;
    color: var(--info);
    font-size: 12px;
    font-weight: 600;
    padding: 0;
  }
  .nxa-link:hover {
    text-decoration: underline;
  }
  .nxa-input {
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
  .nxa-msg {
    font-size: 12px;
    color: var(--danger);
  }
</style>

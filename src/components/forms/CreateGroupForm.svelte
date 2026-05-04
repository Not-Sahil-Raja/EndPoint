<script lang="ts">
  import IconFolderPlusBold from "phosphor-icons-svelte/IconFolderPlusBold.svelte";

  let {
    onCreateGroup,
    onClose,
    closeDialog,
  }: {
    onCreateGroup: (name: string) => Promise<void>;
    onClose: () => void;
    closeDialog?: () => void;
  } = $props();

  let name = $state("");
  let loading = $state(false);
  let error = $state("");

  async function handleSubmit(e: Event) {
    e.preventDefault();
    const trimmedName = name.trim();
    if (!trimmedName) return;
    loading = true;
    error = "";
    try {
      await onCreateGroup(trimmedName);
      name = "";
      onClose();
      closeDialog?.();
    } catch (err: any) {
      console.error("[CreateGroupForm] createGroup error:", err);
      error = err?.message ?? "Failed to create group.";
    } finally {
      loading = false;
    }
  }
</script>

<form onsubmit={handleSubmit} class="flex flex-col gap-5 font-jetbrains">
  <div class="flex flex-col gap-3">
    <div class="flex flex-col gap-1.5">
      <label
        for="group-name"
        class="text-[11px] uppercase tracking-widest text-white/40"
      >
        Group Name
      </label>
      <div
        class="flex items-center gap-2.5 px-3 py-2.5 border border-white/10 bg-white/4 focus-within:border-indigo-500/60 focus-within:bg-white/6 transition-all"
      >
        <IconFolderPlusBold class="w-3.5 h-3.5 text-white/20 shrink-0" />
        <input
          id="group-name"
          type="text"
          bind:value={name}
          placeholder="e.g. Production"
          required
          class="flex-1 bg-transparent text-sm text-white placeholder:text-white/20 outline-none"
        />
      </div>
    </div>
  </div>

  {#if error}
    <p class="text-xs text-rose-400">{error}</p>
  {/if}

  <div class="flex items-center justify-end gap-2 pt-8">
    <button
      type="button"
      onclick={() => {
        onClose();
        closeDialog?.();
      }}
      class="px-4 py-2 text-sm text-white/40 hover:text-white/70 border border-white/10 hover:border-white/20 hover:bg-white/5 transition-all cursor-pointer"
    >
      Cancel
    </button>
    <button
      type="submit"
      disabled={loading}
      class="px-4 py-2 uppercase text-sm font-medium bg-indigo-600 hover:bg-indigo-500 border border-indigo-400/30 text-white transition-all cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
    >
      {loading ? "Creating…" : "Create Group"}
    </button>
  </div>
</form>

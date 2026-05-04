<script>
  import CornerBorders from "../ui/CornerBorders.svelte";

  let { showModal = $bindable(), header, children } = $props();
  let dialog = $state();

  function closeDialog() {
    dialog.close();
  }

  $effect(() => {
    if (showModal) dialog.showModal();
  });
</script>

<dialog
  bind:this={dialog}
  onclose={() => (showModal = false)}
  onclick={(e) => {
    if (e.target === dialog) dialog.close();
  }}
>
  <div class="modal-inner">
    <CornerBorders class="p-6">
      {@render header?.(closeDialog)}
      {@render children?.(closeDialog)}
    </CornerBorders>
  </div>
</dialog>

<style>
  dialog {
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 0;
    background: #1a1a1b;
    padding: 0;
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    margin: 0;
    width: 480px;
    max-width: 95vw;
    box-shadow: 0 25px 60px rgba(0, 0, 0, 0.6);
    overflow: visible;
  }
  dialog::backdrop {
    background: rgba(0, 0, 0, 0.75);
    backdrop-filter: blur(200px);
    -webkit-backdrop-filter: blur(200px);
  }
  .modal-inner {
    padding: 0;
    color: white;
    border-radius: 0;
    position: relative;
    max-height: 90vh;
    overflow: visible;
  }
  dialog[open] {
    animation: zoom 0.25s cubic-bezier(0.34, 1.56, 0.64, 1);
  }
  @keyframes zoom {
    from {
      opacity: 0;
      transform: translate(-50%, -50%) scale(0.95);
    }
    to {
      opacity: 1;
      transform: translate(-50%, -50%) scale(1);
    }
  }
  dialog[open]::backdrop {
    animation: fade-blur 0.2s ease-out;
  }
  @keyframes fade-blur {
    from {
      opacity: 0;
      backdrop-filter: blur(0px);
    }
    to {
      opacity: 1;
      backdrop-filter: blur(200px);
    }
  }
</style>

<script>
  let { showModal = $bindable(), header, children } = $props();

  let dialog = $state();

  $effect(() => {
    if (showModal) dialog.showModal();
  });
</script>

<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_noninteractive_element_interactions -->
<dialog
  bind:this={dialog}
  onclose={() => (showModal = false)}
  onclick={(e) => {
    if (e.target === dialog) dialog.close();
  }}
>
  <div>
    {@render header?.()}

    {@render children?.()}

    <!-- svelte-ignore a11y_autofocus -->
    <button onclick={() => dialog.close()}>Close</button>
  </div>
</dialog>

<style>
  dialog {
    max-width: 32em;
    border-radius: 1rem;
    background-color: rgb(54, 54, 54);
    border: none;
    padding: 0;
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    margin: 0;
  }
  dialog::backdrop {
    background: rgba(0, 0, 0, 0.3);
  }
  dialog > div {
    padding: 1em;
    background-color: rgb(54, 54, 54);
    color: white;
    border-radius: 0.5em;
  }
  dialog[open] {
    animation: zoom 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
  }
  @keyframes zoom {
    from {
      opacity: 0;
      transform: translate(-50%, -50%) scale(0.9);
    }
    to {
      opacity: 1;
      transform: translate(-50%, -50%) scale(1);
    }
  }
  dialog[open]::backdrop {
    animation: fade 0.2s ease-out;
  }
  @keyframes fade {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }
  button {
    display: block;
    margin-top: 1em;
    padding: 0.5em 1em;
    margin-left: auto;
    margin-right: auto;
    border: none;
    border-radius: 0.25em;
    cursor: pointer;
  }
</style>

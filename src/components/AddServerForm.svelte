<script lang="ts">
  let {
    onAddServer,
    onClose,
  }: {
    onAddServer: (url: string, name: string) => Promise<void>;
    onClose: () => void;
  } = $props();

  function handleSubmit(e: Event) {
    e.preventDefault();
    const form = e.target as HTMLFormElement;
    const address = (form.querySelector("#server-address") as HTMLInputElement)
      .value;
    const name = (form.querySelector("#server-name") as HTMLInputElement).value;
    onAddServer(address, name).then(() => {
      onClose();
      form.reset();
    });
  }
</script>

<form class="flex flex-col gap-3 w-full min-w-3xs" onsubmit={handleSubmit}>
  <div class="field">
    <label for="server-address">Server Address</label>
    <input
      id="server-address"
      class="input"
      type="url"
      placeholder="e.g., https://example.com"
      required
    />
  </div>

  <div class="field">
    <label for="server-name">Server Name</label>
    <input
      id="server-name"
      class="input"
      type="text"
      placeholder="e.g., My Server"
      required
    />
  </div>
  <button type="submit" class="btn btn-primary">Add Server</button>
</form>

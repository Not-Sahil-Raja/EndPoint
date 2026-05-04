<script lang="ts">
  interface Props {
    class?: string;
    style?: string;
    children?: any;
  }

  let { class: className = "", style, children }: Props = $props();
</script>

<div class="relative {className}" {style}>
  <div
    class="absolute -left-0.5 -top-0.5 h-3 w-3 border-l-2 border-t-2 border-white/30"
  ></div>
  <div
    class="absolute -right-0.5 -top-0.5 h-3 w-3 border-r-2 border-t-2 border-white/30"
  ></div>
  <div
    class="absolute -bottom-0.5 -left-0.5 h-3 w-3 border-b-2 border-l-2 border-white/30"
  ></div>
  <div
    class="absolute -bottom-0.5 -right-0.5 h-3 w-3 border-b-2 border-r-2 border-white/30"
  ></div>

  {@render children?.()}
</div>

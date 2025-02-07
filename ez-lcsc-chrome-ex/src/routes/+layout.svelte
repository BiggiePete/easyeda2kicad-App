<script lang="ts">
	import '../app.css';
	let { children } = $props();
</script>

<div class="w-96">
	{@render children()}
</div>

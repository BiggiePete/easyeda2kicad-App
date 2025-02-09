<script lang="ts">
	import '../app.css';
	import { ModeWatcher } from 'mode-watcher';
	let { children } = $props();
</script>

<ModeWatcher></ModeWatcher>
<div class="h-[30rem] w-96">
	{@render children()}
</div>

<script>
	import '../app.css';
	import { Toaster } from '$lib/components/ui/sonner/index.js';
	import { ModeWatcher } from 'mode-watcher';
	let { children } = $props();
</script>

<Toaster />
<ModeWatcher></ModeWatcher>
<div class="h-[30rem] w-96">
	{@render children()}
</div>

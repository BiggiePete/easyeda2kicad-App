<script>
	import '../app.css';
	import { ModeWatcher } from 'mode-watcher';
	import LightSwitch from '$lib/components/custom/lightSwitch.svelte';
	let { children } = $props();
</script>

<ModeWatcher></ModeWatcher>
<div class="w-96">
	<div class="w-full">
		<span>Thing</span>
		<div class="flex flex-row-reverse">
			<LightSwitch></LightSwitch>
		</div>
	</div>
	{@render children()}
</div>

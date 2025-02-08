<script>
	import '../app.css';
	import { ModeWatcher } from 'mode-watcher';
	import LightSwitch from '$lib/components/custom/lightSwitch.svelte';
	let { children } = $props();
</script>

<ModeWatcher></ModeWatcher>
<div class="h-[30rem] w-96">
	<div class="grid w-full grid-cols-2">
		<span>Thing</span>
		<div class="flex flex-row-reverse">
			<LightSwitch></LightSwitch>
		</div>
	</div>
	{@render children()}
</div>

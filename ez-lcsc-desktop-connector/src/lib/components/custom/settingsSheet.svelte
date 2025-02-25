<script lang="ts">
	import * as Sheet from '$lib/components/ui/sheet/index.js';
	import { buttonVariants } from '$lib/components/ui/button/index.js';
	import { Settings } from 'lucide-svelte';
	import { Checkbox } from '$lib/components/ui/checkbox';
	import { Label } from '$lib/components/ui/label/index.js';
	import { ScrollArea } from '$lib/components/ui/scroll-area';
	import { enable, isEnabled, disable } from 'tauri-plugin-autostart-api';
	import { onMount } from 'svelte';

	// get settings state
	// await enable();

	// disable();
	let checked = $state(false);
	onMount(async () => {
		checked = await isEnabled();
	});
	$effect(() => {
		checked ? enable() : disable();
		isEnabled().then((v) => {
			console.log(`state = ${v}`);
		});
	});
</script>

<Sheet.Root>
	<Sheet.Trigger class={buttonVariants({ variant: 'outline' })}>
		<Settings></Settings>
	</Sheet.Trigger>
	<Sheet.Content side="right">
		<Sheet.Header>
			<Sheet.Title>Edit Settings</Sheet.Title>
			<Sheet.Description>Edit relivant settings and options for the application</Sheet.Description>
		</Sheet.Header>
		<div class="h-6"></div>
		<ScrollArea>
			<div class="items-top flex space-x-2">
				<Checkbox id="autostart" bind:checked />
				<div class="grid gap-1.5 leading-none">
					<Label
						for="autostart"
						class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
					>
						Auto-Start with Windows
					</Label>
					<p class="text-sm text-muted-foreground">
						You want this application to start automatically with windows on boot.
					</p>
				</div>
			</div>
		</ScrollArea>
		<Sheet.Footer>
			<Sheet.Close class={buttonVariants({ variant: 'outline' })} type="submit">Exit</Sheet.Close>
		</Sheet.Footer>
	</Sheet.Content>
</Sheet.Root>

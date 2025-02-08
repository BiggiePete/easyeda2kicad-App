<script lang="ts">
	import LightSwitch from '$lib/components/custom/lightSwitch.svelte';
	import * as Card from '$lib/components/ui/card';
</script>

<div class="container w-full">
	<Card.Root>
		<Card.Header>
			<Card.Title>
				<div class="grid grid-cols-2">
					<h2>LCSC 2 KiCAD</h2>
					<div class="flex flex-row-reverse">
						<LightSwitch></LightSwitch>
					</div>
				</div>
			</Card.Title>
			<Card.Description>Card Description</Card.Description>
		</Card.Header>
		<Card.Content>
			<p>Card Content</p>
		</Card.Content>
		<Card.Footer>
			<p>Card Footer</p>
		</Card.Footer>
	</Card.Root>
</div>

<script lang="ts">
	import { onMount } from 'svelte';
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';

	onMount(getProjectList);

	async function getProjectList() {
		const r = await fetch('http://localhost:3030/api/getProjectList', {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json'
			},
			mode: 'cors' // Ensure the request respects CORS
		});
	}
</script>

<div class="container m-auto w-full">
	<Card.Root>
		<Card.Header>
			<Card.Title>Add Part To Project</Card.Title>
			<Card.Description>Card Description</Card.Description>
		</Card.Header>
		<Card.Content>
			<Button onclick={getProjectList}>Add 2 Project</Button>
		</Card.Content>
		<Card.Footer>
			<p>Card Footer</p>
		</Card.Footer>
	</Card.Root>
</div>

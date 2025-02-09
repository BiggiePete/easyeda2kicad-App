<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { onMount } from 'svelte';
</script>

<div class="container m-auto w-full">
	<Card.Root>
		<Card.Header>
			<Card.Title>Add Part To Project</Card.Title>
			<Card.Description>Card Description</Card.Description>
		</Card.Header>
		<Card.Content>
			<Button>Add 2 Project</Button>
		</Card.Content>
		<Card.Footer></Card.Footer>
	</Card.Root>
</div>

<script lang="ts">
	import LightSwitch from '$lib/components/custom/lightSwitch.svelte';
	import * as Card from '$lib/components/ui/card';
	import * as Table from '$lib/components/ui/table';
	import { invoke } from '@tauri-apps/api/tauri';
	import type { Project } from './types';
	import { Button } from '$lib/components/ui/button';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import { Input } from '$lib/components/ui/input';

	const projects_ = invoke('get_projects_invoke') as Promise<Project[]>;
</script>

<div class="container w-screen">
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
			<Card.Description>Below you will find a list of all your projects</Card.Description>
		</Card.Header>
		<Card.Content>
			<Table.Root>
				<Table.Caption>A List of all current & active projects</Table.Caption>
				<Table.Header>
					<Table.Row>
						<Table.Head class="w-[100px]">Project Name</Table.Head>
						<Table.Head class="text-center">Add LCSC</Table.Head>
						<Table.Head class="text-right">Delete</Table.Head>
					</Table.Row>
				</Table.Header>
				<Table.Body>
					{#await projects_}
						<div class="flex items-center space-x-4">
							<Skeleton class="h-8 w-[300px]" />
							<div class="space-y-2">
								<Skeleton class="h-4 w-[250px]" />
								<Skeleton class="h-4 w-[250px]" />
								<Skeleton class="h-4 w-[250px]" />
								<Skeleton class="h-4 w-[250px]" />
							</div>
						</div>
					{:then projects}
						{#each projects as p}
							<Table.Row>
								<Table.Cell class="font-medium">{p.proj_name}</Table.Cell>
								<Table.Cell>
									<div class="grid grid-cols-3 gap-2">
										<Input class="col-span-2" />

										<Button variant="secondary">Add To Project</Button>
									</div>
								</Table.Cell>
								<Table.Cell class="text-right">
									<Button variant="destructive">Delete</Button>
								</Table.Cell>
							</Table.Row>
						{/each}
					{/await}
				</Table.Body>
			</Table.Root>
		</Card.Content>
		<Card.Footer>
			<p>Card Footer</p>
		</Card.Footer>
	</Card.Root>
</div>

<script lang="ts">
	import LightSwitch from '$lib/components/custom/lightSwitch.svelte';
	import * as Card from '$lib/components/ui/card';
	import * as Table from '$lib/components/ui/table';
	import { invoke } from '@tauri-apps/api/tauri';
	import type { Project } from './types';
	import { Button } from '$lib/components/ui/button';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import { Input } from '$lib/components/ui/input';
	import { Plus, Trash, LoaderCircle, RefreshCw, Settings } from 'lucide-svelte';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import { toast } from 'svelte-sonner';
	import SettingsSheet from '$lib/components/custom/settingsSheet.svelte';
	let projects_ = invoke('get_projects_invoke') as Promise<Project[]>;
	let importer = $state('');
	let isImporting = $state(false);
	let isAddingProject = $state(false);
	let removingProjectId = $state('');
</script>

<div class="container m-auto mt-4">
	<Card.Root>
		<Card.Header>
			<Card.Title>
				<div class="grid grid-cols-2">
					<h2>LCSC 2 KiCAD</h2>
					<div class="flex flex-row-reverse">
						<SettingsSheet></SettingsSheet>
						<LightSwitch></LightSwitch>
					</div>
				</div>
			</Card.Title>
			<Card.Description>Below you will find a list of all your projects</Card.Description>
		</Card.Header>
		<Card.Content>
			<div class="flex flex-row-reverse">
				<Button
					onclick={() => {
						window.location.reload();
					}}><RefreshCw /> Refresh</Button
				>
				{#if isAddingProject}
					<Button disabled>
						<LoaderCircle class="animate-spin" />
						Follow Prompts
					</Button>
				{:else}
					<Button
						onclick={async () => {
							invoke('add_project_invoke').then((v) => {
								isAddingProject = false;
								window.location.reload();
							});
							isAddingProject = true;
						}}><Plus />Add Project</Button
					>
				{/if}
			</div>
			<Table.Root>
				<Table.Caption>A List of all current & active projects</Table.Caption>
				<Table.Header>
					<Table.Row>
						<Table.Head class="text-left">Project Name</Table.Head>
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
								<Table.Cell class="font-medium">
									<Tooltip.Provider>
										<Tooltip.Root>
											<Tooltip.Trigger
												onclick={() => {
													invoke('open_build_dir_invoke', { dir: p.dir });
												}}>{p.proj_name}</Tooltip.Trigger
											>
											<Tooltip.Content>Click me to open this project's directory</Tooltip.Content>
										</Tooltip.Root>
									</Tooltip.Provider>
								</Table.Cell>
								<Table.Cell>
									<div class="grid grid-cols-4 gap-2">
										<Input class="col-span-3" bind:value={importer} />

										{#if isImporting}
											<Button disabled variant="secondary">
												<LoaderCircle class="animate-spin" /> Importing
											</Button>
										{:else}
											<Tooltip.Provider>
												<Tooltip.Root>
													<Tooltip.Trigger>
														<Button
															variant="secondary"
															onclick={() => {
																isImporting = true;
																invoke('add_part_by_lcsc_invoke', { id: p.id, c: importer }).then(
																	(v) => {
																		if (v == 0) {
																			toast.success('Part added to ' + p.proj_name, {
																				description: 'LCSC ' + importer + ' Successfully added'
																			});
																			importer = '';
																			isImporting = false;
																		} else {
																			toast.error('Part Failed to Add to ' + p.proj_name, {
																				description:
																					'LCSC ' +
																					importer +
																					' Failed to add, check to make sure part has EASYEDA model'
																			});
																			isImporting = false;
																		}
																	}
																);
															}}><Plus />Import</Button
														></Tooltip.Trigger
													>
													<Tooltip.Content>
														Import the item referenced by the preceding C# into this project
													</Tooltip.Content>
												</Tooltip.Root>
											</Tooltip.Provider>
										{/if}
									</div>
								</Table.Cell>
								<Table.Cell class="text-right">
									{#if removingProjectId == p.id}
										<Button disabled variant="destructive">
											<LoaderCircle class="animate-spin" />
											Deleting
										</Button>
									{:else}
										<Tooltip.Provider>
											<Tooltip.Root>
												<Tooltip.Trigger>
													<Button
														variant="destructive"
														onclick={() => {
															removingProjectId = p.id;
															invoke('delete_project_invoke', { id: p.id }).then((v) => {
																window.location.reload();
																setTimeout(() => {
																	removingProjectId = '';
																}, 5000);
															});
														}}><Trash />Delete</Button
													></Tooltip.Trigger
												>
												<Tooltip.Content>
													Remove project from this project list, (this does not delete project
													files!)
												</Tooltip.Content>
											</Tooltip.Root>
										</Tooltip.Provider>
									{/if}
								</Table.Cell>
							</Table.Row>
						{/each}
					{/await}
				</Table.Body>
			</Table.Root>
		</Card.Content>
		<Card.Footer>
			{#if isAddingProject}
				<Button disabled>
					<LoaderCircle class="animate-spin" />
					Follow Prompts
				</Button>
			{:else}
				<Button
					onclick={async () => {
						invoke('add_project_invoke').then((v) => {
							isAddingProject = false;
							window.location.reload();
						});
						isAddingProject = true;
					}}><Plus />Add Project</Button
				>
			{/if}
		</Card.Footer>
	</Card.Root>
</div>

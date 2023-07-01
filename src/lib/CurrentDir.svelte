<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import SubDir from './SubDir.svelte';

	let folder_name = '';
	let contents: Array<Filedata> = [];

	type Filedata = {
		name: string;
		path: string;
		filetype: string;
	};

	type FolderData = {
		name: string;
		files: Array<Filedata>;
	};
	const getCurrentFolder = () => {
		invoke('get_current_folder', {}).then((data: FolderData) => {
			console.log('Getting current folder', data);
			folder_name = data.name;
			contents = data.files;
		});
	};

	const changeDirectory = (path: string) => {
		contents = [];
		invoke('move_to_folder', { folderPath: path }).then(() => {
			getCurrentFolder();
		});
	};

	const changeDirectoryToParent = () => {
		contents = [];
		invoke('move_to_parent_folder').then(() => {
			getCurrentFolder();
		});
	};

	getCurrentFolder();
</script>

<div>
	<h1>{folder_name}</h1>
	<button on:click={changeDirectoryToParent}>Go Back</button>
	<ul>
		{#each contents as content}
			<li><SubDir folderdata={content} onClick={changeDirectory} /></li>
		{/each}
	</ul>
</div>

<style>
	li {
		margin-bottom: 0.5rem;
		border-bottom: 2px solid gray;
	}
</style>

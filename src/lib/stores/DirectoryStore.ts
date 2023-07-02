import { writable } from 'svelte/store';
import type { Filedata } from '$lib/dirFunctions';

const directoryStore = writable({
    dirName: '',
    forward: [],
    siblings: [
        {
            name: '',
            path: '',
            filetype: ''
        }
    ]
});

export default directoryStore;

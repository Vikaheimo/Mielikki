import { writable, type Writable } from 'svelte/store';
import type { Filedata } from '$lib/dirFunctions';

type DirectoryStore = Writable<{
    dirName: string;
    forward: string[];
    siblings: Filedata[];
}>;
const directoryStore: DirectoryStore = writable({
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

export const addForward = (new_path: string) => {
    directoryStore.update((data) => {
        return {
            dirName: data.dirName,
            siblings: data.siblings,
            forward: [...data.forward, new_path]
        };
    });
};

export const popForward = (): string => {
    let last = '';
    directoryStore.update((data) => {
        last = data.forward[data.forward.length - 1];
        return {
            dirName: data.dirName,
            siblings: data.siblings,
            forward: [...data.forward.slice(0, data.forward.length - 1)]
        };
    });
    return last;
};

export default directoryStore;

import { writable, type Writable } from 'svelte/store';
import type { Filedata } from '$lib/DirFunctions';

type DirectoryStore = Writable<{
    dirName: string;
    forward: string[];
    siblings: Filedata[];
    isAtRoot: boolean;
}>;
const directoryStore: DirectoryStore = writable({
    dirName: '',
    forward: [],
    isAtRoot: false,
    siblings: [
        {
            name: '',
            path: '',
            filetype: ''
        }
    ]
});

export const pushForward = (new_path: string) => {
    directoryStore.update((current) => {
        return {
            dirName: current.dirName,
            siblings: current.siblings,
            isAtRoot: current.isAtRoot,
            forward: [...current.forward, new_path]
        };
    });
};

export const popForward = (): string => {
    let last = '';
    directoryStore.update((current) => {
        last = current.forward[current.forward.length - 1];
        return {
            dirName: current.dirName,
            siblings: current.siblings,
            isAtRoot: current.isAtRoot,
            forward: [...current.forward.slice(0, current.forward.length - 1)]
        };
    });
    return last;
};

export const clearForward = () => {
    directoryStore.update((current) => {
        return {
            dirName: current.dirName,
            siblings: current.siblings,
            isAtRoot: current.isAtRoot,
            forward: []
        };
    });
};

export const clearSiblings = () => {
    directoryStore.update((current) => {
        return {
            dirName: current.dirName,
            forward: current.forward,
            isAtRoot: current.isAtRoot,
            siblings: []
        };
    });
};

export default directoryStore;

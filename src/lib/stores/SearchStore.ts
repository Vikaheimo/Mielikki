import type { Filedata, SearchData } from '$lib/DirFunctions';
import { writable, type Writable } from 'svelte/store';

export type SearchStore = Writable<{
    search: SearchData;
    data: Filedata[];
}>;

const searchStore: SearchStore = writable({
    search: {
        name: '',
        files: true,
        folders: true,
        links: true
    },
    data: []
});

export const clearData = () => {
    searchStore.update((oldData) => {
        return {
            search: oldData.search,
            data: []
        };
    });
};

export const addData = (newData: Filedata[]) => {
    // clearData()
    searchStore.update((oldData) => {
        return {
            data: newData,
            search: oldData.search
        };
    });
};

export default searchStore;

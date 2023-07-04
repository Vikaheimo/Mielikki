import type { Filedata } from '$lib/dirFunctions';
import { writable, type Writable } from 'svelte/store';

export type SearchStore = Writable<{
    data: Filedata[];
}>;

const searchStore: SearchStore = writable({
    data: []
});

export const clearData = () => {
    searchStore.set({
        data: []
    });
};

export const addData = (newData: Filedata[]) => {
    // clearData()
    searchStore.set({
        data: newData
    });
};

export default searchStore;

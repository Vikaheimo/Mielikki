import { invoke } from '@tauri-apps/api/tauri';
import directoryStore, { popForward, pushForward, clearSiblings } from './stores/DirectoryStore';
import { addData } from './stores/SearchStore';

export type Filedata = {
    name: string;
    path: string;
    filetype: string;
};

export type FolderData = {
    name: string;
    files: Filedata[];
    is_at_root: boolean;
};

export type SearchData = {
    name: string;
    files: boolean;
    folders: boolean;
    links: boolean;
};

export const updateCurrentDir = (): void => {
    clearSiblings();

    invoke('get_current_folder', {})
        .then((data: FolderData) => {
            directoryStore.update((current) => {
                return {
                    dirName: data.name,
                    forward: current.forward,
                    isAtRoot: data.is_at_root,
                    siblings: data.files
                };
            });
        })
        .catch(() => {
            changeToParentDirectory();
        });
};

export const changeDirectory = (path: string, toParent = false) => {
    invoke('move_to_folder', { folderPath: path, toParent })
        .then(() => {
            updateCurrentDir();
        })
        .catch((err) => console.error(err));
};

export const changeToParentDirectory = () => {
    invoke('move_to_parent_folder')
        .then((path: string) => {
            updateCurrentDir();
            pushForward(path);
        })
        .catch((err) => console.error(err));
};

export const searchFiles = (data: SearchData) => {
    invoke('find_file', data).then((results: Filedata[]) => {
        addData(results);
    });
};

export const moveForwardDir = () => {
    const newDir = popForward();
    changeDirectory(newDir);
};

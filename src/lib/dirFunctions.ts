import { invoke } from '@tauri-apps/api/tauri';
import directoryStore, { popForward, pushForward } from './stores/DirectoryStore';

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
    filename: string;
    files: boolean;
    folders: boolean;
    links: boolean;
};

export const updateCurrentDir = (): void => {
    // clear siblings
    directoryStore.update((current) => {
        return {
            dirName: current.dirName,
            forward: current.forward,
            isAtRoot: current.isAtRoot,
            siblings: []
        };
    });

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
        .catch((err) => console.error(err));
};

export const changeDirectory = (path: string) => {
    invoke('move_to_folder', { folderPath: path })
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

export const moveForwardDir = () => {
    const newDir = popForward();
    changeDirectory(newDir);
};

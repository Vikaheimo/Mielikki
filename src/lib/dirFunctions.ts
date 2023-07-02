import { invoke } from '@tauri-apps/api/tauri';
import directoryStore, { popForward, addForward } from './stores/DirectoryStore';

export type Filedata = {
    name: string;
    path: string;
    filetype: string;
};

export type FolderData = {
    name: string;
    files: Filedata[];
};

export const updateCurrentDir = (): void => {
    // clear siblings
    directoryStore.update((current) => {
        return {
            dirName: current.dirName,
            forward: current.forward,
            siblings: []
        };
    });

    invoke('get_current_folder', {}).then((data: FolderData) => {
        directoryStore.update((current) => {
            return {
                dirName: data.name,
                forward: current.forward,
                siblings: data.files
            };
        });
    });
};

export const changeDirectory = (path: string) => {
    invoke('move_to_folder', { folderPath: path }).then(() => {
        updateCurrentDir();
    });
};

export const changeToParentDirectory = () => {
    invoke('move_to_parent_folder').then((path: string) => {
        updateCurrentDir();
        addForward(path);
    });
};

/** Doesnt work currently*/
export const currentDirIsRoot = (): boolean => {
    let isRoot = false;
    invoke('current_dir_is_root').then((value: boolean) => {
        isRoot = value;
    });
    return isRoot;
};

export const moveForwardDir = () => {
    const newDir = popForward();
    changeDirectory(newDir);
};

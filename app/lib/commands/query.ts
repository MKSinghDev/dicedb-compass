import { invoke } from '@tauri-apps/api/core';

export const getKeys = async (connection: string) => {
    return await invoke<string[]>('get_keys', { connection });
};

export const getKey = async (connection: string, key: string) => {
    return await invoke<string>('get_key', { connection, key });
};

export const searchKey = async (connection: string, key: string) => {
    return await invoke<string[]>('search_key', { connection, key });
};

export const addKey = async (connection: string, name: string, value: string) => {
    return await invoke<boolean>('add_key', { connection, name, value });
};

export const deleteKey = async (connection: string, key: string) => {
    return await invoke('remove_key', { connection, key });
};

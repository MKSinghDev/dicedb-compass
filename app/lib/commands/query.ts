import { invoke } from '@tauri-apps/api/core';

export const getKeys = async (connection: string) => {
    return await invoke<string[]>('get_keys', { connection });
};

export const getKey = async (connection: string, key: string) => {
    return await invoke<string>('get_key', { connection, key });
};

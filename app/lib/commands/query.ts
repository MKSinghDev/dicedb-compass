import { invoke } from '@tauri-apps/api/core';

export const getKeys = async (connection: string) => {
    return await invoke<string[]>('get_keys', { connection });
};

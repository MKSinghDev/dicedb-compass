import { invoke } from '@tauri-apps/api/core';

export const hasStoredPassword = async () => {
    try {
        const hasPassword: boolean = await invoke('has_stored_password');
        return hasPassword;
    } catch (error) {
        console.error('Failed to check if password is stored:', error);
        return false;
    }
};

export const isStrongholdReady = async () => {
    try {
        const res = await invoke('is_stronghold_ready');
        return res;
    } catch (error) {
        console.error('Failed to check if stronghold is ready:', error);
        return false;
    }
};

export const getKeys = async () => {
    try {
        const keys: string[] = await invoke('get_keys');
        return keys;
    } catch (error) {
        console.error('Failed to get keys:', error);
        return [];
    }
};

export const saveSecret = async (key: string, value: string) => {
    try {
        await invoke('save_secret', { key, value });
        return true;
    } catch (error) {
        console.log('Failed to save secret:', error);
        return false;
    }
};

export const getSecret = async (key: string) => {
    try {
        const value = await invoke('get_secret', { key });
        return value;
    } catch (error) {
        console.error('Failed to get secret:', error);
        return null;
    }
};

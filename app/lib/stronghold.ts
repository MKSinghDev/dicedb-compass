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
        const res = await invoke('save_secret', { key, value });
        return res;
    } catch (error) {
        console.error('Failed to save secret:', error);
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

class StrongholdManager {
    initialized = false;

    constructor() {
        this.initialized = false;
    }

    async initialize(password: string) {
        try {
            await invoke('init_stronghold', { password });
            this.initialized = true;
            console.log('Stronghold initialized successfully');
            return true;
        } catch (error) {
            console.error('Failed to initialize Stronghold:', error);
            this.initialized = false;
            return false;
        }
    }

    async saveSecret(key: string, value: string) {
        try {
            await invoke('save_secret', { key, value });
            console.log(`Secret '${key}' saved successfully`);
            return true;
        } catch (error) {
            console.error(`Failed to save secret '${key}':`, error);
            return false;
        }
    }

    async getSecret(key: string) {
        try {
            const value = await invoke('get_secret', { key });
            console.log(`Secret '${key}' retrieved successfully`);
            return value;
        } catch (error) {
            console.error(`Failed to get secret '${key}':`, error);
            return null;
        }
    }

    async removeSecret(key: string) {
        try {
            await invoke('remove_secret', { key });
            console.log(`Secret '${key}' removed successfully`);
            return true;
        } catch (error) {
            console.error(`Failed to remove secret '${key}':`, error);
            return false;
        }
    }

    async isInitialized() {
        try {
            const initialized: boolean = await invoke('is_stronghold_initialized');
            this.initialized = initialized;
            return initialized;
        } catch (error) {
            console.error('Failed to check Stronghold status:', error);
            return false;
        }
    }

    async lock() {
        try {
            await invoke('lock_stronghold');
            this.initialized = false;
            console.log('Stronghold locked successfully');
            return true;
        } catch (error) {
            console.error('Failed to lock Stronghold:', error);
            return false;
        }
    }
}

export const stronghold = new StrongholdManager();

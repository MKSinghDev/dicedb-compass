import { appDataDir } from '@tauri-apps/api/path';
import { Client, Store, Stronghold } from '@tauri-apps/plugin-stronghold';

export const initStronghold = async () => {
    const vaultPath = `${await appDataDir()}/vault.hold`;
    const vaultPassword = 'vault password';
    const stronghold = await Stronghold.load(vaultPath, vaultPassword);

    let client: Client;
    const clientName = 'name your client';
    try {
        client = await stronghold.loadClient(clientName);
    } catch {
        client = await stronghold.createClient(clientName);
    }

    return {
        stronghold,
        client,
    };
};

export const insertRecord = async (store: Store, key: string, value: string) => {
    const data = Array.from(new TextEncoder().encode(value));
    await store.insert(key, data);
};

export const getRecord = async (store: Store, key: string): Promise<string> => {
    const data = await store.get(key);
    if (!data) return '';

    return new TextDecoder().decode(new Uint8Array(data));
};

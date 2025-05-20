import { appDataDir } from '@tauri-apps/api/path';
import { Store, Stronghold } from '@tauri-apps/plugin-stronghold';

interface StrongholdType {
    clientNmae: 'dicedb-compass';
}

const VAULT_PASSWORD = 'vault password';

export const loadStronghold = async ({ clientNmae }: StrongholdType) => {
    const vaultPath = `${await appDataDir()}/vault.hold`;
    const stronghold = await Stronghold.load(vaultPath, VAULT_PASSWORD);

    try {
        const client = await stronghold.loadClient(clientNmae);
        return { stronghold, client };
    } catch {
        return null;
    }
};

export const initStronghold = async ({ clientNmae }: StrongholdType) => {
    const vaultPath = `${await appDataDir()}/vault.hold`;
    const stronghold = await Stronghold.load(vaultPath, VAULT_PASSWORD);

    const client = await stronghold.createClient(clientNmae);
    return { stronghold, client };
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

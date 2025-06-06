import { invoke } from '@tauri-apps/api/core';
import type { ConnectionConfig } from '~/lib/stores/connections';

export const saveConnection = async (config: ConnectionConfig) => {
    return invoke<boolean>('save_connections', { config });
};

export const getConnections = async () => {
    return invoke<ConnectionConfig[] | null>('get_connections');
};

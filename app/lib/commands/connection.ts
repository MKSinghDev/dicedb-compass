import { invoke } from '@tauri-apps/api/core';
import type { ConnectionConfig } from '~/lib/stores/connections';

export const connect = async (config: ConnectionConfig) => {
    return invoke<boolean>('connect', { config });
};

export const saveConnection = async (config: ConnectionConfig) => {
    return invoke<boolean>('save_connection', { config });
};

export const getConnections = async () => {
    return invoke<ConnectionConfig[] | null>('get_connections');
};

export const saveAndConnect = async (config: ConnectionConfig) => {
    return invoke<boolean>('save_connection', { config });
};

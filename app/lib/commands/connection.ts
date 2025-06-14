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

export const addConnection = async (config: ConnectionConfig) => {
    return invoke<boolean>('add_connection', { config });
};

export const getActiveConnections = async () => {
    return invoke<string[]>('get_active_connections');
};

export const removeConnection = (connName: string) => {
    return invoke<boolean>('remove_connection', { connName });
};

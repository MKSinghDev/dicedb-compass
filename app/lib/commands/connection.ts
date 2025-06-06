import { invoke } from '@tauri-apps/api/core';
import type { ConnectionConfig } from '~/lib/stores/connections';

export const getConnections = async () => {
    return invoke<ConnectionConfig[] | null>('get_connections');
};

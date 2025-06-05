import { invoke } from '@tauri-apps/api/core';

export const getConnectionsName = async () => {
    return invoke('get_connections_name');
};

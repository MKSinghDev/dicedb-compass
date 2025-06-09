import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { immer } from 'zustand/middleware/immer';

import { getConnections } from '../commands/connection';

export interface ConnectionConfig {
    name: string;
    conn_string: string;
    history_depth: number;
}

interface ConnectionsState {
    connections: Array<ConnectionConfig>;
}

interface ConnectionsAction {
    setConnections: (connections: ConnectionConfig[]) => void;
    refreshConnections: () => Promise<void>;
}

interface ConnectionsStore extends ConnectionsState, ConnectionsAction {}

const useConnectionsStore = create<ConnectionsStore>()(
    persist(
        immer(set => ({
            connections: [],
            activeConnections: [],
            setConnections: connections => {
                set({ connections });
            },
            refreshConnections: async () => {
                const connections = await getConnections();
                set({ connections: connections || [] });
            },
        })),
        { name: 'connections' }
    )
);

export const useGetConnections = () => useConnectionsStore(state => state.connections);
export const useSetConnections = () => useConnectionsStore(state => state.setConnections);
export const useRefreshConnections = () => useConnectionsStore(state => state.refreshConnections);

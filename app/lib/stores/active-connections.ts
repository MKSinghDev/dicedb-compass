import { create } from 'zustand';
import { immer } from 'zustand/middleware/immer';

import { getActiveConnections } from '../commands/connection';

interface ConnectionsState {
    activeConnections: Array<string>;
}

interface ConnectionsAction {
    setConnections: (activeConnections: string[]) => void;
    refreshConnections: () => Promise<void>;
}

interface ConnectionsStore extends ConnectionsState, ConnectionsAction {}

const useActiveConnectionsStore = create<ConnectionsStore>()(
    immer(set => ({
        connections: [],
        activeConnections: [],
        setConnections: activeConnections => {
            set({ activeConnections });
        },
        refreshConnections: async () => {
            const activeConnections = await getActiveConnections();
            set({ activeConnections });
        },
    }))
);

export const useGetActiveConnections = () => useActiveConnectionsStore(state => state.activeConnections);
export const useSetActiveConnections = () => useActiveConnectionsStore(state => state.setConnections);
export const useRefreshActiveConnections = () => useActiveConnectionsStore(state => state.refreshConnections);

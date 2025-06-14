import { data, redirect } from 'react-router';
import type { Route } from '+/_layout+/+types';

import { Welcome } from '~/components/pages/home/welcome';
import { getActiveConnections, getConnections } from '~/lib/commands/connection';
import { useSetConnections } from '~/lib/stores/connections';

export function meta() {
    return [{ title: 'DiceDB Compass | Developed by MKSingh' }, { name: 'description', content: 'DiceDB Compass | Developed by MKSingh' }];
}

export const clientLoader = async () => {
    const [connections, activeConnections] = await Promise.all([getConnections(), getActiveConnections()]);

    if (connections?.length) {
        return data({ connections, activeConnections });
    }
    throw redirect('/connection/add');
};

export default function Home({ loaderData }: Route.ComponentProps) {
    const setConnections = useSetConnections();
    setConnections(loaderData.connections);

    return <Welcome />;
}

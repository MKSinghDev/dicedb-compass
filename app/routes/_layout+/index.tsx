import { data, redirect } from 'react-router';
import type { Route } from '+/_layout+/+types';

import { Welcome } from '~/components/pages/home/welcome';
import { getConnections } from '~/lib/commands/connection';
import { useSetConnections } from '~/lib/stores/connections';

export function meta() {
    return [{ title: 'DiceDB compass | Developed by MKSingh' }, { name: 'description', content: 'DiceDB compass | Developed by MKSingh' }];
}

export const clientLoader = async () => {
    const connections = await getConnections();
    if (connections.length) {
        return data(connections);
    }
    redirect('/add-connection');
};

export default function Home({ loaderData }: Route.ComponentProps) {
    const setConnections = useSetConnections();
    setConnections(loaderData || []);

    return <Welcome />;
}

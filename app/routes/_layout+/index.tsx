import { data } from 'react-router';
import type { Route } from '+/_layout+/+types';

import { parseWithZod } from '@conform-to/zod/v4';
import { invoke } from '@tauri-apps/api/core';
import type { Action } from '~/components/pages/home/interface';
import { Welcome } from '~/components/pages/home/welcome';
import { getConnectionsName } from '~/lib/commands/connection';
import { schema } from '~/lib/schema/connection';
import { Message } from '~/lib/utils/message-handler';

export function meta() {
    return [{ title: 'DiceDB compass | Developed by MKSingh' }, { name: 'description', content: 'DiceDB compass | Developed by MKSingh' }];
}

export const clientLoader = async () => {
    const connections = await getConnectionsName();
    console.log('PASS STORED', { connections });
    return data(connections);
};

export const clientAction = async ({ request }: Route.ClientActionArgs) => {
    const formData = await request.formData();
    const action = formData.get('action') as Action | null;
    console.log('ACTION', action);
    const parsedData = parseWithZod(formData, { schema });
    if (parsedData.status !== 'success') return parsedData.reply();

    console.log('PARSED DATA', parsedData.value);
    const { host, port } = parsedData.value;
    const key = `${host}-${port}`;
    switch (action) {
        case 'save': {
            try {
                console.log('SAVE', key);
                const { name, host, port } = parsedData.value;
                const res = invoke('db_test', { config: { name, conn_string: `${host}:${port}`, history_depth: 10 } });
                console.log('KEYS', { res });
                return data(Message.success('Secret saved successfully'));
            } catch (error) {
                console.error('Failed to save secret:', error);
                return data(Message.error('Something went wrong while saving secret'));
            }
        }
        case 'connect': {
            try {
                console.log('CONNECT', key);
                const { name, host, port } = parsedData.value;
                const res = invoke('db_test', { config: { name, conn_string: `${host}:${port}`, history_depth: 10 } });
                console.log('Connection Addition Response:', res);
            } catch (err) {
                console.log('This is error', err);
            }
            break;
        }
        default: {
            return data(Message.warning('Invalid action'));
        }
    }
};

export default function Home() {
    return <Welcome />;
}

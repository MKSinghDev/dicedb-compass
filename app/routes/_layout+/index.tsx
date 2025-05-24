import { data } from 'react-router';
import type { Route } from '+/_layout+/+types';

import { parseWithZod } from '@conform-to/zod/v4';
import type { Action } from '~/components/pages/home/interface';
import { Welcome } from '~/components/pages/home/welcome';
import { schema } from '~/lib/schema/connection';
import { getKeys, getSecret, hasStoredPassword, isStrongholdReady, saveSecret } from '~/lib/stronghold';
import { Message } from '~/lib/utils/message-handler';

export function meta() {
    return [{ title: 'DiceDB compass | Developed by MKSingh' }, { name: 'description', content: 'DiceDB compass | Developed by MKSingh' }];
}

export const clientLoader = async () => {
    const passStored = await hasStoredPassword();
    const isStrReady = await isStrongholdReady();

    const keys = await getKeys();
    console.log('PASS STORED', { passStored, isStrReady, keys });
    return keys;
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
                const res = await saveSecret(key, JSON.stringify(parsedData.value));
                const keys = await getKeys();
                const secret = await getSecret(key);
                console.log('KEYS', { res, keys, secret });
                return data(res ? Message.success('Secret saved successfully') : Message.error('Failed to save secret'));
            } catch (error) {
                console.error('Failed to save secret:', error);
                return data(Message.error('Something went wrong while saving secret'));
            }
        }
        default: {
            return data(Message.warning('Invalid action'));
        }
    }
};

export default function Home() {
    return <Welcome />;
}

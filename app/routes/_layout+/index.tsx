import type { Route } from '+/_layout+/+types';

import { parseWithZod } from '@conform-to/zod/v4';
import type { Action } from '~/components/pages/home/interface';
import { Welcome } from '~/components/pages/home/welcome';
import { schema } from '~/lib/schema/connection';
import { getRecord, initStronghold, insertRecord, loadStronghold } from '~/lib/stronghold';

export function meta() {
    return [{ title: 'DiceDB compass | Developed by MKSingh' }, { name: 'description', content: 'DiceDB compass | Developed by MKSingh' }];
}

export const clientLoader = async () => {
    return await loadStronghold({ clientNmae: 'dicedb-compass' });
};

export const clientAction = async ({ request }: Route.ClientActionArgs) => {
    const formData = await request.formData();
    const action = formData.get('action') as Action | null;
    const parsedData = parseWithZod(formData, { schema });
    if (parsedData.status !== 'success') return parsedData.reply();

    const { client } = await initStronghold({ clientNmae: 'dicedb-compass' });
    const store = client.getStore();

    console.log('STORE', store.path, store.client);
    switch (action) {
        case 'save': {
            const key = `${parsedData.value.host}-${parsedData.value.port}`;
            const record = await getRecord(store, key);
            const insertion = await insertRecord(store, key, JSON.stringify(parsedData.value));

            console.log('SAVE', { record, insertion });
            console.log(Object.fromEntries(formData.entries()), parsedData, { action });
            break;
        }
        case 'save-and-connect':
            console.log('SAVE-and-CONNECT');
            console.log(Object.fromEntries(formData.entries()), parsedData, { action });
            break;
        case 'connect':
            console.log('CONNECT');
            console.log(Object.fromEntries(formData.entries()), parsedData, { action });
            break;
        default:
            console.log('DEFAULT');
            return parsedData.reply();
    }
};

export default function Home() {
    return <Welcome />;
}

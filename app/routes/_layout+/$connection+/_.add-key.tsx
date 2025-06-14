import { data, redirect } from 'react-router';
import type { Route } from '+/_layout+/$connection+/+types/_.add-key';

import { parseWithZod } from '@conform-to/zod/v4';
import AddKeyPage from '~/components/pages/connection/add-key';
import { schema } from '~/components/pages/connection/add-key-schema';
import { addKey } from '~/lib/commands/query';

export const clientAction = async ({ request, params: { connection } }: Route.ClientActionArgs) => {
    const formData = await request.formData();
    const result = parseWithZod(formData, { schema });
    if (request.method === 'POST' && result.status === 'success') {
        const { name, value } = result.value;
        const res = await addKey(connection, name, value ?? '');
        if (res) {
            throw redirect(`/${connection}`);
        }
    }
    return data(null);
};

export default AddKeyPage;

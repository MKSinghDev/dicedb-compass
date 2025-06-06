import { data } from 'react-router';

import { parseWithZod } from '@conform-to/zod/v4';
import { invoke } from '@tauri-apps/api/core';
import ConnectionDialog from '~/components/molecules/connection-dialog';
import type { Action } from '~/components/pages/home/interface';
import { schema } from '~/lib/schema/connection';
import { useRefreshConnections } from '~/lib/stores/connections';
import { Message } from '~/lib/utils/message-handler';

import type { Route } from './+types/add-connection';

export const clientAction = async ({ request }: Route.ActionArgs) => {
    const formData = await request.formData();
    const action = formData.get('action') as Action | null;
    const parsedData = parseWithZod(formData, { schema });
    if (parsedData.status !== 'success') return parsedData.reply();

    switch (action) {
        case 'save': {
            try {
                const { name, host, port } = parsedData.value;
                invoke('db_test', { config: { name, conn_string: `${host}:${port}`, history_depth: 10 } });

                return data(Message.success('Secret saved successfully'));
            } catch {
                return data(Message.error('Something went wrong while saving secret'));
            }
        }
        case 'connect': {
            try {
                const { name, host, port } = parsedData.value;
                invoke('db_test', { config: { name, conn_string: `${host}:${port}`, history_depth: 10 } });
            } catch {
                //
            }
            break;
        }
        default: {
            return data(Message.warning('Invalid action'));
        }
    }
};

const AddConnectionPage = ({ actionData }: Route.ComponentProps) => {
    const refreshConnections = useRefreshConnections();
    if (actionData) {
        refreshConnections();
    }
    return <ConnectionDialog defaultOpen />;
};

export default AddConnectionPage;

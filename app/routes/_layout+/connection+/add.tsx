import { data } from 'react-router';
import type { Route } from '+/_layout+/connection+/+types/add';
import { Fragment } from 'react/jsx-runtime';

import { parseWithZod } from '@conform-to/zod/v4';
import ConnectionDialog from '~/components/molecules/connection-dialog';
import type { Action } from '~/components/pages/home/interface';
import { connect, saveAndConnect, saveConnection } from '~/lib/commands/connection';
import { schema } from '~/lib/schema/connection';
import { useRefreshConnections } from '~/lib/stores/connections';
import { Message } from '~/lib/utils/message-handler';
import { ActionMessageToaster } from '~/lib/utils/message-handler/toaster';

export const clientAction = async ({ request }: Route.ClientActionArgs) => {
    const formData = await request.formData();
    const action = formData.get('action') as Action | null;
    const parsedData = parseWithZod(formData, { schema });
    if (parsedData.status !== 'success') return data(null);

    switch (action) {
        case 'save': {
            try {
                const { name, host, port } = parsedData.value;
                const res = await saveConnection({ name, conn_string: `${host}:${port}`, history_depth: 10 });
                return data(
                    res ? Message.success('Config saved successfully', { action }) : Message.error('Failed to save connection details', { action })
                );
            } catch {
                return data(Message.error('Something went wrong while saving secret', { action }));
            }
        }

        case 'connect': {
            try {
                const { name, host, port } = parsedData.value;
                const res = await connect({ name, conn_string: `${host}:${port}`, history_depth: 10 });
                return data(res ? Message.success('Connected successfully', { action }) : Message.error('Failed to connect', { action }));
            } catch {
                return data(Message.error('Something went wrong while connecting', { action }));
            }
        }

        case 'save-and-connect': {
            try {
                const { name, host, port } = parsedData.value;
                const res = await saveAndConnect({ name, conn_string: `${host}:${port}`, history_depth: 10 });
                return data(
                    res ? Message.success('Saved and connected successfully', { action }) : Message.error('Failed to save and connect', { action })
                );
            } catch {
                return data(Message.error('Something went wrong while saving and connecting', { action }));
            }
        }
    }
};

const AddConnectionPage = ({ actionData }: Route.ComponentProps) => {
    const refreshConnections = useRefreshConnections();
    if (actionData) {
        refreshConnections();
    }
    return (
        <Fragment>
            <ConnectionDialog defaultOpen />
            <ActionMessageToaster />
        </Fragment>
    );
};

export default AddConnectionPage;

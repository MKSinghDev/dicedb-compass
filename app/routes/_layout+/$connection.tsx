import { data, useParams } from 'react-router';

import Typography from '~/components/atoms/typography';
import { getKeys } from '~/lib/commands/query';

import type { Route } from './+types/$connection';

export const clientLoader = async ({ params: { connection } }: Route.ClientLoaderArgs) => {
    return data(await getKeys(connection));
};

const ConnectionPage = ({ loaderData }: Route.ComponentProps) => {
    const params = useParams<Route.ComponentProps['params']>();
    return (
        <div className="flex flex-col gap-4">
            <Typography>{params.connection}</Typography>
            {loaderData?.map(key => (
                <div className="flex gap-3" key={key}>
                    <Typography>{key}</Typography>
                </div>
            ))}
        </div>
    );
};

export default ConnectionPage;

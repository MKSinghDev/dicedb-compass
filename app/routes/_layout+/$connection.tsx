import { data, useParams } from 'react-router';

import Typography from '~/components/atoms/typography';
import SearchBar from '~/components/molecules/search-bar';
import { getKey, getKeys } from '~/lib/commands/query';

import type { Route } from './+types/$connection';

export const clientLoader = async ({ request, params: { connection } }: Route.ClientLoaderArgs) => {
    const { searchParams } = new URL(request.url);
    const key = searchParams.get('key');
    console.log('Search params key:', key);
    if (key) {
        const value = await getKey(connection!, key);
        return data(value ? [value] : []);
    }
    return data(await getKeys(connection!));
};

const ConnectionPage = ({ loaderData }: Route.ComponentProps) => {
    const params = useParams<Route.ComponentProps['params']>();
    console.log({ loaderData });
    return (
        <div className="flex flex-col gap-2 h-full">
            <SearchBar />
            <Typography>{params.connection}</Typography>
            {loaderData.length ? (
                loaderData.map(key => (
                    <div className="flex gap-3" key={key}>
                        <Typography>{key}</Typography>
                    </div>
                ))
            ) : (
                <div className="flex flex-col m-auto flex-1 items-center justify-center gap-3">
                    <Typography variant="muted">No result found!</Typography>
                </div>
            )}
        </div>
    );
};

export default ConnectionPage;

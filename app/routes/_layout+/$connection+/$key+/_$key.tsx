import { data } from 'react-router';
import type { Route } from '+/_layout+/$connection+/$key+/+types/_$key';

import { typographyVariants } from '~/components/atoms/typography';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '~/components/ui/card';
import { getKey } from '~/lib/commands/query';

export const clientLoader = async ({ params: { connection, key } }: Route.ClientLoaderArgs) => {
    return data(await getKey(connection, key));
};

const ConnectionKeyPage = ({ loaderData, params: { key } }: Route.ComponentProps) => {
    return (
        <Card className="w-full">
            <CardHeader className="sr-only">
                <CardTitle>Value Card</CardTitle>
                <CardDescription>Value of {decodeURIComponent(key)}</CardDescription>
            </CardHeader>
            <CardContent>
                <span className={typographyVariants({ variant: 'h3' })}>{loaderData}</span>
            </CardContent>
        </Card>
    );
};

export default ConnectionKeyPage;

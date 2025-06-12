import { data, Link, Outlet, useParams } from 'react-router';
import type { Route } from '+/_layout+/$connection+/+types/_$connection';
import { TriangleAlert } from 'lucide-react';

import Typography from '~/components/atoms/typography';
import SearchBar from '~/components/molecules/search-bar';
import { ScrollArea } from '~/components/ui/scroll-area';
import { Table, TableBody, TableCell, TableRow } from '~/components/ui/table';
import { getKey, getKeys } from '~/lib/commands/query';

export const clientLoader = async ({ request, params: { connection } }: Route.ClientLoaderArgs) => {
    const { searchParams } = new URL(request.url);
    const key = searchParams.get('key');
    if (key) {
        const value = await getKey(connection!, key);
        return data(value ? [value] : []);
    }
    return data(await getKeys(connection!));
};

const ConnectionPage = ({ loaderData }: Route.ComponentProps) => {
    const params = useParams<Route.ComponentProps['params']>();
    return (
        <div className="flex flex-col gap-2 h-full">
            <SearchBar />
            <Typography>{params.connection}</Typography>
            <ScrollArea className="h-full">
                <div className="flex gap-2 h-full">
                    {loaderData.length ? (
                        <div className="flex w-full h-fit gap-2">
                            <div className="flex flex-col gap-2 w-full">
                                <div className="bg-background overflow-hidden rounded-md border">
                                    <Table>
                                        <TableBody>
                                            {loaderData.map(key => (
                                                <TableRow className="*:border-border hover:bg-transparent [&>:not(:last-child)]:border-r" key={key}>
                                                    <TableCell className="bg-muted/50 py-2 font-medium">
                                                        <Link to={encodeURIComponent(key)}>{key}</Link>
                                                    </TableCell>
                                                </TableRow>
                                            ))}
                                        </TableBody>
                                    </Table>
                                </div>
                            </div>
                            <Outlet />
                        </div>
                    ) : (
                        <div className="flex flex-col m-auto items-center justify-center gap-3">
                            <TriangleAlert className="text-muted-foreground" />
                            <Typography variant="muted">No result found!</Typography>
                        </div>
                    )}
                </div>
            </ScrollArea>
        </div>
    );
};

export default ConnectionPage;

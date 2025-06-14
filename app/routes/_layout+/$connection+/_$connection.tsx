import { data, Link, NavLink, Outlet, useParams } from 'react-router';
import type { Route } from '+/_layout+/$connection+/+types/_$connection';
import { Plus, Trash2, TriangleAlert } from 'lucide-react';

import Typography from '~/components/atoms/typography';
import SearchBar from '~/components/molecules/search-bar';
import DeleteConnection from '~/components/pages/connection/delete-connection';
import { Button, buttonVariants } from '~/components/ui/button';
import { ScrollArea } from '~/components/ui/scroll-area';
import { Table, TableBody, TableCell, TableRow } from '~/components/ui/table';
import { getKeys, searchKey } from '~/lib/commands/query';
import { cn } from '~/lib/utils';

export const clientLoader = async ({ request, params: { connection } }: Route.ClientLoaderArgs) => {
    const { searchParams } = new URL(request.url);
    const key = searchParams.get('key');
    if (request.method === 'GET' && key) {
        const value = await searchKey(connection!, key);
        return data(value ?? []);
    }
    return data(await getKeys(connection!));
};

const ConnectionPage = ({ loaderData, params: { connection } }: Route.ComponentProps) => {
    const params = useParams<Route.ComponentProps['params']>();
    return (
        <div className="flex flex-col gap-2 h-full">
            <div className="flex items-center w-full gap-2">
                <SearchBar />
                <Link to={`/${connection}/add-key`} className={buttonVariants()}>
                    <Plus />
                    Add
                </Link>
            </div>
            <Typography>{params.connection}</Typography>
            <ScrollArea className="h-full">
                <div className="flex gap-2 h-full">
                    {loaderData.length ? (
                        <div className="flex w-full h-fit">
                            <div className="flex flex-col gap-2 w-full">
                                <div className="bg-background overflow-hidden rounded-r-none border-r-0 rounded-md border">
                                    <Table>
                                        <TableBody>
                                            {loaderData.map(key => (
                                                <TableRow className="*:border-border hover:bg-transparent [&>:not(:last-child)]:border-r" key={key}>
                                                    <TableCell className="flex font-medium w-full bg-muted/50 p-0">
                                                        <NavLink
                                                            to={encodeURIComponent(key)}
                                                            className={({ isActive }) => cn('w-full flex p-3', isActive ? 'bg-background' : '')}
                                                        >
                                                            {key}
                                                            <DeleteConnection keyName={key} connection={connection}>
                                                                <Button
                                                                    className={cn(
                                                                        buttonVariants({ variant: 'destructive', size: 'icon' }),
                                                                        'z-10 ml-auto size-6 text-destructive-foreground cursor-pointer'
                                                                    )}
                                                                >
                                                                    <Trash2 size={12} />
                                                                </Button>
                                                            </DeleteConnection>
                                                        </NavLink>
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

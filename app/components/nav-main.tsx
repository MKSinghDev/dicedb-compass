import { useState } from 'react';
import { NavLink, useNavigate } from 'react-router';
import { DatabaseZap, MoreHorizontal, Trash2 } from 'lucide-react';

import RawLoader from '~/components/atoms/raw-loader';
import {
    AlertDialog,
    AlertDialogAction,
    AlertDialogCancel,
    AlertDialogContent,
    AlertDialogDescription,
    AlertDialogFooter,
    AlertDialogHeader,
    AlertDialogTitle,
} from '~/components/ui/alert-dialog';
import { Button } from '~/components/ui/button';
import { Collapsible } from '~/components/ui/collapsible';
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from '~/components/ui/dropdown-menu';
import { SidebarGroup, SidebarGroupLabel, SidebarMenu, SidebarMenuButton, SidebarMenuItem } from '~/components/ui/sidebar';
import { addConnection, removeConnection } from '~/lib/commands/connection';
import { useGetActiveConnections, useRefreshActiveConnections } from '~/lib/stores/active-connections';
import { type ConnectionConfig, useGetConnections, useRefreshConnections } from '~/lib/stores/connections';
import { cn } from '~/lib/utils';
import { dispatchToast } from '~/lib/utils/message-handler';

export function NavMain() {
    const connections = useGetConnections();
    const refreshActiveConnections = useRefreshActiveConnections();
    const activeConnections = useGetActiveConnections();
    const navigate = useNavigate();
    const [isConnecting, setIsConnecting] = useState<{ status?: 'connecting'; index: number }>();
    const [selectedConnectionName, setSelectedConnectionName] = useState<string | null>(null);

    const handleConnect = async (config: ConnectionConfig, index: number) => {
        try {
            setIsConnecting({ status: 'connecting', index });
            const res = await addConnection(config);
            if (res) {
                dispatchToast({ type: 'success', message: 'Connection successfull' });
                await refreshActiveConnections();
                navigate(`/${config.name}`);
            }
        } catch (error) {
            if (typeof error === 'string' && error.includes('Connection error:')) {
                dispatchToast({ type: 'error', message: { title: error.split(':')[1], description: error.split(':')[2], duration: Infinity } });
            }
        } finally {
            setIsConnecting(undefined);
        }
    };

    return (
        <SidebarGroup className="max-w-64">
            <SidebarGroupLabel>Connections</SidebarGroupLabel>
            <RemoveDialog connectionName={selectedConnectionName} setConnectionName={setSelectedConnectionName} />
            <SidebarMenu>
                {connections?.map((connection, i) => (
                    <Collapsible key={connection.name} asChild defaultOpen={true} className="group/collapsible">
                        <SidebarMenuItem>
                            {activeConnections.includes(connection.name) ? (
                                <NavLink to={`/${connection.name}`}>
                                    {({ isActive }) => (
                                        <SidebarMenuButton tooltip={connection.name} isActive={isActive} className="cursor-pointer" asChild>
                                            <div>
                                                <DatabaseZap />
                                                <span className="whitespace-nowrap truncate mr-auto">{connection.name}</span>
                                                <GreenDot />
                                                <DropdownMenu>
                                                    <DropdownMenuTrigger asChild>
                                                        <Button size="icon" variant="ghost" className="size-6 cursor-pointer">
                                                            <MoreHorizontal />
                                                        </Button>
                                                    </DropdownMenuTrigger>
                                                    <DropdownMenuContent>
                                                        <DropdownMenuItem
                                                            variant="destructive"
                                                            className="text-destructive-foreground"
                                                            onClick={() => setSelectedConnectionName(connection.name)}
                                                        >
                                                            <Trash2 />
                                                            Remove
                                                        </DropdownMenuItem>
                                                    </DropdownMenuContent>
                                                </DropdownMenu>
                                            </div>
                                        </SidebarMenuButton>
                                    )}
                                </NavLink>
                            ) : (
                                <SidebarMenuButton tooltip={connection.name} asChild>
                                    <div className="group/item transition-all duration-200">
                                        {isConnecting?.index === i ? <RawLoader className="size-4" /> : <DatabaseZap />}
                                        <span className={cn('whitespace-nowrap truncate mr-auto')}>{connection.name}</span>
                                        <Button
                                            size="sm"
                                            className={cn('h-5 ml-auto hidden group-hover/item:block', isConnecting?.index === i && 'block')}
                                            onClick={() => handleConnect(connection, i)}
                                            disabled={isConnecting?.status === 'connecting'}
                                        >
                                            {isConnecting?.index === i ? 'Connecting' : 'Connect'}
                                        </Button>
                                        <DropdownMenu>
                                            <DropdownMenuTrigger asChild>
                                                <Button size="icon" variant="ghost" className="size-6 cursor-pointer">
                                                    <MoreHorizontal />
                                                </Button>
                                            </DropdownMenuTrigger>
                                            <DropdownMenuContent>
                                                <DropdownMenuItem
                                                    variant="destructive"
                                                    className="text-destructive-foreground"
                                                    onClick={() => setSelectedConnectionName(connection.name)}
                                                >
                                                    <Trash2 />
                                                    Remove
                                                </DropdownMenuItem>
                                            </DropdownMenuContent>
                                        </DropdownMenu>
                                    </div>
                                </SidebarMenuButton>
                            )}
                        </SidebarMenuItem>
                    </Collapsible>
                ))}
            </SidebarMenu>
        </SidebarGroup>
    );
}

const GreenDot = () => <span className="ml-auto size-2 rounded-full shrink-0 bg-emerald-500 dark:bg-emerald-400" />;
const RemoveDialog = ({ connectionName, setConnectionName }: { connectionName: string | null; setConnectionName: (arg: null) => void }) => {
    const refreshConnections = useRefreshConnections();
    const handleRemove = async () => {
        try {
            if (connectionName) {
                await removeConnection(connectionName);
                refreshConnections();
            }
        } catch (error) {
            dispatchToast({ type: 'error', message: (error as Error).message });
        }
    };

    return (
        <AlertDialog
            open={Boolean(connectionName)}
            onOpenChange={() => {
                setConnectionName(null);
            }}
        >
            <AlertDialogContent>
                <AlertDialogHeader>
                    <AlertDialogTitle>Are you sure?</AlertDialogTitle>
                    <AlertDialogDescription>Take a moment to rethink, this action cannot be undone!</AlertDialogDescription>
                </AlertDialogHeader>
                <AlertDialogFooter>
                    <AlertDialogCancel>Cancel</AlertDialogCancel>
                    <AlertDialogAction asChild>
                        <Button variant="destructive" className="text-destructive-foreground" onClick={handleRemove}>
                            Okay
                        </Button>
                    </AlertDialogAction>
                </AlertDialogFooter>
            </AlertDialogContent>
        </AlertDialog>
    );
};

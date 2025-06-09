import { useState } from 'react';
import { NavLink, useNavigate } from 'react-router';
import { DatabaseZap } from 'lucide-react';

import { Button } from '~/components/ui/button';
import { Collapsible } from '~/components/ui/collapsible';
import { SidebarGroup, SidebarGroupLabel, SidebarMenu, SidebarMenuButton, SidebarMenuItem } from '~/components/ui/sidebar';
import { addConnection } from '~/lib/commands/connection';
import { useGetActiveConnections, useRefreshActiveConnections } from '~/lib/stores/active-connections';
import { type ConnectionConfig, useGetConnections } from '~/lib/stores/connections';
import { cn } from '~/lib/utils';
import { dispatchToast } from '~/lib/utils/message-handler';

import RawLoader from './atoms/raw-loader';

export function NavMain() {
    const connections = useGetConnections();
    const refreshActiveConnections = useRefreshActiveConnections();
    const activeConnections = useGetActiveConnections();
    const navigate = useNavigate();
    const [isConnecting, setIsConnecting] = useState<{ status?: 'connecting'; index: number }>();

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
            <SidebarMenu>
                {connections?.map((connection, i) => (
                    <Collapsible key={connection.name} asChild defaultOpen={true} className="group/collapsible">
                        <SidebarMenuItem>
                            {activeConnections.includes(connection.name) ? (
                                <NavLink to={`/${connection.name}`}>
                                    {({ isActive }) => (
                                        <SidebarMenuButton tooltip={connection.name} isActive={isActive} className="cursor-pointer">
                                            <DatabaseZap />
                                            <span className="whitespace-nowrap truncate">{connection.name}</span>
                                            <GreenDot />
                                        </SidebarMenuButton>
                                    )}
                                </NavLink>
                            ) : (
                                <SidebarMenuButton tooltip={connection.name} asChild>
                                    <div className="group/item transition-all duration-200">
                                        {isConnecting?.index === i ? <RawLoader className="size-4" /> : <DatabaseZap />}
                                        <span className={cn('whitespace-nowrap truncate')}>{connection.name}</span>
                                        <Button
                                            size="sm"
                                            className={cn('h-5 ml-auto hidden group-hover/item:block', isConnecting?.index === i && 'block')}
                                            onClick={() => handleConnect(connection, i)}
                                            disabled={isConnecting?.status === 'connecting'}
                                        >
                                            {isConnecting?.index === i ? 'Connecting' : 'Connect'}
                                        </Button>
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

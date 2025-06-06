import { Link, useLocation } from 'react-router';
import { DatabaseZap } from 'lucide-react';

import { Collapsible } from '~/components/ui/collapsible';
import { SidebarGroup, SidebarGroupLabel, SidebarMenu, SidebarMenuButton, SidebarMenuItem } from '~/components/ui/sidebar';
import { useGetConnections } from '~/lib/stores/connections';

export function NavMain() {
    const connections = useGetConnections();
    const { pathname } = useLocation();
    return (
        <SidebarGroup>
            <SidebarGroupLabel>Connections</SidebarGroupLabel>
            <SidebarMenu>
                {connections.map(connection => {
                    console.log('PATHNAME', decodeURIComponent(pathname), connection.name);

                    return (
                        <Collapsible key={connection.name} asChild defaultOpen={true} className="group/collapsible">
                            <SidebarMenuItem>
                                <SidebarMenuButton tooltip={connection.name} isActive={decodeURIComponent(pathname).includes(connection.name)} asChild>
                                    <Link to={`/${connection.name}`}>
                                        <DatabaseZap />
                                        <span className="whitespace-nowrap">{connection.name}</span>
                                    </Link>
                                </SidebarMenuButton>
                            </SidebarMenuItem>
                        </Collapsible>
                    );
                })}
            </SidebarMenu>
        </SidebarGroup>
    );
}

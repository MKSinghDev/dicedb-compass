import * as React from 'react';
import { Bell, Database, DatabaseZap, type LucideIcon, PlusCircle, ShieldQuestion } from 'lucide-react';

import { NavMain } from '~/components/nav-main';
import { NavProjects } from '~/components/nav-projects';
import { NavUser } from '~/components/nav-user';
import { ScrollArea } from '~/components/ui/scroll-area';
import {
    Sidebar,
    SidebarContent,
    SidebarFooter,
    SidebarHeader,
    SidebarMenu,
    SidebarMenuButton,
    SidebarMenuItem,
    SidebarRail,
} from '~/components/ui/sidebar';

import Logo from './atoms/logo';
import CloudDatabase from './icons/cloud-database';
import ConnectionDialog from './molecules/connection-dialog';

// This is sample data.
const data = {
    user: {
        name: 'DiceDB User',
        email: 'm@example.com',
        avatar: '/icon.png',
    },
    connections: [
        {
            title: 'Product 1',
            url: '#',
            icon: DatabaseZap,
            isActive: true,
            items: [
                {
                    title: 'Production',
                    url: '#',
                },
                {
                    title: 'Staging',
                    url: '#',
                },
                {
                    title: 'QA',
                    url: '#',
                },
            ],
        },
        {
            title: 'Production',
            url: '#',
            icon: Database,
            items: [
                {
                    title: 'Product 2',
                    url: '#',
                },
                {
                    title: 'Product 3',
                    url: '#',
                },
                {
                    title: 'Product 4',
                    url: '#',
                },
            ],
        },
        {
            title: 'Staging',
            url: '#',
            icon: Database,
            items: [
                {
                    title: 'Product 2',
                    url: '#',
                },
                {
                    title: 'Product 3',
                    url: '#',
                },
                {
                    title: 'Product 4',
                    url: '#',
                },
            ],
        },
    ],
    settingsAndGuides: [
        {
            name: 'FREE DiceDB Cloud',
            url: '#',
            icon: CloudDatabase as unknown as LucideIcon,
        },
        {
            name: 'Notification Center',
            url: '#',
            icon: Bell,
        },
        {
            name: 'Help Center',
            url: '#',
            icon: ShieldQuestion,
        },
    ],
};

export function AppSidebar({ ...props }: React.ComponentProps<typeof Sidebar>) {
    return (
        <Sidebar collapsible="icon" {...props}>
            <SidebarHeader>
                <SidebarMenu>
                    <SidebarMenuItem>
                        <SidebarMenuButton asChild className="hover:bg-transparent p-0 group-data-[collapsible=icon]:p-0!">
                            <a href="https://dicedb.io" target="_blank" rel="noreferrer">
                                <Logo size="12" />
                            </a>
                        </SidebarMenuButton>
                    </SidebarMenuItem>
                </SidebarMenu>
            </SidebarHeader>
            <SidebarContent>
                <ScrollArea className="h-full">
                    <div className="p-2">
                        <ConnectionDialog>
                            <SidebarMenuButton variant="outline" tooltip="New Connection" className="w-full">
                                <PlusCircle />
                                <span className="whitespace-nowrap">New Connection</span>
                            </SidebarMenuButton>
                        </ConnectionDialog>
                    </div>
                    <NavMain items={data.connections} />
                </ScrollArea>
            </SidebarContent>
            <SidebarFooter>
                <NavProjects projects={data.settingsAndGuides} />
                <NavUser user={data.user} />
            </SidebarFooter>
            <SidebarRail />
        </Sidebar>
    );
}

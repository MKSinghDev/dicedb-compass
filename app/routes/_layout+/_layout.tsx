import { Outlet } from 'react-router';

import { Separator } from '@radix-ui/react-separator';
import { AppSidebar } from '~/components/app-sidebar';
import { Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList } from '~/components/ui/breadcrumb';
import { ScrollArea } from '~/components/ui/scroll-area';
import { SidebarInset, SidebarProvider, SidebarTrigger } from '~/components/ui/sidebar';

const Layout = () => (
    <SidebarProvider>
        <AppSidebar />
        <SidebarInset>
            <header className="flex h-16 shrink-0 items-center gap-2 transition-[width,height] ease-linear group-has-data-[collapsible=icon]/sidebar-wrapper:h-12">
                <div className="flex items-center gap-2 px-4">
                    <SidebarTrigger className="-ml-1" />
                    <Separator orientation="vertical" className="mr-2 data-[orientation=vertical]:h-4" />
                    <Breadcrumb>
                        <BreadcrumbList>
                            <BreadcrumbItem className="hidden md:block">
                                <BreadcrumbLink href="#">Home</BreadcrumbLink>
                            </BreadcrumbItem>
                            {/* <BreadcrumbSeparator className="hidden md:block" /> */}
                            {/* <BreadcrumbItem> */}
                            {/*     <BreadcrumbPage>Data Fetching</BreadcrumbPage> */}
                            {/* </BreadcrumbItem> */}
                        </BreadcrumbList>
                    </Breadcrumb>
                </div>
            </header>
            <div className="flex flex-1 flex-col gap-4 p-4 pt-0">
                <ScrollArea className="h-full">
                    <Outlet />
                </ScrollArea>
            </div>
        </SidebarInset>
    </SidebarProvider>
);

export default Layout;

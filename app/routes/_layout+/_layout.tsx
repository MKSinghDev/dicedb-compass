import { Outlet } from 'react-router';

import { Separator } from '@radix-ui/react-separator';
import { AppSidebar } from '~/components/app-sidebar';
import { SidebarInset, SidebarProvider, SidebarTrigger } from '~/components/ui/sidebar';

const Layout = () => (
    <SidebarProvider>
        <AppSidebar />
        <SidebarInset>
            <header className="flex h-16 shrink-0 items-center gap-2 transition-[width,height] ease-linear group-has-data-[collapsible=icon]/sidebar-wrapper:h-12">
                <div className="flex items-center gap-2 px-4">
                    <SidebarTrigger className="-ml-1" />
                    <Separator orientation="vertical" className="mr-2 data-[orientation=vertical]:h-4" />
                    {/* <Breadcrumb> */}
                    {/*     <BreadcrumbList> */}
                    {/*         <BreadcrumbItem> */}
                    {/*             <BreadcrumbLink href="#">Databases</BreadcrumbLink> */}
                    {/*         </BreadcrumbItem> */}
                    {/*         <BreadcrumbSeparator /> */}
                    {/*         <BreadcrumbItem>Connection</BreadcrumbItem> */}
                    {/*     </BreadcrumbList> */}
                    {/* </Breadcrumb> */}
                </div>
            </header>
            <div className="flex grow flex-col gap-4 p-4 pt-0">
                <Outlet />
            </div>
        </SidebarInset>
    </SidebarProvider>
);

export default Layout;

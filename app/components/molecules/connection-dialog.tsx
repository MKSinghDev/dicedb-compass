import { type ReactNode } from 'react';
import { DatabaseZap } from 'lucide-react';

import Loader from '~/components/atoms/loader';
import ConnectionForm from '~/components/molecules/connection-form';
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle, DialogTrigger } from '~/components/ui/dialog';

const ConnectionDialog = ({ children, defaultOpen }: { children: ReactNode; defaultOpen?: boolean }) => (
    <Dialog defaultOpen={defaultOpen}>
        <DialogTrigger asChild>{children}</DialogTrigger>
        <DialogContent className="p-0">
            <div className="relative p-6 space-y-6">
                <div className="flex items-center gap-2">
                    <div className="flex size-11 shrink-0 items-center justify-center rounded-full border" aria-hidden="true">
                        <DatabaseZap className="opacity-80" size={16} />
                    </div>
                    <DialogHeader>
                        <DialogTitle className="text-left">New Connection</DialogTitle>
                        <DialogDescription className="text-left">Manage your connection settings</DialogDescription>
                    </DialogHeader>
                </div>
                <ConnectionForm />
                <Loader />
            </div>
        </DialogContent>
    </Dialog>
);

export default ConnectionDialog;

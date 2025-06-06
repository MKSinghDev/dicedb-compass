import { type ReactNode } from 'react';
import { useNavigate, useNavigation } from 'react-router';
import { DatabaseZap } from 'lucide-react';

import ConnectionForm from '~/components/molecules/connection-form';
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle, DialogTrigger } from '~/components/ui/dialog';

const ConnectionDialog = ({ children, defaultOpen }: { children?: ReactNode; defaultOpen?: boolean }) => {
    const navigate = useNavigate();
    const { state } = useNavigation();

    const handleOnOpenChange = () => {
        if (state !== 'submitting') {
            navigate(-1);
        }
    };
    return (
        <Dialog defaultOpen={defaultOpen} onOpenChange={handleOnOpenChange} open={state === 'submitting' ? true : undefined}>
            {children ? <DialogTrigger asChild>{children}</DialogTrigger> : null}
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
                </div>
            </DialogContent>
        </Dialog>
    );
};

export default ConnectionDialog;

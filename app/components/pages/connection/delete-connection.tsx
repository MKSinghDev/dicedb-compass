import type { ReactNode } from 'react';

import {
    AlertDialog,
    AlertDialogAction,
    AlertDialogCancel,
    AlertDialogContent,
    AlertDialogDescription,
    AlertDialogFooter,
    AlertDialogHeader,
    AlertDialogTitle,
    AlertDialogTrigger,
} from '~/components/ui/alert-dialog';
import { Button } from '~/components/ui/button';
import { deleteKey } from '~/lib/commands/query';

const DeleteConnection = ({ connection, keyName, children }: { connection: string; keyName: string; children: ReactNode }) => {
    const handleDelete = async () => {
        try {
            const res = await deleteKey(connection, keyName);
            console.log('Deletion res', res);
        } catch (error) {
            console.log(error);
        }
    };
    return (
        <AlertDialog>
            <AlertDialogTrigger asChild>{children}</AlertDialogTrigger>
            <AlertDialogContent>
                <AlertDialogHeader>
                    <AlertDialogTitle>Are you sure?</AlertDialogTitle>
                    <AlertDialogDescription>Take a moment to rethink, this action cannot be undone!</AlertDialogDescription>
                </AlertDialogHeader>
                <AlertDialogFooter>
                    <AlertDialogCancel>Cancel</AlertDialogCancel>
                    <AlertDialogAction asChild>
                        <Button variant="destructive" className="text-destructive-foreground" name="key" value={keyName} onClick={handleDelete}>
                            Okay
                        </Button>
                    </AlertDialogAction>
                </AlertDialogFooter>
            </AlertDialogContent>
        </AlertDialog>
    );
};

export default DeleteConnection;

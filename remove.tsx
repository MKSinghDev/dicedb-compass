import { data, Form, useNavigate } from 'react-router';
import type { Route } from '+/_layout+/connection+/+types/remove';

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
import { removeConnection } from '~/lib/commands/connection';

export const clientAction = async ({ request }: Route.ClientActionArgs) => {
    const { searchParams } = new URL(request.url);
    const formData = await request.formData();

    console.log(searchParams.get('connecction'));
    if (formData.get('confirmation') === 'ok') return data(await removeConnection(searchParams.get('connection') ?? ''));
    return data(null);
};

const RemoveConnectionPage = () => {
    const navigate = useNavigate();
    return (
        <AlertDialog defaultOpen onOpenChange={() => navigate(-1)}>
            <AlertDialogContent>
                <AlertDialogHeader>
                    <AlertDialogTitle>Are you sure?</AlertDialogTitle>
                    <AlertDialogDescription>Take a moment to rethink, this action cannot be undone!</AlertDialogDescription>
                </AlertDialogHeader>
                <AlertDialogFooter>
                    <AlertDialogCancel>Cancel</AlertDialogCancel>
                    <Form method="DELETE">
                        <AlertDialogAction asChild>
                            <Button variant="destructive" className="text-destructive-foreground" type="submit" name="confirmation" value="ok">
                                Okay
                            </Button>
                        </AlertDialogAction>
                    </Form>
                </AlertDialogFooter>
            </AlertDialogContent>
        </AlertDialog>
    );
};

export default RemoveConnectionPage;

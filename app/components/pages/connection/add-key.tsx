import { useState } from 'react';
import { Form, useNavigate } from 'react-router';
import { CircleCheckBig } from 'lucide-react';

import { useForm } from '@conform-to/react';
import { parseWithZod } from '@conform-to/zod/v4';
import InputWithLabel from '~/components/atoms/form-elements/input-with-label';
import SubmitButton from '~/components/atoms/form-elements/submit-button';
import { Button } from '~/components/ui/button';
import { Dialog, DialogClose, DialogContent, DialogDescription, DialogHeader, DialogTitle } from '~/components/ui/dialog';

import { schema } from './add-key-schema';

type State = 'ADDING' | 'ADDED' | 'FAILED' | null;

const AddKey = () => {
    const [state, setState] = useState<State>(null);
    const navigate = useNavigate();
    const [form, fields] = useForm({
        shouldValidate: 'onSubmit',
        shouldRevalidate: 'onInput',
        onValidate({ formData }) {
            return parseWithZod(formData, { schema });
        },
    });

    return (
        <Dialog defaultOpen onOpenChange={() => navigate(-1)}>
            <DialogContent>
                <DialogHeader>
                    <DialogTitle>Add new data</DialogTitle>
                    <DialogDescription>Will replace if key already exists</DialogDescription>
                </DialogHeader>
                <Form method="POST" id={form.id} onSubmit={form.onSubmit} className="flex flex-col gap-4 space-y-5">
                    <div className="flex gap-4">
                        <InputWithLabel field={fields.name} label="Key" />
                        <InputWithLabel field={fields.value} label="value" />
                    </div>
                    {state === 'ADDED' ? (
                        <DialogClose asChild>
                            <Button className="w-fit ml-auto" onClick={() => setState(null)}>
                                <CircleCheckBig className="text-emerald-500 dark:accent-emerald-400" /> Added
                            </Button>
                        </DialogClose>
                    ) : (
                        <SubmitButton className="w-fit ml-auto">Add</SubmitButton>
                    )}
                </Form>
            </DialogContent>
        </Dialog>
    );
};

export default AddKey;
